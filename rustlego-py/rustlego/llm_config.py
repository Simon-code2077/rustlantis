#!/usr/bin/env python3
"""
改进的LLM客户端配置
支持更灵活的模型和参数配置
"""

import os
import json
from typing import Optional, Dict, Any
from abc import ABC, abstractmethod

class LLMConfig:
    """LLM配置类"""
    
    def __init__(self, 
                 llm_type: str,
                 model: str,
                 api_key: Optional[str] = None,
                 base_url: Optional[str] = None,
                 **model_params):
        """
        初始化LLM配置
        
        Args:
            llm_type: LLM类型
            model: 模型名称
            api_key: API密钥
            base_url: API基础URL
            **model_params: 模型特定参数
        """
        self.llm_type = llm_type
        self.model = model
        self.api_key = api_key
        self.base_url = base_url
        self.model_params = model_params
        
        # 验证配置
        self._validate()
    
    def _validate(self):
        """验证配置有效性"""
        if self.llm_type == "openai":
            if not (self.api_key or os.getenv("OPENAI_API_KEY")):
                raise ValueError("OpenAI需要API密钥")
        elif self.llm_type == "claude":
            if not (self.api_key or os.getenv("ANTHROPIC_API_KEY")):
                raise ValueError("Claude需要API密钥")
        elif self.llm_type == "local":
            if not self.base_url:
                self.base_url = "http://localhost:11434"
    
    @classmethod
    def from_file(cls, config_path: str) -> 'LLMConfig':
        """从配置文件加载"""
        with open(config_path, 'r') as f:
            config = json.load(f)
        return cls(**config)
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return {
            "llm_type": self.llm_type,
            "model": self.model,
            "api_key": self.api_key,
            "base_url": self.base_url,
            **self.model_params
        }

# 预定义的模型配置
PRESET_MODELS = {
    "openai": {
        "gpt-4": {
            "model": "gpt-4",
            "temperature": 0.2,
            "max_tokens": 2000,
            "top_p": 0.9
        },
        "gpt-4-turbo": {
            "model": "gpt-4-turbo-preview",
            "temperature": 0.3,
            "max_tokens": 2000,
            "top_p": 0.95
        },
        "gpt-3.5-turbo": {
            "model": "gpt-3.5-turbo",
            "temperature": 0.4,
            "max_tokens": 1500,
            "top_p": 0.9
        },
        "code-optimized": {
            "model": "gpt-4",
            "temperature": 0.1,  # 更确定性的代码生成
            "max_tokens": 2000,
            "top_p": 0.8,
            "frequency_penalty": 0.1
        }
    },
    "claude": {
        "claude-3-opus": {
            "model": "claude-3-opus-20240229",
            "temperature": 0.3,
            "max_tokens": 2000
        },
        "claude-3-sonnet": {
            "model": "claude-3-sonnet-20240229", 
            "temperature": 0.4,
            "max_tokens": 1500
        },
        "claude-3-haiku": {
            "model": "claude-3-haiku-20240307",
            "temperature": 0.5,
            "max_tokens": 1000
        }
    },
    "qwen": {
        "qwen-turbo": {
            "model": "qwen-turbo",
            "temperature": 0.3,
            "max_tokens": 1500,
            "top_p": 0.9
        },
        "qwen-plus": {
            "model": "qwen-plus",
            "temperature": 0.2,
            "max_tokens": 2000,
            "top_p": 0.85
        },
        "qwen-max": {
            "model": "qwen-max",
            "temperature": 0.1,
            "max_tokens": 2000,
            "top_p": 0.8
        },
        "qwen-coder": {
            "model": "qwen-coder-turbo",
            "temperature": 0.1,  # 代码生成专用，低温度
            "max_tokens": 2000,
            "top_p": 0.8
        }
    },
    "custom": {
        "custom-api": {
            "model": "custom-model",
            "temperature": 0.3,
            "max_tokens": 1500,
            "top_p": 0.9
        }
    },
    "local": {
        "codellama-7b": {
            "model": "codellama:7b",
            "temperature": 0.2,
            "num_predict": 1000,
            "top_p": 0.9
        },
        "codellama-13b": {
            "model": "codellama:13b", 
            "temperature": 0.15,
            "num_predict": 1500,
            "top_p": 0.85
        },
        "deepseek-coder": {
            "model": "deepseek-coder:6.7b",
            "temperature": 0.1,
            "num_predict": 1200,
            "top_p": 0.8
        },
        "starcoder": {
            "model": "starcoder:7b",
            "temperature": 0.2,
            "num_predict": 1000,
            "top_p": 0.9
        }
    }
}

def create_preset_config(llm_type: str, preset_name: str, **overrides) -> LLMConfig:
    """
    创建预设配置
    
    Args:
        llm_type: LLM类型
        preset_name: 预设名称
        **overrides: 覆盖参数
        
    Returns:
        LLMConfig: 配置实例
    """
    if llm_type not in PRESET_MODELS:
        raise ValueError(f"不支持的LLM类型: {llm_type}")
    
    if preset_name not in PRESET_MODELS[llm_type]:
        raise ValueError(f"不支持的预设: {preset_name} for {llm_type}")
    
    preset = PRESET_MODELS[llm_type][preset_name].copy()
    preset.update(overrides)
    
    return LLMConfig(llm_type=llm_type, **preset)

def list_available_models() -> Dict[str, list]:
    """列出所有可用的模型预设"""
    return {
        llm_type: list(models.keys()) 
        for llm_type, models in PRESET_MODELS.items()
    }

def generate_config_template() -> Dict[str, Any]:
    """生成配置文件模板"""
    return {
        "llm_type": "openai",
        "model": "gpt-3.5-turbo", 
        "api_key": "your-api-key-here",
        "base_url": None,
        "temperature": 0.3,
        "max_tokens": 1500,
        "top_p": 0.9,
        "description": "这是一个LLM配置模板，请根据需要修改参数"
    }

# 使用示例
if __name__ == "__main__":
    # 显示可用模型
    print("可用模型:")
    for llm_type, models in list_available_models().items():
        print(f"{llm_type}: {', '.join(models)}")
    
    # 创建预设配置 (仅用于展示，不验证API密钥)
    try:
        config = create_preset_config("local", "codellama-7b")
        print(f"\n本地模型配置示例: {config.to_dict()}")
    except Exception as e:
        print(f"\n配置示例跳过: {e}")
    
    # 生成配置模板
    template = generate_config_template()
    print(f"\n配置文件模板:")
    print(json.dumps(template, indent=2, ensure_ascii=False))