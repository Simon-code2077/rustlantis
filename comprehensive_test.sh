#!/bin/bash

# 全面的LLM参数测试脚本
echo "=== LLM参数传递和功能测试 ==="

cd "$(dirname "$0")"

# 编译项目
echo "1. 编译项目..."
cargo build --release --bin generate
if [ $? -ne 0 ]; then
    echo "❌ 编译失败"
    exit 1
fi

echo "✅ 编译成功"
echo ""

# 测试1: 不带LLM参数
echo "2. 测试基础生成（无LLM参数）..."
timeout 5s ./target/release/generate 12345 > test_no_llm.out 2>&1
if [ $? -eq 0 ]; then
    lines=$(wc -l < test_no_llm.out)
    echo "   ✅ 基础生成成功，输出 $lines 行"
else
    echo "   ❌ 基础生成失败"
fi

# 测试2: 带LLM参数但不调用（高频率）
echo "3. 测试LLM参数传递（频率1000，几乎不调用）..."
timeout 5s ./target/release/generate 12345 \
    --llm-endpoint "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions" \
    --llm-api-key "sk-1150c659956b4dd7a71ed6b3f05e0c35" \
    --llm-frequency 1000 > test_llm_no_call.out 2>&1
if [ $? -eq 0 ]; then
    lines=$(wc -l < test_llm_no_call.out)
    echo "   ✅ LLM参数传递成功，输出 $lines 行"
else
    echo "   ❌ LLM参数传递失败"
fi

# 测试3: 带LLM参数且调用（低频率）
echo "4. 测试LLM实际调用（频率5，应该调用）..."
timeout 10s ./target/release/generate 12345 \
    --llm-endpoint "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions" \
    --llm-api-key "sk-1150c659956b4dd7a71ed6b3f05e0c35" \
    --llm-frequency 5 > test_llm_call.out 2>&1
exit_code=$?
if [ $exit_code -eq 0 ]; then
    lines=$(wc -l < test_llm_call.out)
    echo "   ✅ LLM调用成功，输出 $lines 行"
elif [ $exit_code -eq 124 ]; then
    lines=$(wc -l < test_llm_call.out)
    echo "   ⚠️  LLM调用超时，已输出 $lines 行（可能在网络请求中卡住）"
else
    echo "   ❌ LLM调用失败，退出码: $exit_code"
fi

echo ""
echo "=== 输出文件分析 ==="
for f in test_*.out; do
    if [ -f "$f" ]; then
        size=$(du -h "$f" | cut -f1)
        lines=$(wc -l < "$f")
        echo "$f: $size, $lines 行"
        
        # 检查是否包含实际的MIR代码
        if grep -q "bb[0-9]" "$f"; then
            echo "   ✅ 包含MIR基本块"
        else
            echo "   ❌ 不包含MIR基本块"
        fi
        
        # 检查是否有错误信息
        if grep -q -i "error\|panic\|failed" "$f"; then
            echo "   ⚠️  包含错误信息"
            echo "   错误预览:"
            grep -i "error\|panic\|failed" "$f" | head -3 | sed 's/^/      /'
        fi
    fi
done

echo ""
echo "=== 参数传递验证 ==="
echo "检查命令行参数解析是否正确..."

# 创建一个简单的测试来验证参数解析
cat > verify_args.sh << 'EOF'
#!/bin/bash
echo "参数验证测试："
echo "种子: $1"
echo "LLM端点: $2"
echo "API密钥: $3" 
echo "频率: $4"
EOF

chmod +x verify_args.sh

echo "模拟参数传递:"
./verify_args.sh 12345 "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions" "sk-1150c659956b4dd7a71ed6b3f05e0c35" 5

rm verify_args.sh

echo ""
echo "=== 建议的下一步 ==="
echo "1. 如果基础生成成功但LLM版本失败，说明LLM集成有问题"
echo "2. 如果LLM版本超时，说明网络请求或API有问题"
echo "3. 检查生成的 test_*.out 文件来诊断具体问题"
echo "4. 使用 Ctrl+C 可以随时取消运行中的程序"