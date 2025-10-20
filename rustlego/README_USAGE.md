# RustLego 使用指南

## 概述

RustLego 现在是一个纯粹的 Rust 代码生成工具，专注于生成和组合函数。生成的文件可以直接使用原始的 difftest 二进制进行差分测试。

## 基本工作流程

### 1. 查看帮助信息
```bash
cargo run --bin rustlego
```

### 2. 组合现有函数
```bash
# 使用现有的测试函数进行组合
cargo run --bin compose -- --input ../test_functions --output composed_programs --count 5

# 查看生成的文件
ls composed_programs/
```

### 3. 使用原始 difftest 进行测试
```bash
# 测试单个文件
cargo run -p difftest -- composed_programs/001_combined_3_functions.rs

# 批量测试目录中的所有文件
for file in composed_programs/*.rs; do
    echo "Testing $file"
    cargo run -p difftest -- "$file"
done
```

## 可用命令

### generate（需要 API 密钥）
生成新的函数：
```bash
cargo run --bin generate -- --category arithmetic --count 10 --output generated_functions
```

### compose
组合函数成程序：
```bash
cargo run --bin compose -- --input generated_functions --output composed_programs --count 5 --functions-per-program 3
```

### pipeline
运行完整的生成和组合流程：
```bash
cargo run --bin pipeline -- --iterations 3 --functions-per-batch 10 --programs-per-batch 5
```

## 优势

1. **解耦设计**：rustlego 专注于代码生成，difftest 专注于差分测试
2. **简化依赖**：减少了复杂的依赖关系和耦合
3. **灵活性**：可以单独使用 rustlego 生成代码，然后用任何测试工具进行测试
4. **兼容性**：生成的文件完全兼容原始 difftest 工具

## 文件结构

```
rustlego/
├── src/
│   ├── bin/
│   │   ├── generate.rs    # 函数生成
│   │   ├── compose.rs     # 函数组合
│   │   └── pipeline.rs    # 完整流程
│   ├── composer/          # 组合逻辑
│   ├── fuzzer/           # 流程管理
│   ├── llm/              # LLM 集成
│   └── templates/        # 模板系统
└── composed_programs/    # 生成的测试文件（示例）
```

## 注意事项

- `generate` 命令需要配置 LLM API 密钥
- `compose` 命令可以直接使用现有的函数文件
- 所有生成的文件都是标准的 Rust 代码，可以用任何 Rust 工具进行处理