use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 RustLego - Template-based Rust Code Generator");
    println!("===============================================");
    println!();
    println!("RustLego combines LegoFuzz templates with LLM generation");
    println!("to create Rust programs for compiler testing.");
    println!();
    println!("Available commands:");
    println!("  cargo run --bin generate   - Generate functions from templates");
    println!("  cargo run --bin compose    - Compose programs from functions");  
    println!("  cargo run --bin pipeline   - Run generation and composition pipeline");
    println!();
    println!("Example:");
    println!("  cargo run --bin pipeline -- --iterations 3");
    println!("  cargo run --bin generate -- --count 10 --category arithmetic");
    println!("  cargo run --bin compose -- --input test_functions --output test_composed");
    println!();
    println!("Generated Rust files can be tested with the original difftest binary:");
    println!("  difftest <generated_file.rs>");
    println!();
    println!("For help with any command, use --help:");
    println!("  cargo run --bin generate -- --help");
    
    Ok(())
}