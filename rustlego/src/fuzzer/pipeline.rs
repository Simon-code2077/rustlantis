use crate::composer::combiner::{ComposedProgram, FunctionCombiner};
use crate::difftest::runner::{DiffTestRunner, DiffTestConfig, DiffTestReport};
use crate::llm::function_generator::{FunctionGenerator, GeneratedFunction};
use crate::templates::TemplateLibrary;
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub struct FuzzingPipeline {
    function_generator: FunctionGenerator,
    function_combiner: FunctionCombiner,
    difftest_runner: DiffTestRunner,
    template_library: TemplateLibrary,
}

#[derive(Debug, Clone)]
pub struct FuzzingConfig {
    pub functions_per_batch: usize,
    pub programs_per_batch: usize,
    pub categories: Vec<String>,
    pub max_function_complexity: u32,
    pub output_dir: PathBuf,
    pub difftest_config: DiffTestConfig,
}

#[derive(Debug, Clone)]
pub struct FuzzingSession {
    pub config: FuzzingConfig,
    pub generated_functions: Vec<GeneratedFunction>,
    pub composed_programs: Vec<ComposedProgram>,
    pub test_reports: Vec<DiffTestReport>,
    pub bugs_found: usize,
}

impl FuzzingPipeline {
    pub fn new(difftest_config: DiffTestConfig) -> Result<Self> {
        Ok(Self {
            function_generator: FunctionGenerator::new()?,
            function_combiner: FunctionCombiner::new(),
            difftest_runner: DiffTestRunner::new(difftest_config),
            template_library: TemplateLibrary::new(),
        })
    }

    pub fn default_config() -> FuzzingConfig {
        FuzzingConfig {
            functions_per_batch: 10,
            programs_per_batch: 5,
            categories: vec![
                "arithmetic".to_string(),
                "memory".to_string(),
                "control_flow".to_string(),
                "type_conversion".to_string(),
                "string_ops".to_string(),
            ],
            max_function_complexity: 5,
            output_dir: PathBuf::from("fuzzing_output"),
            difftest_config: DiffTestRunner::default_config(),
        }
    }

    pub async fn run_fuzzing_session(&self, config: FuzzingConfig) -> Result<FuzzingSession> {
        println!("Starting fuzzing session with config:");
        println!("  Functions per batch: {}", config.functions_per_batch);
        println!("  Programs per batch: {}", config.programs_per_batch);
        println!("  Categories: {:?}", config.categories);
        
        // Create output directory
        fs::create_dir_all(&config.output_dir).await?;
        
        let mut session = FuzzingSession {
            config: config.clone(),
            generated_functions: Vec::new(),
            composed_programs: Vec::new(),
            test_reports: Vec::new(),
            bugs_found: 0,
        };

        // Phase 1: Generate functions
        println!("\n=== Phase 1: Function Generation ===");
        for category in &config.categories {
            println!("Generating functions for category: {}", category);
            
            let functions = self.function_generator
                .generate_batch(category, config.functions_per_batch)
                .await?;
            
            println!("  Generated {} functions", functions.len());
            session.generated_functions.extend(functions);
        }

        // Save generated functions
        let functions_dir = config.output_dir.join("generated_functions");
        self.function_generator
            .save_functions_to_directory(&session.generated_functions, &functions_dir)
            .await?;

        // Phase 2: Compose programs
        println!("\n=== Phase 2: Program Composition ===");
        for i in 0..config.programs_per_batch {
            // Select random subset of functions for combination
            let functions_for_program = self.select_functions_for_composition(
                &session.generated_functions,
                3, // Functions per program
                config.max_function_complexity,
            );

            if functions_for_program.len() >= 2 {
                println!("Composing program {} with {} functions", i + 1, functions_for_program.len());
                
                // Try both combination strategies
                if let Ok(composed) = self.function_combiner.combine_functions(&functions_for_program) {
                    session.composed_programs.push(composed);
                }
                
                if let Ok(chained) = self.function_combiner.create_chained_composition(&functions_for_program) {
                    session.composed_programs.push(chained);
                }
            }
        }

        println!("  Composed {} programs", session.composed_programs.len());

        // Save composed programs
        let programs_dir = config.output_dir.join("composed_programs");
        fs::create_dir_all(&programs_dir).await?;
        
        for (i, program) in session.composed_programs.iter().enumerate() {
            let program_file = programs_dir.join(format!("{:03}_{}.rs", i + 1, program.name));
            fs::write(&program_file, &program.code).await?;
        }

        // Phase 3: Differential testing
        println!("\n=== Phase 3: Differential Testing ===");
        let test_reports = self.difftest_runner
            .run_batch_tests(&session.composed_programs)
            .await?;

        // Count bugs found
        let bugs_found = test_reports
            .iter()
            .map(|report| report.discrepancies.len())
            .sum();

        session.test_reports = test_reports;
        session.bugs_found = bugs_found;

        println!("\n=== Fuzzing Session Complete ===");
        println!("  Functions generated: {}", session.generated_functions.len());
        println!("  Programs composed: {}", session.composed_programs.len());
        println!("  Bugs found: {}", session.bugs_found);

        // Save session summary
        let summary_file = config.output_dir.join("session_summary.json");
        let summary_json = serde_json::to_string_pretty(&session)?;
        fs::write(summary_file, summary_json).await?;

        Ok(session)
    }

