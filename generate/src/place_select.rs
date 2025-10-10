use std::{rc::Rc, vec, cell::RefCell};

use abi::size::Size;
use mir::{
    syntax::{Literal, Place, TyId},
    tyctxt::TyCtxt,
};
use rand_distr::WeightedIndex;

use crate::{
    llm_optimizer::{LLMOptimizer, LLMConfig, WeightOptimizationRequest, PlaceFeature, ContextInfo},
    mem::BasicMemory,
    pgraph::{PlaceGraph, PlaceIndex, PlacePath, ToPlaceIndex},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PlaceUsage {
    Operand,
    LHS,
    RET,
    SetDiscriminant,
    Pointee,
    Argument,
    KnownVal,
    NonZero,
    Offsetee,
}

#[derive(Clone)]
pub struct PlaceSelector {
    tys: Option<Vec<TyId>>,
    exclusions: Vec<Place>,
    moved: Vec<PlaceIndex>,
    refed: Vec<PlaceIndex>,
    size: Option<Size>,
    allow_uninit: bool,
    usage: PlaceUsage,
    tcx: Rc<TyCtxt>,
    llm_optimizer: Option<Rc<RefCell<LLMOptimizer>>>,
    llm_config: LLMConfig,
    optimization_counter: usize,
}

pub type Weight = usize;

const RET_LHS_WEIGHT_FACTOR: Weight = 2;
const UNINIT_WEIGHT_FACTOR: Weight = 2;
const DEREF_WEIGHT_FACTOR: Weight = 20;
const LIT_ARG_WEIGHT_FACTOR: Weight = 2;
const PTR_ARG_WEIGHT_FACTOR: Weight = 20;
const REF_ARG_WEIGHT_FACTOR: Weight = 20;
const OFFSETTED_PTR_WEIGHT_FACTOR: Weight = 10;
const ROUNDTRIPPED_PTR_WEIGHT_FACTOR: Weight = 100;

impl PlaceSelector {
    pub fn for_pointee(tcx: Rc<TyCtxt>, allow_uninit: bool) -> Self {
        Self {
            usage: PlaceUsage::Pointee,
            allow_uninit,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_operand(tcx: Rc<TyCtxt>) -> Self {
        Self {
            tys: None,
            size: None,
            usage: PlaceUsage::Operand,
            exclusions: vec![],
            allow_uninit: false,
            tcx,
            moved: vec![],
            refed: vec![],
            llm_optimizer: None,
            llm_config: LLMConfig::default(),
            optimization_counter: 0,
        }
    }

    pub fn with_llm_config(mut self, config: LLMConfig) -> Self {
        println!("DEBUG: PlaceSelector.with_llm_config called, enabled={}", config.enabled);
        if config.enabled {
            println!("DEBUG: 创建LLM优化器，端点: {}", config.api_endpoint);
            self.llm_optimizer = Some(Rc::new(RefCell::new(LLMOptimizer::new(
                config.api_endpoint.clone(),
                config.api_key.clone(),
            ))));
            println!("DEBUG: LLM优化器创建成功");
        }
        self.llm_config = config;
        self
    }

    pub fn enable_llm_optimization(&mut self, api_endpoint: String, api_key: Option<String>) {
        self.llm_optimizer = Some(Rc::new(RefCell::new(LLMOptimizer::new(api_endpoint, api_key))));
        self.llm_config.enabled = true;
    }

    pub fn for_set_discriminant(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::SetDiscriminant,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_return_place(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::RET,
            ..Self::for_lhs(tcx)
        }
    }

    pub fn for_argument(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::Argument,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_lhs(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::LHS,
            allow_uninit: true,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_known_val(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::KnownVal,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_non_zero(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::NonZero,
            ..Self::for_operand(tcx)
        }
    }

    pub fn for_offsetee(tcx: Rc<TyCtxt>) -> Self {
        Self {
            usage: PlaceUsage::Offsetee,
            ..Self::for_operand(tcx)
        }
    }

    pub fn of_ty(self, ty: TyId) -> Self {
        let tys = Some(vec![ty]);
        Self { tys, ..self }
    }

    pub fn of_tys(self, types: &[TyId]) -> Self {
        let tys = Some(Vec::from(types));
        Self { tys, ..self }
    }

    pub fn of_size(self, size: Size) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn except(self, exclude: &Place) -> Self {
        let mut exclusions = self.exclusions;
        // TODO: More granular place discrimination
        exclusions.push(exclude.clone());
        Self { exclusions, ..self }
    }

    pub fn having_moved(self, place: PlaceIndex) -> Self {
        assert_eq!(self.usage, PlaceUsage::Argument);
        let mut moved = self.moved;
        moved.push(place.clone());
        Self { moved, ..self }
    }

    pub fn having_refed(self, target: PlaceIndex) -> Self {
        assert_eq!(self.usage, PlaceUsage::Argument);
        let mut refed = self.refed;
        refed.push(target.clone());
        Self { refed, ..self }
    }

    fn into_iter_path(self, pt: &PlaceGraph) -> impl Iterator<Item = PlacePath> + Clone + '_ {
        let exclusion_indicies: Vec<PlaceIndex> = self
            .exclusions
            .iter()
            .map(|place| place.to_place_index(pt).expect("excluded place exists"))
            .chain(pt.return_dest_stack()) // Don't touch anything that overlaps with any RET in the stack
            .chain(pt.moved_in_args_stack()) // Don't touch anything that overlaps with moved in args in the stack
            .collect();
        let moved: Vec<PlaceIndex> = self
            .moved
            .iter()
            .map(|place| place.to_place_index(pt).expect("place exists"))
            .collect();
        let refed: Vec<PlaceIndex> = self
            .refed
            .iter()
            .map(|place| place.to_place_index(pt).expect("place exists"))
            .collect();
        pt.reachable_nodes().filter(move |ppath| {
            let index = ppath.target_index();

            // Well-typedness
            if let Some(tys) = &self.tys
                && !tys.contains(&pt.ty(index))
            {
                return false;
            }

            // Liveness
            if !pt.is_place_live(index) {
                return false;
            }

            // Initness
            if !self.allow_uninit && !pt.is_place_init(index) {
                return false;
            };

            // Ref validity
            if self.usage != PlaceUsage::LHS && !pt.contains_only_valid_ref(index) {
                return false;
            }

            // Known val
            if self.usage == PlaceUsage::KnownVal && pt.known_val(index).is_none() {
                return false;
            }

            if self.usage == PlaceUsage::NonZero {
                let Some(known_val) = pt.known_val(index) else {
                    return false;
                };
                match known_val {
                    Literal::Uint(v, _) if *v != 0 => {}
                    Literal::Int(v, _) if *v != 0 => {}
                    Literal::Float(v, _) if *v != 0. => {}
                    _ => return false,
                }
            }

            // Avoid having ref in return type
            if self.usage == PlaceUsage::RET
                && pt.ty(index).contains(&self.tcx, |tcx, ty| ty.is_ref(tcx))
            {
                return false;
            }

            // Not excluded
            if exclusion_indicies
                .iter()
                .any(|excl| pt.overlap(index, excl))
            {
                return false;
            }

            // Has the right size
            if self.size.is_some() && BasicMemory::ty_size(pt.ty(index), &self.tcx) != self.size {
                return false;
            }

            // Check for aliasing rules
            match self.usage {
                // writes
                PlaceUsage::LHS | PlaceUsage::SetDiscriminant | PlaceUsage::RET => {
                    if !pt.can_write_through(ppath.source(), index) {
                        return false;
                    }
                }
                // reads that will be done as moves
                PlaceUsage::Operand | PlaceUsage::Argument if !pt.ty(index).is_copy(&self.tcx) => {
                    if !pt.can_write_through(ppath.source(), index) {
                        return false;
                    }
                }
                // reads
                _ => {
                    if !pt.can_read_through(ppath.source(), index) {
                        return false;
                    }
                }
            }

            // Function arguments
            if self.usage == PlaceUsage::Argument {
                if moved.iter().any(|excl| pt.overlap(index, excl)) {
                    return false;
                }
                // If this contains a ref, then it cannot point to an already-picked moved arg
                for m in &moved {
                    if pt.contains_ref_to(index, *m) {
                        return false;
                    }
                }

                if !pt.ty(index).is_copy(&self.tcx) {
                    // If this is a type that must be moved, then it must not be referenced by an already-picked reference
                    for r in &refed {
                        if pt.contains_ref_to(*r, index) {
                            return false;
                        }
                    }
                }
            }

            true
        })
    }

    pub fn into_weighted(self, pt: &PlaceGraph) -> Option<(Vec<PlacePath>, WeightedIndex<Weight>)> {
        let usage = self.usage;
        let tcx = self.tcx.clone();
        let should_optimize = self.should_optimize_with_llm();
        let llm_optimizer = self.llm_optimizer.clone();
        let _llm_config = self.llm_config.clone();
        
        let (places, mut weights): (Vec<PlacePath>, Vec<Weight>) =
            self.into_iter_path(pt)
                .map(|ppath| {
                    let place = ppath.target_index();
                    let mut weight = match usage {
                        PlaceUsage::Argument => {
                            let mut weight = 1;
                            let index = ppath.target_index();
                            let ty = pt.ty(index);
                            if ty.contains(&tcx, |tcx, ty| ty.is_ref(tcx)) {
                                weight *= REF_ARG_WEIGHT_FACTOR;
                            }
                            if ty.contains(&tcx, |tcx, ty| ty.is_raw_ptr(tcx)) {
                                weight *= PTR_ARG_WEIGHT_FACTOR;
                            }
                            if pt.known_val(index).is_some() {
                                weight *= LIT_ARG_WEIGHT_FACTOR;
                            }
                            // Encourage isize for pointer offset
                            if ty.contains(&tcx, |_, ty| ty == TyCtxt::ISIZE) {
                                weight *= LIT_ARG_WEIGHT_FACTOR;
                            }
                            if ty.is_raw_ptr(&tcx) && pt.offseted(index) {
                                weight *= OFFSETTED_PTR_WEIGHT_FACTOR;
                            }
                            weight
                        }
                        PlaceUsage::LHS | PlaceUsage::SetDiscriminant | PlaceUsage::RET => {
                            let mut weight = if !pt.is_place_init(place) {
                                if ppath.is_return_proj(pt) {
                                    RET_LHS_WEIGHT_FACTOR
                                } else {
                                    UNINIT_WEIGHT_FACTOR
                                }
                            } else {
                                1
                            };
                            if pt.ty(place).is_raw_ptr(&tcx) && pt.get_offset(place).is_some() {
                                weight = 0;
                            }
                            weight
                        }
                        PlaceUsage::Operand => pt.get_complexity(place),
                        PlaceUsage::Pointee => 1,
                        PlaceUsage::KnownVal | PlaceUsage::NonZero => pt.get_complexity(place),
                        PlaceUsage::Offsetee => 1,
                    };

                    if ppath.projections(pt).any(|proj| proj.is_deref()) {
                        weight *= DEREF_WEIGHT_FACTOR;
                    }

                    if ppath.nodes(pt).any(|place| {
                        pt.ty(place).is_raw_ptr(&tcx) && pt.has_offset_roundtripped(place)
                    }) {
                        weight *= ROUNDTRIPPED_PTR_WEIGHT_FACTOR;
                    }

                    (ppath, weight)
                })
                .unzip();

        // Apply LLM optimization if enabled and appropriate
        if should_optimize {
            println!("DEBUG: 准备调用LLM优化，places数量: {}, usage: {:?}", places.len(), usage);
            if let Some(optimized_weights) = Self::optimize_weights_with_llm_static(
                &places, &weights, pt, llm_optimizer, &usage
            ) {
                println!("DEBUG: LLM优化成功，应用新权重");
                weights = optimized_weights;
            } else {
                println!("DEBUG: LLM优化失败，使用原始权重");
            }
        } else {
            println!("DEBUG: 跳过LLM优化");
        }

        if let Ok(weighted_index) = WeightedIndex::new(weights) {
            Some((places, weighted_index))
        } else {
            None
        }
    }

    fn should_optimize_with_llm(&self) -> bool {
        let should_optimize = self.llm_config.enabled && self.llm_optimizer.is_some();
        println!("DEBUG: should_optimize_with_llm = {}, enabled={}, optimizer_exists={}", 
                should_optimize, self.llm_config.enabled, self.llm_optimizer.is_some());
        should_optimize
    }

    fn optimize_weights_with_llm_static(
        places: &[PlacePath],
        current_weights: &[Weight],
        pt: &PlaceGraph,
        llm_optimizer: Option<Rc<RefCell<LLMOptimizer>>>,
        usage: &PlaceUsage,
    ) -> Option<Vec<Weight>> {
        println!("DEBUG: 进入optimize_weights_with_llm_static，places数量: {}", places.len());
        
        // 如果没有places，直接返回
        if places.is_empty() {
            println!("DEBUG: places为空，直接返回");
            return None;
        }
        
        let optimizer = llm_optimizer?;
        println!("DEBUG: LLM优化器存在");
        
        // 限制优化的place数量，避免API调用过大
        if places.len() > 50 {
            println!("DEBUG: places数量超过50，跳过优化");
            return None;
        }
        
        println!("DEBUG: 开始构建PlaceFeature");
        let place_features: Vec<PlaceFeature> = places
            .iter()
            .enumerate()
            .map(|(i, ppath)| {
                let index = ppath.target_index();
                let ty = pt.ty(index);
                
                // Create a simplified type info string
                let type_info = format!("{:?}", ty);
                
                // Use available public methods to check type properties
                let is_ref = type_info.contains("&");
                let is_raw_ptr = type_info.contains("*");
                
                PlaceFeature {
                    place_id: i,
                    type_info,
                    is_ref,
                    is_raw_ptr,
                    has_known_val: pt.known_val(index).is_some(),
                    is_uninit: !pt.is_place_init(index),
                    complexity: pt.get_complexity(index),
                    has_deref: ppath.projections(pt).any(|proj| proj.is_deref()),
                    is_offsetted: false, // 暂时禁用以避免panic
                    is_roundtripped: false, // 暂时禁用以避免panic
                    current_weight: current_weights[i],
                }
            })
            .collect();

        println!("DEBUG: 构建PlaceFeature完成，数量: {}", place_features.len());

        let context_info = ContextInfo {
            current_bb_count: 0, // You'll need to pass this from GenerationCtx
            current_stmt_count: 0, // You'll need to pass this from GenerationCtx
            function_depth: 0, // You'll need to pass this from GenerationCtx
            total_variables: places.len(),
        };

        let request = WeightOptimizationRequest {
            usage_type: format!("{:?}", usage),
            place_features,
            context_info,
        };

        println!("DEBUG: 准备调用LLM API，usage_type: {}", request.usage_type);

        match optimizer.borrow_mut().optimize_weights_sync(request) {
            Ok(response) => {
                println!("DEBUG: LLM API调用成功，reasoning: {}", response.reasoning);
                if response.reasoning.contains("Failed to parse LLM response") {
                    println!("DEBUG: LLM响应解析失败");
                    None
                } else {
                    let mut optimized_weights = current_weights.to_vec();
                    for opt_weight in response.optimized_weights {
                        if opt_weight.place_id < optimized_weights.len() {
                            optimized_weights[opt_weight.place_id] = opt_weight.weight;
                        }
                    }
                    println!("DEBUG: 权重优化完成，返回新权重");
                    Some(optimized_weights)
                }
            }
            Err(e) => {
                println!("DEBUG: LLM API调用失败: {:?}", e);
                None
            }
        }
    }

    pub fn into_iter_place(self, pt: &PlaceGraph) -> impl Iterator<Item = Place> + Clone + '_ {
        self.into_iter_path(pt).map(|ppath| ppath.to_place(pt))
    }
}

#[cfg(test)]
mod llm_tests;

#[cfg(test)]
mod tests {
    extern crate test;
    use std::rc::Rc;

    use mir::{
        syntax::{Local, Place},
        tyctxt::TyCtxt,
    };
    use rand::{
        rngs::SmallRng,
        seq::{IteratorRandom, SliceRandom},
        Rng, SeedableRng,
    };
    use test::Bencher;

    use crate::{
        pgraph::PlaceGraph,
        ty::{seed_tys, TySelect},
    };

    use super::PlaceSelector;

    fn build_pt(rng: &mut impl Rng) -> (PlaceGraph, Rc<TyCtxt>) {
        let tcx = Rc::new(seed_tys(rng));
        let mut pt = PlaceGraph::new(tcx.clone());
        let ty_weights = TySelect::new(&tcx);
        for i in 0..=32 {
            let pidx = pt.allocate_local(Local::new(i), ty_weights.choose_ty(rng, &tcx));
            if i % 2 == 0 {
                pt.mark_place_init(pidx);
            }
        }
        (pt, tcx)
    }

    #[bench]
    fn bench_select(b: &mut Bencher) {
        let mut rng = SmallRng::seed_from_u64(0);
        let (pt, tcx) = build_pt(&mut rng);

        b.iter(|| {
            PlaceSelector::for_lhs(tcx.clone())
                .except(&Place::RETURN_SLOT)
                .into_iter_place(&pt)
                .choose(&mut rng)
                .expect("places not empty");
        })
    }

    #[bench]
    fn bench_materialise_into_vec(b: &mut Bencher) {
        let mut rng = SmallRng::seed_from_u64(0);
        let (pt, tcx) = build_pt(&mut rng);

        b.iter(|| {
            let places: Vec<Place> = PlaceSelector::for_lhs(tcx.clone())
                .except(&Place::RETURN_SLOT)
                .into_iter_place(&pt)
                .collect();

            // places.choose(&mut rng).expect("not empty");
            places
                .choose_weighted(&mut rng, |p| p.projection().len())
                .expect("places not empty");
        })
    }
}
