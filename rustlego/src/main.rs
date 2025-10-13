use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 RustLego - Template-based Rust Fuzzing Tool");
    println!("===============================================");
    println!();
    println!("RustLego combines LegoFuzz templates with LLM generation");
    println!("and Rustlantis differential testing to find Rust compiler bugs.");
    println!();
    println!("Available commands:");
    println!("  cargo run --bin generate   - Generate functions from templates");
    println!("  cargo run --bin compose    - Compose programs from functions");  
    println!("  cargo run --bin difftest   - Run differential testing");
    println!("  cargo run --bin pipeline   - Run complete fuzzing pipeline");
    println!();
    println!("Example:");
    println!("  cargo run --bin pipeline -- --iterations 3 --detailed-stats");
    println!();
    println!("For help with any command, use --help:");
    println!("  cargo run --bin generate -- --help");
    
    Ok(())
}