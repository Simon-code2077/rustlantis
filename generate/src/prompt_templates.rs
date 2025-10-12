use serde::{Deserialize, Serialize};

// 更详细的prompt模板系统
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub response_format: String,
    pub model_name: String,
}

impl PromptTemplate {
    pub fn default_optimization_template() -> Self {
        Self {
            model_name: "llama2".to_string(),
            system_prompt: r#"
You are an expert in Rust compiler fuzzing and MIR (Mid-level Intermediate Representation) generation optimization.
Your goal is to optimize weight selection for place choices to maximize bug discovery potential in the Rust compiler.

Key principles:
1. Prioritize choices that are likely to expose compiler edge cases
2. Balance between valid and boundary-case constructs  
3. Consider type interactions and aliasing scenarios
4. Adapt to current generation context (function depth, complexity, etc.)
"#.to_string(),

            user_prompt_template: r#"
## Current Generation Context:
- **Usage Type**: {usage_type}
- **Basic Block Count**: {bb_count}
- **Statement Count**: {stmt_count}  
- **Function Depth**: {function_depth}
- **Total Variables**: {total_variables}

## Place Candidates for Weight Optimization:

{place_candidates}

## Current Weight Distribution:

{current_weights}

## Optimization Guidelines by Usage Type:

### For "{usage_type}" usage:
{usage_specific_guidelines}

## Historical Bug Discovery Patterns:
- Places with moderate complexity (5-15) often reveal interesting compiler behaviors
- Uninitialized places in LHS context create more state exploration opportunities
- Pointer/reference interactions frequently expose aliasing bugs
- Type transmutations and casts are high-value targets

Please analyze the current candidates and provide optimized weights that will:
1. Increase probability of discovering compiler bugs
2. Maintain program validity while exploring edge cases
3. Adapt to the current generation context
4. Balance exploration vs exploitation

Return your response as valid JSON in the specified format.
"#.to_string(),

            response_format: r#"
{
  "optimized_weights": [
    {"place_id": 0, "weight": 15, "adjustment_factor": 1.5},
    {"place_id": 1, "weight": 8, "adjustment_factor": 0.8}
  ],
  "reasoning": "Detailed explanation of optimization decisions"
}
"#.to_string(),
        }
    }

    pub fn usage_specific_guidelines(&self, usage_type: &str) -> String {
        match usage_type {
            "Operand" => r#"
- Favor places with complexity 5-15 (sweet spot for bug discovery)
- Increase weight for places with known values (enables more directed testing)
- Slightly prefer copy types to reduce move complexity
- Boost deref operations (common source of borrowing issues)
"#.to_string(),
            
            "LHS" => r#"
- Heavily favor uninitialized places (2-3x weight boost)
- Prioritize return projections for state exploration
- Avoid offsetted raw pointers (weight = 0)
- Balance between simple and complex assignment targets
"#.to_string(),
            
            "Argument" => r#"
- Balance literal arguments (2x weight) with complex types
- Heavily weight reference arguments (20x) for aliasing exploration
- Boost raw pointer arguments (20x) for unsafe code paths
- Prefer isize types for pointer arithmetic scenarios
"#.to_string(),
            
            "RET" => r#"
- Completely avoid reference types in return positions
- Favor diverse type combinations
- Prefer return projections (2x weight)
- Balance initialized vs uninitialized returns
"#.to_string(),
            
            _ => "Apply general optimization principles focusing on edge case discovery.".to_string(),
        }
    }

    pub fn format_place_candidates(&self, candidates: &[crate::llm_optimizer::PlaceFeature]) -> String {
        candidates.iter().enumerate().map(|(i, feature)| {
            format!(
                "**Place {}**:
  - Type: {}
  - Characteristics: {}{}{}{}{}
  - Complexity: {}
  - Current Weight: {}",
                i,
                feature.type_info,
                if feature.is_ref { "ref " } else { "" },
                if feature.is_raw_ptr { "raw_ptr " } else { "" },
                if feature.has_known_val { "known_val " } else { "" },
                if feature.is_uninit { "uninit " } else { "init " },
                if feature.has_deref { "deref " } else { "" },
                feature.complexity,
                feature.current_weight
            )
        }).collect::<Vec<_>>().join("\n\n")
    }

    pub fn format_weights_analysis(&self, candidates: &[crate::llm_optimizer::PlaceFeature]) -> String {
        let total_weight: usize = candidates.iter().map(|c| c.current_weight).sum();
        let avg_weight = if candidates.is_empty() { 0.0 } else { total_weight as f64 / candidates.len() as f64 };
        
        let mut analysis = format!(
            "- Total candidates: {}\n- Total weight: {}\n- Average weight: {:.1}\n\n",
            candidates.len(), total_weight, avg_weight
        );

        // Weight distribution analysis
        let high_weight_count = candidates.iter().filter(|c| c.current_weight as f64 > avg_weight * 1.5).count();
        let low_weight_count = candidates.iter().filter(|c| (c.current_weight as f64) < avg_weight * 0.5).count();
        
        analysis.push_str(&format!(
            "- High weight places (>1.5x avg): {}\n- Low weight places (<0.5x avg): {}\n",
            high_weight_count, low_weight_count
        ));

        // Type-based analysis
        let ref_count = candidates.iter().filter(|c| c.is_ref).count();
        let ptr_count = candidates.iter().filter(|c| c.is_raw_ptr).count();
        let uninit_count = candidates.iter().filter(|c| c.is_uninit).count();
        
        analysis.push_str(&format!(
            "- Reference types: {}\n- Raw pointer types: {}\n- Uninitialized places: {}",
            ref_count, ptr_count, uninit_count
        ));

        analysis
    }
}