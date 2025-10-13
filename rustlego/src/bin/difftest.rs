use rustlego::composer::combiner::ComposedProgram;
use rustlego::difftest::runner::{DiffTestRunner, DiffTestConfig};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tokio::fs;

#[derive(Parser)]
#[command(name = "rustlego-difftest")]
#[command(about = "Run differential testing on composed Rust programs")]
struct DiffTestArgs {
    /// Directory containing composed programs
    #[arg(short = 'i', long)]
    input: PathBuf,
    
    /// Output directory for test results
    #[arg(short = 'o', long, default_value = "difftest_results")]
    output: PathBuf,
    
    /// Backends to test (comma-separated)
    #[arg(short = 'b', long, default_value = "llvm,cranelift")]
    backends: String,
    
    /// Optimization levels to test (comma-separated)
    #[arg(long, default_value = "0,1,2,3")]
    opt_levels: String,
    
    /// Timeout in seconds
    #[arg(short = 't', long, default_value = "30")]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = DiffTestArgs::parse();
    
    println!("RustLego Differential Tester");
    println!("============================");
    println!("Input: {}", args.input.display());
    println!("Output: {}", args.output.display());
    println!("Backends: {}", args.backends);
    println!("Optimization levels: {}", args.opt_levels);
    println!("Timeout: {}s", args.timeout);
    
    // Parse configuration
    let backends: Vec<String> = args.backends.split(',').map(|s| s.trim().to_string()).collect();
    let opt_levels: Vec<String> = args.opt_levels.split(',').map(|s| s.trim().to_string()).collect();
    
    let config = DiffTestConfig {
        backends,
        optimization_levels: opt_levels,
        timeout_seconds: args.timeout,
        output_dir: args.output.clone(),
    };
    
    // Load composed programs
    let programs = load_programs_from_directory(&args.input).await?;
    println!("Loaded {} programs for testing", programs.len());
    
    if programs.is_empty() {
        println!("No programs found to test.");
        return Ok(());
    }
    
    // Run differential testing
    let runner = DiffTestRunner::new(config);
    let reports = runner.run_batch_tests(&programs).await?;
    
    // Summary
    let total_discrepancies: usize = reports.iter().map(|r| r.discrepancies.len()).sum();
    
    println!("\n📊 Differential Testing Results:");
    println!("  Programs tested: {}", reports.len());
    println!("  Total discrepancies found: {}", total_discrepancies);
    
    for report in &reports {
        if !report.discrepancies.is_empty() {
            println!("  🐛 {}: {} discrepancies", report.program_name, report.discrepancies.len());
        } else {
            println!("  ✅ {}: no discrepancies", report.program_name);
        }
    }
    
    if total_discrepancies > 0 {
        println!("\n🎉 Found {} potential bugs! Check the results directory for details.", total_discrepancies);
    } else {
        println!("\n✅ No discrepancies found in this batch.");
    }
    
    Ok(())
}

async fn load_programs_from_directory(dir: &PathBuf) -> Result<Vec<ComposedProgram>> {
    let mut programs = Vec::new();
    let mut entries = fs::read_dir(dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path).await?;
            
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            programs.push(ComposedProgram {
                name,
                code: content,
                functions: Vec::new(), // We don't need to reconstruct the original functions for testing
                complexity: 1, // Simplified
                main_function: String::new(), // Will be extracted from code if needed
            });
        }
    }
    
    Ok(programs)
}