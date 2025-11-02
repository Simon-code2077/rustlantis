#!/usr/bin/env python3
"""
真实LLM客户端集成
"""

import os
from typing import Optional
from abc import ABC, abstractmethod

class LLMClient(ABC):
    """LLM客户端基类"""
    
    @abstractmethod
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        pass

class OpenAIClient(LLMClient):
    """OpenAI GPT客户端"""
    
    def __init__(self, 
                 api_key: Optional[str] = None,
                 model: str = "gpt-3.5-turbo",
                 base_url: Optional[str] = None):
        """
        初始化OpenAI客户端
        
        Args:
            api_key: API密钥（如果为None，从环境变量OPENAI_API_KEY读取）
            model: 使用的模型
            base_url: API基础URL（用于代理或自部署）
        """
        try:
            from openai import AsyncOpenAI
        except ImportError:
            raise ImportError("请安装OpenAI SDK: pip install openai")
        
        self.api_key = api_key or os.getenv("OPENAI_API_KEY")
        self.model = model
        
        if not self.api_key:
            raise ValueError("必须提供OpenAI API密钥，请设置环境变量OPENAI_API_KEY或传入api_key参数")
        
        self.client = AsyncOpenAI(
            api_key=self.api_key,
            base_url=base_url
        )
    
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        try:
            response = await self.client.chat.completions.create(
                model=self.model,
                messages=[
                    {
                        "role": "system", 
                        "content": "你是一个专业的Rust程序员，专门生成安全、高质量的Rust函数。只返回Rust代码，用```rust代码块包围。"
                    },
                    {
                        "role": "user", 
                        "content": prompt
                    }
                ],
                temperature=0.7,
                max_tokens=1000
            )
            
            return response.choices[0].message.content
            
        except Exception as e:
            raise RuntimeError(f"OpenAI API调用失败: {e}")

class ClaudeClient(LLMClient):
    """Anthropic Claude客户端"""
    
    def __init__(self, 
                 api_key: Optional[str] = None,
                 model: str = "claude-3-sonnet-20240229"):
        """
        初始化Claude客户端
        """
        try:
            from anthropic import AsyncAnthropic
        except ImportError:
            raise ImportError("请安装Anthropic SDK: pip install anthropic")
        
        self.api_key = api_key or os.getenv("ANTHROPIC_API_KEY")
        self.model = model
        
        if not self.api_key:
            raise ValueError("必须提供Anthropic API密钥，请设置环境变量ANTHROPIC_API_KEY或传入api_key参数")
        
        self.client = AsyncAnthropic(api_key=self.api_key)
    
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        try:
            response = await self.client.messages.create(
                model=self.model,
                max_tokens=1000,
                temperature=0.7,
                messages=[
                    {
                        "role": "user",
                        "content": f"你是一个专业的Rust程序员，专门生成安全、高质量的Rust函数。只返回Rust代码，用```rust代码块包围。\n\n{prompt}"
                    }
                ]
            )
            
            return response.content[0].text
            
        except Exception as e:
            raise RuntimeError(f"Claude API调用失败: {e}")

class QwenClient(LLMClient):
    """Qwen API客户端"""
    
    def __init__(self, 
                 api_key: Optional[str] = None,
                 model: str = "qwen-turbo",
                 base_url: Optional[str] = None,
                 temperature: float = 0.3):  # 添加 temperature 参数
        """
        初始化Qwen客户端
        
        Args:
            api_key: API密钥
            model: 使用的模型
            base_url: API基础URL
            temperature: 温度参数
        """
        try:
            import aiohttp
        except ImportError:
            raise ImportError("请安装HTTP客户端: pip install aiohttp")
        
        self.api_key = api_key or os.getenv("QWEN_API_KEY")
        self.model = model
        self.base_url = base_url or os.getenv("QWEN_BASE_URL", "https://dashscope.aliyuncs.com/api/v1")
        self.temperature = temperature  # 存储 temperature 参数
        
        if not self.api_key:
            raise ValueError("必须提供Qwen API密钥，请设置环境变量QWEN_API_KEY或传入api_key参数")
        
        if not self.base_url:
            raise ValueError("必须提供Qwen API地址，请设置环境变量QWEN_BASE_URL或传入base_url参数")
    
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        import aiohttp
        
        url = f"{self.base_url.rstrip('/')}/chat/completions"
        
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json"
        }
        
        payload = {
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "你是一个专业的Rust程序员，专门生成安全、高质量的Rust函数。只返回Rust代码，用```rust代码块包围。"
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.3,
            "max_tokens": 1500,
            "top_p": 0.9
        }
        
        try:
            timeout = aiohttp.ClientTimeout(total=60)
            async with aiohttp.ClientSession(timeout=timeout) as session:
                async with session.post(url, json=payload, headers=headers) as response:
                    if response.status == 200:
                        data = await response.json()
                        return data["choices"][0]["message"]["content"]
                    else:
                        error_text = await response.text()
                        raise RuntimeError(f"Qwen API返回错误状态: {response.status}, {error_text}")
                        
        except aiohttp.ClientError as e:
            raise RuntimeError(f"无法连接到Qwen API: {e}")
        except Exception as e:
            raise RuntimeError(f"Qwen API调用失败: {e}")

