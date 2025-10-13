use rustlego::composer::combiner::FunctionCombiner;
use rustlego::llm::function_generator::GeneratedFunction;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tokio::fs;

#[derive(Parser)]
#[command(name = "rustlego-compose")]
#[command(about = "Compose multiple Rust functions into complex programs")]
struct ComposeArgs {
    /// Directory containing generated functions
    #[arg(short = 'i', long)]
    input: PathBuf,
    
    /// Output directory for composed programs
    #[arg(short = 'o', long, default_value = "composed")]
    output: PathBuf,
    
    /// Number of programs to compose
    #[arg(short = 'c', long, default_value = "3")]
    count: usize,
    
    /// Functions per program
    #[arg(short = 'f', long, default_value = "3")]
    functions_per_program: usize,
    
    /// Use chained composition instead of simple combination
    #[arg(long)]
    chained: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = ComposeArgs::parse();
    
    println!("RustLego Function Composer");
    println!("==========================");
    println!("Input: {}", args.input.display());
    println!("Output: {}", args.output.display());
    println!("Programs to compose: {}", args.count);
    println!("Functions per program: {}", args.functions_per_program);
    println!("Composition mode: {}", if args.chained { "chained" } else { "combined" });
    
    // Load generated functions from input directory
    let functions = load_functions_from_directory(&args.input).await?;
    println!("Loaded {} functions", functions.len());
    
    if functions.len() < args.functions_per_program {
        eprintln!("Not enough functions to compose programs. Need at least {} functions.", args.functions_per_program);
        return Ok(());
    }
    
    // Create output directory
    fs::create_dir_all(&args.output).await?;
    
    let combiner = FunctionCombiner::new();
    let mut composed_programs = Vec::new();
    
    // Compose programs
    for i in 0..args.count {
        let start_idx = (i * args.functions_per_program) % functions.len();
        let end_idx = ((i + 1) * args.functions_per_program).min(functions.len());
        
        if end_idx <= start_idx {
            break;
        }
        
        let functions_subset = &functions[start_idx..end_idx];
        
        let program = if args.chained {
            combiner.create_chained_composition(functions_subset)?
        } else {
            combiner.combine_functions(functions_subset)?
        };
        
        // Save program to file
        let program_file = args.output.join(format!("{:03}_{}.rs", i + 1, program.name));
        fs::write(&program_file, &program.code).await?;
        
        println!("  ✓ Composed program {}: {}", i + 1, program.name);
        composed_programs.push(program);
    }
    
    println!("\n✅ Successfully composed {} programs!", composed_programs.len());
    
    Ok(())
}

async fn load_functions_from_directory(dir: &PathBuf) -> Result<Vec<GeneratedFunction>> {
    let mut functions = Vec::new();
    let mut entries = fs::read_dir(dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path).await?;
            
            // Extract function info from comment headers
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let category = extract_from_comment(&content, "Category:")
                .unwrap_or_else(|| "unknown".to_string());
            
            let complexity = extract_from_comment(&content, "Complexity:")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            
            functions.push(GeneratedFunction {
                name,
                code: content.clone(),
                category,
                complexity,
                template_used: extract_from_comment(&content, "Template:"),
            });
        }
    }
    
    Ok(functions)
}

fn extract_from_comment(content: &str, prefix: &str) -> Option<String> {
    content
        .lines()
        .find(|line| line.trim_start().starts_with("//") && line.contains(prefix))
        .and_then(|line| {
            line.split(prefix)
                .nth(1)?
                .trim()
                .to_string()
                .into()
        })
}