# RustLego 快速开始指南

## 🚀 工具已成功创建！

恭喜！RustLego 模糊测试工具已经成功构建并可以运行。以下是使用指南：

## 📋 修复的问题

✅ **解决了 clap 短选项冲突**
- `generate` 命令：`-c` (category), `-n` (count), `-o` (output)
- `compose` 命令：`-i` (input), `-c` (count), `-f` (functions-per-program), `-o` (output)  
- `difftest` 命令：`-i` (input), `-o` (output), `-b` (backends), `-t` (timeout)
- `pipeline` 命令：`-o` (output), 其他使用长选项

✅ **所有二进制文件编译成功**
- `generate` - 函数生成器
- `compose` - 程序组合器  
- `difftest` - 差分测试器
- `pipeline` - 完整流水线
- `rustlego` - 主程序（显示帮助信息）

## 🔧 使用前准备

### 1. 设置 API 密钥（用于 LLM 生成）

```bash
# 使用 OpenAI
export OPENAI_API_KEY="your-openai-api-key"

# 或者使用本地 LLM
export LLM_API_KEY="your-key"
export LLM_BASE_URL="http://localhost:8000/v1" 
export LLM_MODEL="codellama:7b"
```

### 2. 确保有 Rust 编译器后端

```bash
# 默认 LLVM 后端已安装
# 对于 Cranelift 后端（可选）：
rustup component add cranelift-codegen
```

## 📖 使用示例

### 1. 查看可用命令

```bash
cargo run --bin rustlego
```

### 2. 生成函数（需要 API 密钥）

```bash
# 生成 5 个算术函数
cargo run --bin generate -- -c arithmetic -n 5 -o generated/

# 生成不同类别的函数
cargo run --bin generate -- -c memory -n 3 -o generated/
cargo run --bin generate -- -c control_flow -n 2 -o generated/
```

### 3. 组合程序

```bash
# 从生成的函数组合程序
cargo run --bin compose -- -i generated/ -o composed/ -c 3 -f 2

# 使用链式组合
cargo run --bin compose -- -i generated/ -o composed/ --chained
```

### 4. 运行差分测试

```bash
# 测试组合的程序
cargo run --bin difftest -- -i composed/ -o results/ -b llvm,cranelift
```

### 5. 运行完整流水线

```bash
# 单次迭代（需要 API 密钥）
cargo run --bin pipeline -- -o pipeline_results/ --detailed-stats

# 多次迭代
cargo run --bin pipeline -- -o multi_results/ --iterations 3 --functions-per-batch 15
```

## 🎯 无需 API 密钥的测试

如果暂时没有 LLM API 密钥，可以：

1. **查看帮助信息**：
```bash
cargo run --bin generate -- --help
cargo run --bin compose -- --help  
cargo run --bin difftest -- --help
cargo run --bin pipeline -- --help
```

2. **测试项目结构**：
```bash
# 编译检查
cargo check

# 运行测试
cargo test
```

3. **手动创建测试函数**：
```bash
mkdir -p manual_test/
echo "fn test_add(a: i32, b: i32) -> i32 { a + b }" > manual_test/test.rs
cargo run --bin compose -- -i manual_test/ -o composed_manual/
```

## 📊 期望输出

### 成功的 pipeline 运行会显示：

```
🦀 RustLego Fuzzing Pipeline
============================
🚀 Running single fuzzing iteration...

=== Phase 1: Function Generation ===
Generating functions for category: arithmetic
  Generated 5 functions

=== Phase 2: Program Composition ===  
Composing program 1 with 3 functions
  Composed 2 programs

=== Phase 3: Differential Testing ===
  ✅ program1: no discrepancies
  🐛 program2: 1 discrepancies

📊 Session Results:
  Functions generated: 15
  Programs composed: 6  
  Bugs found: 1

🎉 Found 1 potential bugs! Check the output directory for details.
```

## 🛠️ 下一步

1. **获取 LLM API 密钥**以使用完整功能
2. **运行真实的模糊测试会话**
3. **分析生成的 bug 报告**
4. **根据统计建议优化配置**

## ✅ 项目状态总结

- ✅ 完整的模块化架构
- ✅ 5 个函数模板类别  
- ✅ LLM 集成（支持多个提供商）
- ✅ 函数组合系统
- ✅ 差分测试框架
- ✅ 统计分析和建议
- ✅ 4 个命令行工具
- ✅ 详细的错误处理
- ✅ 完善的帮助文档

项目已经可以投入使用！🎉