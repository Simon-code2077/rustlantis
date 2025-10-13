use rustlego::llm::function_generator::FunctionGenerator;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustlego-generate")]
#[command(about = "Generate Rust functions using LLM and templates")]
struct GenerateArgs {
    /// Template category to generate from
    #[arg(short = 'c', long)]
    category: String,
    
    /// Number of functions to generate
    #[arg(short = 'n', long, default_value = "5")]
    count: usize,
    
    /// Output directory
    #[arg(short = 'o', long, default_value = "generated")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = GenerateArgs::parse();
    
    println!("RustLego Function Generator");
    println!("===========================");
    println!("Category: {}", args.category);
    println!("Count: {}", args.count);
    println!("Output: {}", args.output.display());
    
    let generator = FunctionGenerator::new()?;
    let functions = generator.generate_batch(&args.category, args.count).await?;
    
    generator.save_functions_to_directory(&functions, &args.output).await?;
    
    println!("\n✅ Successfully generated {} functions!", functions.len());
    
    Ok(())
}