# 🚀 RustLego-Py 快速操作指南

## 1分钟快速体验

```bash
# 克隆项目
git clone <repo-url>
cd rustlego-py

# 检查环境
rustc --version && cargo --version && python3 --version

# 体验核心功能
python3 working_demo.py          # 完整工作链演示
python3 test_advanced_chaining.py   # 高级功能测试
python3 advanced_demo.py        # 完整功能展示
```

## 核心命令速查

### 🎯 演示命令 (推荐)
```bash
python3 working_demo.py          # ⭐ 最佳入门体验
python3 test_advanced_chaining.py   # 🔬 技术深度体验  
python3 quick_test.py            # ⚡ 快速性能测试
python3 advanced_demo.py        # 🚀 完整功能展示
```

### 🛠️ 工具命令
```bash
# 函数生成
python3 scripts/generate.py --category arithmetic --count 5 --validate

# 智能组合
python3 scripts/compose.py --input examples/generated/ --output examples/composed/

# 批量验证
python3 scripts/validate.py --input examples/ --stats
```

### 🏃‍♂️ 快速验证
```bash
# 编译运行示例
cd examples/working
rustc complete_chain.rs -o complete_chain && ./complete_chain

# 验证所有功能
python3 verify_readme.py
```

## 🎯 核心亮点

- **3ms内处理100个函数** - 极致性能
- **60-80%覆盖率** - 智能链匹配  
- **100%可编译** - 真实代码生成
- **复杂类型支持** - Result/Option/Vec等

## 📁 重要文件

- `README.md` - 完整文档
- `ADVANCED_FEATURES.md` - 技术细节
- `rustlego/combiner.py` - 核心算法
- `examples/working/` - 工作示例
- `verify_readme.py` - 功能验证

---
**开始探索高级Rust代码生成的魅力吧！** 🦀✨