class CustomAPIClient(LLMClient):
    """自定义API客户端"""
    
    def __init__(self, 
                 api_key: str,
                 base_url: str,
                 model: str,
                 temperature: float = 0.3,
                 max_tokens: int = 1500,
                 **kwargs):
        """
        初始化自定义API客户端
        
        Args:
            api_key: API密钥
            base_url: API基础URL
            model: 模型名称
            temperature: 温度参数
            max_tokens: 最大令牌数
            **kwargs: 其他参数
        """
        try:
            import aiohttp
        except ImportError:
            raise ImportError("请安装HTTP客户端: pip install aiohttp")
        
        if not api_key:
            raise ValueError("自定义API必须提供API密钥")
        if not base_url:
            raise ValueError("自定义API必须提供基础URL")
        if not model:
            raise ValueError("自定义API必须提供模型名称")
        
        self.api_key = api_key
        self.base_url = base_url.rstrip('/')
        self.model = model
        self.temperature = temperature
        self.max_tokens = max_tokens
        self.extra_params = kwargs
    
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        import aiohttp
        
        # 支持多种API格式
        url = f"{self.base_url}/chat/completions"
        
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json"
        }
        
        payload = {
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "你是一个专业的Rust程序员，专门生成安全、高质量的Rust函数。只返回Rust代码，用```rust代码块包围。"
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
            **self.extra_params
        }
        
        try:
            timeout = aiohttp.ClientTimeout(total=60)
            async with aiohttp.ClientSession(timeout=timeout) as session:
                async with session.post(url, json=payload, headers=headers) as response:
                    if response.status == 200:
                        data = await response.json()
                        # 兼容不同的响应格式
                        if "choices" in data and len(data["choices"]) > 0:
                            return data["choices"][0]["message"]["content"]
                        elif "response" in data:
                            return data["response"]
                        else:
                            raise RuntimeError(f"无法解析API响应格式: {data}")
                    else:
                        error_text = await response.text()
                        raise RuntimeError(f"自定义API返回错误状态: {response.status}, {error_text}")
                        
        except aiohttp.ClientError as e:
            raise RuntimeError(f"无法连接到自定义API {self.base_url}: {e}")
        except Exception as e:
            raise RuntimeError(f"自定义API调用失败: {e}")

class LocalLLMClient(LLMClient):
    """本地LLM客户端（如Ollama）"""
    
    def __init__(self, 
                 base_url: str = "http://localhost:11434",
                 model: str = "codellama:7b"):
        """
        初始化本地LLM客户端
        """
        try:
            import aiohttp
        except ImportError:
            raise ImportError("请安装HTTP客户端: pip install aiohttp")
        
        if not base_url:
            raise ValueError("必须提供本地LLM服务地址")
        
        self.base_url = base_url.rstrip('/')
        self.model = model
    
    async def generate(self, prompt: str) -> str:
        """生成代码"""
        import aiohttp
        
        url = f"{self.base_url}/api/generate"
        payload = {
            "model": self.model,
            "prompt": f"你是一个Rust专家。{prompt}\n\n请只返回Rust代码，用```rust代码块包围：",
            "stream": False,
            "options": {
                "temperature": 0.7,
                "top_p": 0.9,
                "num_predict": 1000
            }
        }
        
        try:
            timeout = aiohttp.ClientTimeout(total=60)  # 60秒超时
            async with aiohttp.ClientSession(timeout=timeout) as session:
                async with session.post(url, json=payload) as response:
                    if response.status == 200:
                        data = await response.json()
                        return data.get("response", "")
                    else:
                        raise RuntimeError(f"本地LLM API返回错误状态: {response.status}")
        except aiohttp.ClientError as e:
            raise RuntimeError(f"无法连接到本地LLM服务 {self.base_url}: {e}")
        except Exception as e:
            raise RuntimeError(f"本地LLM调用失败: {e}")

def create_llm_client(llm_type: str, **kwargs) -> LLMClient:
    """
    创建LLM客户端工厂函数
    
    Args:
        llm_type: LLM类型 ("openai", "claude", "qwen", "custom", "local")
        **kwargs: LLM客户端的参数
        
    Returns:
        LLMClient: LLM客户端实例
        
    Raises:
        ValueError: 不支持的LLM类型
    """
    if llm_type == "openai":
        return OpenAIClient(**kwargs)
    elif llm_type == "claude":
        return ClaudeClient(**kwargs)
    elif llm_type == "qwen":
        return QwenClient(**kwargs)
    elif llm_type == "custom":
        return CustomAPIClient(**kwargs)
    elif llm_type == "local":
        return LocalLLMClient(**kwargs)
    else:
        supported_types = ["openai", "claude", "qwen", "custom", "local"]
        raise ValueError(f"不支持的LLM类型: {llm_type}。支持的类型: {', '.join(supported_types)}")