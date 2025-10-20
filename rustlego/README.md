# RustLego

RustLego is an innovative Rust code generation tool that combines LegoFuzz-inspired template methodology with LLM-powered code generation, focused on producing Rust code for differential testing.

## Overview

This project integrates two powerful approaches:

1. **Template-based Generation**: Uses LegoFuzz methodology to decompose complex functions into basic building blocks
2. **LLM-powered Code Generation**: Leverages large language models to generate diverse Rust functions from templates

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
│  Generated      │    │  Original        │◀───│  Program        │
│  Rust Files     │    │  difftest Binary │    │  Composer       │
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Features

- **5 Function Categories**: arithmetic, memory, control_flow, type_conversion, string_ops
- **Template-driven Prompts**: Generate specific, constrained, and adaptive prompts for LLMs
- **Function Composition**: Combine basic functions into complex programs using both simple combination and chaining strategies
- **Decoupled Design**: rustlego focuses on code generation, works with any testing tool
- **Statistical Analysis**: Comprehensive generation statistics and recommendations
- **Modular Design**: Each component can be used independently or as part of the full pipeline

## Installation

### Prerequisites

- Rust 1.70+ with `rustc` and `cargo`
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

RustLego provides three main executables:

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

#### 3. Full Pipeline

```bash
# Run complete generation pipeline
cargo run --bin pipeline -- --output pipeline_results/ --iterations 5 --functions-per-batch 15

# Quick single iteration
cargo run --bin pipeline -- --output quick_test/ --functions-per-batch 5 --programs-per-batch 3
```

### Integration with Original difftest

After generating code, you can directly use the original difftest binary for testing:

```bash
# Test single file
cargo run -p difftest -- composed_programs/001_combined_3_functions.rs

# Batch test all files in directory
for file in composed_programs/*.rs; do
    echo "Testing $file"
    cargo run -p difftest -- "$file"
done
```

### Library Usage

```rust
use rustlego::composer::Composer;
use rustlego::fuzzer::pipeline::{FuzzingPipeline, FuzzingConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create function composer
    let composer = Composer::new();
    
    // Compose programs from existing functions
    let programs = composer.compose_from_directory("test_functions", 5, 3)?;
    
    println!("Generated {} programs", programs.len());
    
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

### Generation Configuration

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
├── session_summary.json          # Overall session statistics
└── final_statistics.json         # Aggregated statistics
```

## Statistics and Analysis

RustLego provides comprehensive statistics:

- **Function Generation**: Success rates by category
- **Complexity Analysis**: Relationship between complexity and quality
- **Recommendations**: Actionable insights for improving generation effectiveness

## Advantages

1. **Decoupled Design**: rustlego focuses on code generation, testing tools focus on testing
2. **Simplified Dependencies**: Reduced complex dependencies and coupling
3. **Flexibility**: Can use rustlego standalone for code generation, then test with any tool
4. **Compatibility**: Generated files are fully compatible with original difftest and other Rust tools

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