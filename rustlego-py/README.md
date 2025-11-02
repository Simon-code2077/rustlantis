# RustLego-Py: 高级Rust代码生成与智能组合系统

这是一个先进的Rust代码生成和验证工具，具备深度类型分析、智能函数链匹配和高性能组合优化能力。

## 🚀 核心特性

- � **智能类型推导**: 深度分析 `Result<T,E>`、`Option<T>`、`Vec<T>` 等复杂类型
- 🔗 **高级函数链匹配**: 多策略搜索算法，自动构建最优函数调用链
- ⚡ **自适应性能优化**: 小规模图搜索，大规模贪心算法，毫秒级响应
- ✅ **实时编译验证**: 集成Cargo，确保生成代码100%可编译
- 🎯 **真实代码生成**: 生成可运行的完整Rust程序，包含智能参数推导
- 📊 **详细性能分析**: 提供覆盖率、复杂度、类型多样性等深度统计

## 🔥 技术突破

### 深度类型匹配
```rust
// 自动处理复杂类型转换链
fn load_config() -> Result<Config, Error>
fn extract_url(config: Result<Config, Error>) -> Option<String>  
fn create_pool(url: Option<String>) -> Result<Pool, Error>
```

### 智能函数组合
- **最长路径策略**: 寻找包含最多函数的调用链
- **复杂度优化策略**: 优先组合高价值函数
- **类型多样性策略**: 最大化类型覆盖范围

### 高性能扩展
| 函数规模 | 处理时间 | 覆盖率 | 算法策略 |
|---------|---------|-------|---------|
| 10-30个 | 3-10ms  | 60-80% | 图搜索 |
| 50-100个| 1-5ms   | 40-60% | 贪心算法 |
| 100+个  | 3-8ms   | 30-50% | 优化贪心 |

## 🛠️ 项目结构

```
rustlego-py/
├── README.md
├── ADVANCED_FEATURES.md      # 高级功能详细文档
├── requirements.txt
├── rustlego/
│   ├── __init__.py
│   ├── validator.py          # 高性能编译验证器
│   ├── generator.py          # LLM驱动函数生成器
│   ├── combiner.py           # 智能函数组合器 (核心)
│   ├── templates.py          # 高级模板系统
│   └── utils.py              # 工具函数
├── scripts/
│   ├── generate.py           # 生成函数命令
│   ├── compose.py            # 智能组合命令
│   └── validate.py           # 批量验证命令
├── examples/
│   ├── generated/            # 生成的函数示例
│   ├── composed/             # 基础组合程序
│   ├── chained/              # 智能链式程序
│   ├── working/              # 完整工作示例
│   └── advanced/             # 高级功能演示
├── test_advanced_chaining.py # 高级链匹配测试
├── performance_test.py       # 性能基准测试
├── working_demo.py          # 完整工作演示
└── advanced_demo.py         # 高级功能展示
```

## 🚀 快速开始

### 🎯 一键安装 (推荐新用户)
```bash
# 下载项目并运行安装脚本
git clone <repository-url>
cd rustlego-py
./install.sh

# 或手动安装 (见下方)
```

### 1. 环境设置
```bash
# 确保已安装 Rust 和 Cargo
rustc --version
cargo --version

# 创建Python虚拟环境 (推荐)
python3 -m venv venv
source venv/bin/activate  # Linux/Mac
# 或 venv\Scripts\activate  # Windows

# 安装 Python 依赖
pip install -r requirements.txt

# 安装 LLM 客户端依赖（选择一个）
pip install openai          # OpenAI GPT
pip install anthropic       # Claude
pip install aiohttp         # 本地LLM (如Ollama)
```

### 2. 🔑 LLM配置 (必需)

**重要**: 系统需要真实的LLM服务才能工作，现在配置更加简单！

#### 🎯 简化配置 (推荐方式)

1. **创建配置文件**:
```bash
# 运行配置工具，自动创建 llm_config.json
python simple_config.py
```

2. **编辑配置文件**:
```json
{
  "llm_type": "qwen",
  "model": "qwen-coder-turbo",
  "api_key": null,
  "base_url": "https://dashscope.aliyuncs.com/api/v1",
  "temperature": 0.1,
  "max_tokens": 2000,
  "top_p": 0.8
}
```

3. **支持的配置示例**:

**通义千问 (Qwen)**:
```json
{
  "llm_type": "qwen",
  "model": "qwen-coder-turbo",
  "api_key": "your-dashscope-api-key",
  "base_url": "https://dashscope.aliyuncs.com/api/v1",
  "temperature": 0.1,
  "max_tokens": 2000
}
```

**OpenAI**:
```json
{
  "llm_type": "openai", 
  "model": "gpt-3.5-turbo",
  "api_key": "your-openai-api-key",
  "temperature": 0.2,
  "max_tokens": 2000
}
```

**本地模型 (Ollama)**:
```json
{
  "llm_type": "local",
  "model": "codellama:7b", 
  "base_url": "http://localhost:11434",
  "temperature": 0.2,
  "num_predict": 1000
}
```

#### 🏃‍♂️ 快速验证
```bash
# 验证配置是否正确
python simple_config.py

# 或直接测试生成器
python rustlego/generator.py
```



### 3. 基础功能体验
```bash
# 生成简单函数 (需要先配置LLM)
python3 scripts/generate.py --category arithmetic --count 5 --validate

# 基础函数组合
python3 scripts/compose.py --input examples/generated/ --output examples/composed/
```



### 3. 编译运行生成的程序
```bash
# 编译并运行完整示例
cd examples/working
rustc complete_chain.rs -o complete_chain
./complete_chain
```

