#!/usr/bin/env python3
"""
LLM模型配置管理工具
用于交互式选择和配置LLM模型
"""

import os
import json
from pathlib import Path
from typing import Dict, List, Optional
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.prompt import Prompt, Confirm

from rustlego.llm_config import (
    LLMConfig, 
    create_preset_config, 
    list_available_models,
    PRESET_MODELS,
    generate_config_template
)

console = Console()

class ModelConfigManager:
    """模型配置管理器"""
    
    def __init__(self):
        self.console = Console()
        self.config_file = Path("llm_config.json")
    
    def show_available_models(self):
        """显示所有可用模型"""
        models = list_available_models()
        
        table = Table(title="🤖 可用LLM模型")
        table.add_column("LLM类型", style="cyan")
        table.add_column("模型/预设", style="magenta") 
        table.add_column("说明", style="green")
        
        for llm_type, model_list in models.items():
            for i, model in enumerate(model_list):
                # 获取模型配置详情
                config = PRESET_MODELS[llm_type][model]
                
                if llm_type == "openai":
                    icon = "🤖"
                    if "gpt-4" in config['model']:
                        desc = "最强推理能力，适合复杂代码"
                    elif "turbo" in config['model']:
                        desc = "速度快，成本低"
                    elif model == "code-optimized":
                        desc = "代码生成优化，低温度参数"
                    else:
                        desc = "通用模型"
                elif llm_type == "claude":
                    icon = "🧠"
                    if "opus" in model:
                        desc = "最强性能，推理卓越"
                    elif "sonnet" in model:
                        desc = "平衡性能和速度"
                    elif "haiku" in model:
                        desc = "轻量快速"
                    else:
                        desc = "通用模型"
                elif llm_type == "qwen":
                    icon = "🚀"
                    if "coder" in model:
                        desc = "代码生成专用模型"
                    elif "max" in model:
                        desc = "最强版本，推理能力卓越"
                    elif "plus" in model:
                        desc = "高性能版本"
                    elif "turbo" in model:
                        desc = "快速响应版本"
                    else:
                        desc = "通用模型"
                elif llm_type == "custom":
                    icon = "🔧"
                    desc = "自定义API服务"
                else:  # local
                    icon = "🏠"
                    if "codellama" in model:
                        desc = "Meta代码专用模型"
                    elif "deepseek" in model:
                        desc = "DeepSeek代码模型"
                    elif "starcoder" in model:
                        desc = "BigCode星级编码器"
                    else:
                        desc = "本地模型"
                
                if i == 0:
                    table.add_row(f"{icon} {llm_type.upper()}", model, desc)
                else:
                    table.add_row("", model, desc)
        
        console.print(table)
    
    def create_interactive_config(self) -> Optional[LLMConfig]:
        """交互式创建配置"""
        console.print(Panel.fit(
            "[bold blue]🎯 LLM模型配置向导[/bold blue]\n"
            "帮助您选择最适合的模型和参数",
            border_style="blue"
        ))
        
        # 1. 选择LLM类型
        console.print("\n[bold]步骤1: 选择LLM类型[/bold]")
        console.print("1. [cyan]OpenAI[/cyan] - 强大但需要API密钥")
        console.print("2. [magenta]Claude[/magenta] - Anthropic的智能助手")
        console.print("3. [yellow]Qwen[/yellow] - 阿里云通义千问，支持中文")
        console.print("4. [blue]Custom[/blue] - 自定义API服务")
        console.print("5. [green]Local[/green] - 本地模型，免费但需要硬件")
        
        type_choice = Prompt.ask(
            "请选择", 
            choices=["1", "2", "3", "4", "5"],
            default="3"  # 默认选择Qwen
        )
        
        llm_type_map = {
            "1": "openai", 
            "2": "claude", 
            "3": "qwen", 
            "4": "custom", 
            "5": "local"
        }
        llm_type = llm_type_map[type_choice]
        
        # 2. 选择模型
        console.print(f"\n[bold]步骤2: 选择{llm_type.upper()}模型[/bold]")
        
        if llm_type == "custom":
            # 自定义API配置
            console.print("自定义API需要手动配置以下参数:")
            
            base_url = Prompt.ask("API地址 (如: https://api.your-service.com/v1)")
            model = Prompt.ask("模型名称 (如: your-model-name)")
            api_key = Prompt.ask("API密钥", password=True)
            
            config_params = {
                "model": model,
                "api_key": api_key,
                "base_url": base_url,
                "temperature": 0.3,
                "max_tokens": 1500
            }
            
        else:
            available = list(PRESET_MODELS[llm_type].keys())
            for i, model in enumerate(available, 1):
                config = PRESET_MODELS[llm_type][model]
                temp = config.get('temperature', 0.3)
                
                if llm_type == "openai":
                    if model == "code-optimized":
                        desc = f"🎯 代码优化 (温度: {temp}, 专门用于代码生成)"
                    elif "gpt-4" in config['model']:
                        desc = f"💎 {config['model']} (温度: {temp}, 最强性能)"
                    else:
                        desc = f"⚡ {config['model']} (温度: {temp}, 速度快)"
                elif llm_type == "claude":
                    desc = f"🧠 {config['model']} (温度: {temp})"
                elif llm_type == "qwen":
                    if model == "qwen-coder":
                        desc = f"🎯 {config['model']} (温度: {temp}, 代码生成专用)"
                    elif "max" in model:
                        desc = f"💎 {config['model']} (温度: {temp}, 最强版本)"
                    elif "plus" in model:
                        desc = f"⚡ {config['model']} (温度: {temp}, 高性能)"
                    else:
                        desc = f"🚀 {config['model']} (温度: {temp}, 快速响应)"
                else:
                    desc = f"🏠 {config['model']} (温度: {temp}, 本地运行)"
                
                console.print(f"{i}. [cyan]{model}[/cyan] - {desc}")
            
            model_choice = Prompt.ask(
                "请选择模型",
                choices=[str(i) for i in range(1, len(available) + 1)],
                default="1"
            )
            
            model = available[int(model_choice) - 1]
            config_params = PRESET_MODELS[llm_type][model].copy()
        
        # 3. 高级配置
        console.print(f"\n[bold]步骤3: 高级参数配置[/bold]")
        
        preset_config = PRESET_MODELS[llm_type][model]
        
        use_advanced = Confirm.ask("是否要自定义高级参数？", default=False)
        
        config_params = preset_config.copy()
        
        if use_advanced:
            if llm_type in ["openai", "claude"]:
                # 温度设置
                temp = Prompt.ask(
                    f"温度 (当前: {config_params.get('temperature', 0.3)}, 范围: 0.0-2.0)",
                    default=str(config_params.get('temperature', 0.3))
                )
                try:
                    config_params['temperature'] = float(temp)
                except ValueError:
                    pass
                
                # 最大令牌数
                max_tokens = Prompt.ask(
                    f"最大令牌数 (当前: {config_params.get('max_tokens', 1000)})",
                    default=str(config_params.get('max_tokens', 1000))
                )
                try:
                    config_params['max_tokens'] = int(max_tokens)
                except ValueError:
                    pass
            
            elif llm_type == "local":
                # 本地LLM参数
                base_url = Prompt.ask(
                    "服务地址",
                    default="http://localhost:11434"
                )
                config_params['base_url'] = base_url
        
        # 4. API密钥配置
        if llm_type in ["openai", "claude", "qwen"] and llm_type != "custom":
            if llm_type == "openai":
                env_var = "OPENAI_API_KEY"
            elif llm_type == "claude":
                env_var = "ANTHROPIC_API_KEY"
            else:  # qwen
                env_var = "QWEN_API_KEY"
            
            if os.getenv(env_var):
                console.print(f"✅ 检测到环境变量 {env_var}")
                config_params['api_key'] = None  # 使用环境变量
            else:
                console.print(f"⚠️  未检测到环境变量 {env_var}")
                
                input_key = Confirm.ask("是否要现在输入API密钥？", default=False)
                if input_key:
                    api_key = Prompt.ask("请输入API密钥", password=True)
                    config_params['api_key'] = api_key
                else:
                    console.print("💡 稍后请设置环境变量或使用 setup_llm.py")
                    config_params['api_key'] = None
            
            # Qwen还需要配置base_url
            if llm_type == "qwen":
                qwen_url = os.getenv("QWEN_BASE_URL")
                if qwen_url:
                    console.print(f"✅ 检测到Qwen API地址: {qwen_url}")
                    config_params['base_url'] = qwen_url
                else:
                    base_url = Prompt.ask(
                        "Qwen API地址",
                        default="https://dashscope.aliyuncs.com/api/v1"
                    )
                    config_params['base_url'] = base_url
        
        # 创建配置
        try:
            config = LLMConfig(llm_type=llm_type, **config_params)
            
            # 显示配置摘要
            self.show_config_summary(config)
            
            if Confirm.ask("保存此配置？", default=True):
                self.save_config(config)
                return config
            
        except Exception as e:
            console.print(f"❌ 配置创建失败: {e}")
            return None
    
    def show_config_summary(self, config: LLMConfig):
        """显示配置摘要"""
        summary = f"""[bold green]📋 配置摘要[/bold green]

🤖 LLM类型: {config.llm_type.upper()}
🎯 模型: {config.model}
🌡️  温度: {config.model_params.get('temperature', 'N/A')}"""

        if config.llm_type in ["openai", "claude", "qwen", "custom"]:
            summary += f"\n🔑 API密钥: {'已设置' if config.api_key else '使用环境变量'}"
        
        if config.llm_type in ["qwen", "custom", "local"]:
            summary += f"\n🌐 服务地址: {config.base_url}"
        
        if config.model_params.get('max_tokens'):
            summary += f"\n📏 最大令牌: {config.model_params['max_tokens']}"
        
        console.print(Panel(summary, border_style="green"))
    
    def save_config(self, config: LLMConfig):
        """保存配置到文件"""
        try:
            with open(self.config_file, 'w') as f:
                json.dump(config.to_dict(), f, indent=2, ensure_ascii=False)
            
            console.print(f"✅ 配置已保存到 {self.config_file}")
            
        except Exception as e:
            console.print(f"❌ 保存配置失败: {e}")
    
    def load_config(self) -> Optional[LLMConfig]:
        """从文件加载配置"""
        if not self.config_file.exists():
            return None
        
        try:
            with open(self.config_file, 'r') as f:
                data = json.load(f)
            
            return LLMConfig(**data)
            
        except Exception as e:
            console.print(f"❌ 加载配置失败: {e}")
            return None

def main():
    """主函数"""
    manager = ModelConfigManager()
    
    console.print("🚀 RustLego 模型配置工具")
    console.print("=" * 50)
    
    # 显示可用模型
    manager.show_available_models()
    
    console.print("\n")
    
    # 检查现有配置
    existing_config = manager.load_config()
    if existing_config:
        console.print("📋 发现现有配置:")
        manager.show_config_summary(existing_config)
        
        if not Confirm.ask("创建新配置？", default=False):
            return
    
    # 创建新配置
    new_config = manager.create_interactive_config()
    
    if new_config:
        console.print("\n🎉 配置完成！现在可以使用以下方式:")
        console.print("```python")
        console.print("from rustlego.generator import FunctionGenerator")
        console.print("from rustlego.llm_config import LLMConfig")
        console.print("")
        console.print("# 加载配置")
        console.print("config = LLMConfig.from_file('llm_config.json')")
        console.print("generator = FunctionGenerator(llm_config=config)")
        console.print("")
        console.print("# 或直接使用预设")
        console.print(f"generator = FunctionGenerator(llm_type='{new_config.llm_type}', model='{new_config.model}')")
        console.print("```")

if __name__ == "__main__":
    main()