    pub async fn run_continuous_fuzzing(
        &self,
        config: FuzzingConfig,
        iterations: usize,
    ) -> Result<Vec<FuzzingSession>> {
        let mut sessions = Vec::new();
        
        for i in 0..iterations {
            println!("\n🔬 Starting fuzzing iteration {} of {}", i + 1, iterations);
            
            let mut iteration_config = config.clone();
            iteration_config.output_dir = config.output_dir.join(format!("iteration_{:03}", i + 1));
            
            match self.run_fuzzing_session(iteration_config).await {
                Ok(session) => {
                    println!("✅ Iteration {} completed - {} bugs found", i + 1, session.bugs_found);
                    sessions.push(session);
                }
                Err(e) => {
                    eprintln!("❌ Iteration {} failed: {}", i + 1, e);
                }
            }
        }
        
        // Generate overall statistics
        let total_bugs = sessions.iter().map(|s| s.bugs_found).sum::<usize>();
        let total_programs = sessions.iter().map(|s| s.composed_programs.len()).sum::<usize>();
        let total_functions = sessions.iter().map(|s| s.generated_functions.len()).sum::<usize>();
        
        println!("\n📊 Continuous Fuzzing Summary:");
        println!("  Iterations completed: {}", sessions.len());
        println!("  Total functions generated: {}", total_functions);
        println!("  Total programs composed: {}", total_programs);
        println!("  Total bugs found: {}", total_bugs);
        
        if !sessions.is_empty() {
            let avg_bugs_per_iteration = total_bugs as f64 / sessions.len() as f64;
            println!("  Average bugs per iteration: {:.2}", avg_bugs_per_iteration);
        }
        
        Ok(sessions)
    }

    fn select_functions_for_composition(
        &self,
        all_functions: &[GeneratedFunction],
        target_count: usize,
        max_complexity: u32,
    ) -> Vec<GeneratedFunction> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        // Filter functions by complexity
        let eligible_functions: Vec<_> = all_functions
            .iter()
            .filter(|f| f.complexity <= max_complexity)
            .cloned()
            .collect();
        
        if eligible_functions.len() <= target_count {
            return eligible_functions;
        }
        
        // Randomly select functions
        let mut rng = thread_rng();
        let mut selected = eligible_functions;
        selected.shuffle(&mut rng);
        selected.truncate(target_count);
        
        selected
    }
}

impl serde::Serialize for FuzzingSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut state = serializer.serialize_struct("FuzzingSession", 4)?;
        state.serialize_field("functions_generated", &self.generated_functions.len())?;
        state.serialize_field("programs_composed", &self.composed_programs.len())?;
        state.serialize_field("bugs_found", &self.bugs_found)?;
        state.serialize_field("test_reports_count", &self.test_reports.len())?;
        state.end()
    }
}