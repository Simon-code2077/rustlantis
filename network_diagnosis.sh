#!/bin/bash

echo "=== 网络连接诊断工具 ==="
echo

echo "1. 测试基本网络连接:"
if ping -c 2 -W 3 8.8.8.8 &>/dev/null; then
    echo "   ✅ 基本网络连接正常"
else
    echo "   ❌ 基本网络连接失败"
fi

echo
echo "2. 测试DNS解析:"
if nslookup dashscope.aliyuncs.com &>/dev/null; then
    echo "   ✅ DNS解析正常"
else
    echo "   ❌ DNS解析失败"
fi

echo
echo "3. 测试HTTPS端口连接:"
if timeout 5 bash -c "echo >/dev/tcp/dashscope.aliyuncs.com/443" 2>/dev/null; then
    echo "   ✅ HTTPS端口连接正常"
else
    echo "   ❌ HTTPS端口连接失败"
fi

echo
echo "4. 测试简单HTTP请求:"
curl_result=$(curl -s -w "%{http_code}" --connect-timeout 10 --max-time 15 https://dashscope.aliyuncs.com 2>/dev/null)
if [ $? -eq 0 ] && [ ! -z "$curl_result" ]; then
    echo "   ✅ HTTP请求成功，响应码: ${curl_result: -3}"
else
    echo "   ❌ HTTP请求失败"
fi

echo
echo "5. 检查代理设置:"
if [ ! -z "$http_proxy" ] || [ ! -z "$https_proxy" ] || [ ! -z "$HTTP_PROXY" ] || [ ! -z "$HTTPS_PROXY" ]; then
    echo "   ⚠️  检测到代理设置:"
    [ ! -z "$http_proxy" ] && echo "     http_proxy: $http_proxy"
    [ ! -z "$https_proxy" ] && echo "     https_proxy: $https_proxy"
    [ ! -z "$HTTP_PROXY" ] && echo "     HTTP_PROXY: $HTTP_PROXY"
    [ ! -z "$HTTPS_PROXY" ] && echo "     HTTPS_PROXY: $HTTPS_PROXY"
else
    echo "   ✅ 未检测到代理设置"
fi

echo
echo "6. 测试curl直接调用API:"
echo "   尝试简单的API测试请求..."
curl_api_test=$(curl -s -w "HTTP_CODE:%{http_code}" --connect-timeout 10 --max-time 15 \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer sk-1150c659956b4dd7a71ed6b3f05e0c35" \
    -d '{"model":"qwen-plus","messages":[{"role":"user","content":"测试"}],"max_tokens":10}' \
    https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions 2>&1)

if echo "$curl_api_test" | grep -q "HTTP_CODE:"; then
    http_code=$(echo "$curl_api_test" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
    echo "   API请求完成，HTTP状态码: $http_code"
    if [ "$http_code" = "200" ]; then
        echo "   ✅ API调用成功"
    elif [ "$http_code" = "401" ]; then
        echo "   ⚠️  API密钥认证失败（但连接正常）"
    else
        echo "   ⚠️  API返回状态码: $http_code"
    fi
else
    echo "   ❌ API请求失败: $curl_api_test"
fi