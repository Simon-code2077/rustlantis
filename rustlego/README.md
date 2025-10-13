# RustLego

RustLego is an innovative fuzzing tool that combines LegoFuzz-inspired template methodology with LLM-powered code generation and Rustlantis differential testing to discover bugs in the Rust compiler.

## Overview

This project integrates three powerful approaches:

1. **Template-based Generation**: Uses LegoFuzz methodology to decompose complex functions into basic building blocks
2. **LLM-powered Code Generation**: Leverages large language models to generate diverse Rust functions from templates
3. **Differential Testing**: Uses Rustlantis-style testing across multiple backends (LLVM, Cranelift) and optimization levels

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Template       │    │  LLM Function    │    │  Function       │
│  Library        │───▶│  Generator       │───▶│  Pool           │
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                         │
                                                         ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Bug Reports    │◀───│  Differential    │◀───│  Program        │
│  & Statistics   │    │  Testing         │    │  Composer       │
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Features

- **5 Function Categories**: arithmetic, memory, control_flow, type_conversion, string_ops
- **Template-driven Prompts**: Generate specific, constrained, and adaptive prompts for LLMs
- **Function Composition**: Combine basic functions into complex programs using both simple combination and chaining strategies
- **Multi-backend Testing**: Test with LLVM and Cranelift backends across optimization levels 0-3
- **Statistical Analysis**: Comprehensive fuzzing statistics and recommendations
- **Modular Design**: Each component can be used independently or as part of the full pipeline

## Installation

### Prerequisites

- Rust 1.70+ with `rustc` and `cargo`
- For Cranelift backend: Install cranelift-codegen backend
- API access to an LLM service (OpenAI, or local models)

### Build

```bash
git clone <repository>
cd rustlego
cargo build --release
```

### Environment Setup

```bash
# For OpenAI
export OPENAI_API_KEY="your-api-key"

# For custom LLM endpoints
export LLM_API_KEY="your-api-key"
export LLM_BASE_URL="http://localhost:8000/v1"  # For local models
export LLM_MODEL="gpt-4"  # Or your preferred model
```

## Usage

### Command Line Tools

RustLego provides four main executables:

#### 1. Function Generation

```bash
# Generate 10 arithmetic functions
cargo run --bin generate -- --category arithmetic --count 10 --output generated_functions/

# Available categories: arithmetic, memory, control_flow, type_conversion, string_ops
```

#### 2. Program Composition

```bash
# Compose programs from generated functions
cargo run --bin compose -- --input generated_functions/ --output composed_programs/ --count 5 --functions-per-program 3

# Use chained composition
cargo run --bin compose -- --input generated_functions/ --output composed_programs/ --chained
```

#### 3. Differential Testing

```bash
# Test composed programs
cargo run --bin difftest -- --input composed_programs/ --output test_results/ --backends llvm,cranelift

# Custom optimization levels
cargo run --bin difftest -- --input composed_programs/ --opt-levels 0,2,3 --timeout 60
```

#### 4. Full Pipeline

```bash
# Run complete fuzzing pipeline
cargo run --bin pipeline -- --output pipeline_results/ --iterations 5 --functions-per-batch 15 --detailed-stats

# Quick single iteration
cargo run --bin pipeline -- --output quick_test/ --functions-per-batch 5 --programs-per-batch 3
```

### Library Usage

```rust
use rustlego::fuzzer::pipeline::{FuzzingPipeline, FuzzingConfig};
use rustlego::difftest::runner::DiffTestRunner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create fuzzing pipeline
    let config = FuzzingPipeline::default_config();
    let pipeline = FuzzingPipeline::new(config.difftest_config.clone())?;
    
    // Run fuzzing session
    let session = pipeline.run_fuzzing_session(config).await?;
    
    println!("Generated {} functions, found {} bugs", 
             session.generated_functions.len(), 
             session.bugs_found);
    
    Ok(())
}
```

## Template System

RustLego uses a comprehensive template system with 5 categories:

### Arithmetic Templates
- Basic operations: add, subtract, multiply, divide
- Overflow handling and wrapping operations
- Floating-point arithmetic with edge cases

### Memory Templates
- Vector operations and transformations
- Array manipulations and bounds checking
- Memory allocation patterns

### Control Flow Templates
- Conditional logic with various patterns
- Loop constructs and early termination
- Pattern matching and guards

### Type Conversion Templates
- Safe and unsafe casting operations
- Numeric type conversions
- String and collection conversions

### String Operations Templates
- String manipulation and formatting
- Unicode handling and validation
- Parsing and serialization

## Differential Testing

The tool tests programs across:

- **Backends**: LLVM (default), Cranelift
- **Optimization Levels**: 0, 1, 2, 3
- **Discrepancy Detection**: Exit codes, stdout/stderr differences, compilation failures

### Example Bug Report

```json
{
  "program_name": "combined_3_functions",
  "discrepancies": [
    {
      "backend1": "llvm",
      "backend2": "cranelift", 
      "optimization1": "2",
      "optimization2": "2",
      "discrepancy_type": "ExitCode",
      "details": "Exit codes differ: 0 vs 1"
    }
  ],
  "summary": {
    "total_tests": 8,
    "discrepancies_found": 1
  }
}
```

## Configuration

### LLM Configuration

The system supports multiple LLM providers:

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# Local models (ollama, vllm, etc.)
export LLM_BASE_URL="http://localhost:11434/v1"
export LLM_MODEL="codellama:7b"

# Custom endpoints
export LLM_BASE_URL="https://api.anthropic.com/v1"
export LLM_API_KEY="your-key"
export LLM_MODEL="claude-3-sonnet"
```

### Fuzzing Configuration

```rust
FuzzingConfig {
    functions_per_batch: 20,        // Functions to generate per category
    programs_per_batch: 10,         // Programs to compose per iteration
    categories: vec![               // Categories to include
        "arithmetic".to_string(),
        "memory".to_string(),
    ],
    max_function_complexity: 5,     // Maximum complexity score
    output_dir: PathBuf::from("results/"),
    difftest_config: DiffTestConfig {
        backends: vec!["llvm".to_string(), "cranelift".to_string()],
        optimization_levels: vec!["0".to_string(), "3".to_string()],
        timeout_seconds: 30,
        output_dir: PathBuf::from("difftest/"),
    },
}
```

## Output Structure

```
pipeline_output/
├── generated_functions/          # Individual generated functions
│   ├── 000001_add_numbers.rs
│   ├── 000002_process_array.rs
│   └── ...
├── composed_programs/            # Combined programs
│   ├── 001_combined_3_functions.rs
│   ├── 002_chained_4_functions.rs
│   └── ...
├── difftest/                     # Test results
│   ├── program1_report.json
│   ├── program2_report.json
│   └── ...
├── session_summary.json          # Overall session statistics
└── final_statistics.json         # Aggregated statistics
```

## Statistics and Analysis

RustLego provides comprehensive statistics:

- **Function Generation**: Success rates by category
- **Bug Detection**: Discrepancy types and frequency
- **Backend Analysis**: Which backend pairs find most bugs
- **Complexity Analysis**: Relationship between complexity and bug detection
- **Recommendations**: Actionable insights for improving fuzzing effectiveness

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out html

# Watch mode for development
cargo watch -x test
```

## License

This project is licensed under the MIT License - see LICENSE file for details.

## Acknowledgments

- **LegoFuzz**: Template-based fuzzing methodology
- **Rustlantis**: Differential testing inspiration
- **Rust Compiler Team**: For the robust infrastructure that makes this testing possible