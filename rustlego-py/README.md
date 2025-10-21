# RustLego-Py: Python版本的Rust代码生成器

这是一个用Python实现的Rust代码生成和验证工具，专注于生成高质量的Rust函数并进行编译验证。

## 特性

- 🤖 **LLM驱动生成**: 使用大语言模型生成Rust函数
- ✅ **编译验证**: 自动验证生成的函数是否能编译
- 🔧 **函数组合**: 将多个函数组合成复杂程序
- 📊 **详细统计**: 提供生成和验证的详细统计信息
- 🚀 **易于扩展**: Python实现，易于修改和扩展

## 项目结构

```
rustlego-py/
├── README.md
├── requirements.txt
├── rustlego/
│   ├── __init__.py
│   ├── validator.py          # 编译验证器
│   ├── generator.py          # 函数生成器
│   ├── combiner.py           # 函数组合器
│   ├── templates.py          # 模板系统
│   └── utils.py              # 工具函数
├── scripts/
│   ├── generate.py           # 生成函数命令
│   ├── compose.py            # 组合函数命令
│   └── validate.py           # 验证命令
└── examples/
    ├── generated/            # 生成的函数示例
    └── composed/             # 组合的程序示例
```

## 快速开始

### 1. 安装依赖
```bash
pip install -r requirements.txt
```

### 2. 生成函数
```bash
python scripts/generate.py --category arithmetic --count 10 --validate
```

### 3. 组合程序
```bash
python scripts/compose.py --input generated/ --output composed/ --validate
```

### 4. 批量验证
```bash
python scripts/validate.py --input generated/ --stats
```

## 与Rust版本的对比

| 特性 | Rust版本 | Python版本 |
|------|----------|------------|
| 类型安全 | 编译时保证 | 运行时检查 |
| 开发速度 | 较慢 | 快速 |
| 错误处理 | 严格 | 灵活 |
| 扩展性 | 需重编译 | 动态修改 |
| 性能 | 高 | 中等 |

## 优势

1. **快速原型**: Python的灵活性使得快速原型开发成为可能
2. **易于调试**: 动态语言特性，便于调试和错误排查
3. **丰富生态**: 可以利用Python丰富的机器学习和数据处理库
4. **简单部署**: 无需编译，直接运行