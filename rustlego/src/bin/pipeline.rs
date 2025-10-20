use rustlego::fuzzer::pipeline::{FuzzingPipeline, FuzzingConfig};
use rustlego::fuzzer::statistics::StatisticsCollector;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustlego-pipeline")]
#[command(about = "Run the complete generation pipeline: generate → compose")]
struct PipelineArgs {
    /// Output directory for all results
    #[arg(short = 'o', long, default_value = "pipeline_output")]
    output: PathBuf,
    
    /// Number of functions to generate per category
    #[arg(long, default_value = "10")]
    functions_per_batch: usize,
    
    /// Number of programs to compose
    #[arg(long, default_value = "5")]
    programs_per_batch: usize,
    
    /// Maximum function complexity
    #[arg(long, default_value = "5")]
    max_complexity: u32,
    
    /// Function categories (comma-separated)
    #[arg(long, default_value = "arithmetic,memory,control_flow")]
    categories: String,
    
    /// Number of iterations to run
    #[arg(short, long, default_value = "1")]
    iterations: usize,
    
    /// Show detailed statistics
    #[arg(long)]
    detailed_stats: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = PipelineArgs::parse();
    
    println!("🚀 RustLego Generation Pipeline");
    println!("==============================");
    println!("Output directory: {}", args.output.display());
    println!("Functions per batch: {}", args.functions_per_batch);
    println!("Programs per batch: {}", args.programs_per_batch);
    println!("Max complexity: {}", args.max_complexity);
    println!("Categories: {}", args.categories);
    println!("Iterations: {}", args.iterations);
    
    // Parse configuration
    let categories: Vec<String> = args.categories.split(',').map(|s| s.trim().to_string()).collect();
    let fuzzing_config = FuzzingConfig {
        functions_per_batch: args.functions_per_batch,
        programs_per_batch: args.programs_per_batch,
        categories,
        max_function_complexity: args.max_complexity,
        output_dir: args.output.clone(),
    };
    
    // Create pipeline
    let pipeline = FuzzingPipeline::new()?;
    
    // Run fuzzing
    let sessions = if args.iterations == 1 {
        vec![pipeline.run_fuzzing_session(fuzzing_config).await?]
    } else {
        pipeline.run_continuous_fuzzing(fuzzing_config, args.iterations).await?
    };
    
    // Generate statistics
    let stats = StatisticsCollector::analyze_sessions(&sessions);
    StatisticsCollector::print_statistics(&stats);
    
    if args.detailed_stats {
        println!("\n💡 Recommendations:");
        let recommendations = StatisticsCollector::generate_recommendations(&stats);
        for (i, rec) in recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, rec);
        }
    }
    
    // Save final statistics
    let stats_file = args.output.join("final_statistics.json");
    let stats_json = serde_json::to_string_pretty(&stats)?;
    tokio::fs::write(stats_file, stats_json).await?;
    
    println!("\n🎯 Pipeline completed! Check {} for detailed results.", args.output.display());
    
    Ok(())
}