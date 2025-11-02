#!/bin/bash
# RustLego 一键安装脚本

set -e  # 遇到错误就退出

echo "🚀 RustLego 一键安装脚本"
echo "========================="

# 检查Python3
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 未安装，请先安装Python3"
    exit 1
fi

echo "✅ Python3 已安装: $(python3 --version)"

# 检查Rust
if ! command -v rustc &> /dev/null; then
    echo "⚠️  Rust 未安装，正在安装..."
    echo "请访问 https://rustup.rs/ 安装Rust，然后重新运行此脚本"
    exit 1
fi

echo "✅ Rust 已安装: $(rustc --version)"

# 创建虚拟环境
if [ ! -d "venv" ]; then
    echo "🔧 创建Python虚拟环境..."
    python3 -m venv venv
    echo "✅ 虚拟环境创建成功"
else
    echo "✅ 虚拟环境已存在"
fi

# 激活虚拟环境
echo "🔧 激活虚拟环境..."
source venv/bin/activate

# 安装依赖
echo "📦 安装Python依赖..."
pip install -r requirements.txt

echo "📦 安装LLM客户端依赖..."
echo "选择LLM服务 (可以安装多个):"
echo "1) OpenAI GPT (推荐)"
echo "2) Anthropic Claude"  
echo "3) 本地LLM (Ollama)"
echo "4) 全部安装"
echo "5) 跳过 (稍后手动安装)"

read -p "请选择 (1-5): " choice

case $choice in
    1)
        pip install openai
        echo "✅ OpenAI客户端已安装"
        ;;
    2)
        pip install anthropic
        echo "✅ Claude客户端已安装"
        ;;
    3)
        pip install aiohttp
        echo "✅ 本地LLM客户端已安装"
        echo "💡 请运行 'python setup_llm.py' 配置Ollama"
        ;;
    4)
        pip install openai anthropic aiohttp
        echo "✅ 所有LLM客户端已安装"
        ;;
    5)
        echo "⚠️  跳过LLM客户端安装"
        echo "稍后请运行: pip install openai 或 pip install anthropic 或 pip install aiohttp"
        ;;
    *)
        echo "无效选择，跳过LLM客户端安装"
        ;;
esac

echo ""
echo "🎉 安装完成!"
echo "============"

# 运行系统检查
echo "🔍 运行系统检查..."
python3 system_check.py

echo ""
echo "📋 下一步操作:"
echo "1. 激活虚拟环境: source venv/bin/activate"
echo "2. 配置LLM服务: python setup_llm.py"  
echo "3. 测试配置: python test_llm_config.py"
echo "4. 运行演示: python production_demo.py"

echo ""
echo "📚 更多信息请查看 README.md"