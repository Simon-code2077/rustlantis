#!/usr/bin/env python3
"""
函数生成器

使用LLM生成Rust函数，支持编译验证和批量生成。
"""

import re
import time
import json
import os
import random
from pathlib import Path
from typing import List, Optional, Dict, Any
from dataclasses import dataclass, asdict
from rich.console import Console
from rich.progress import Progress, TaskID

try:
    from .validator import RustValidator, ValidationResult
    from .templates import TemplateLibrary, FunctionTemplate, PromptGenerator
    from .llm_clients import LLMClient, create_llm_client
except ImportError:
    # 处理直接运行时的导入
    import sys
    from pathlib import Path
    sys.path.append(str(Path(__file__).parent.parent))
    from rustlego.validator import RustValidator, ValidationResult
    from rustlego.templates import TemplateLibrary, FunctionTemplate, PromptGenerator
    from rustlego.llm_clients import LLMClient, create_llm_client

console = Console()

@dataclass
class GeneratedFunction:
    """生成的函数"""
    name: str
    code: str
    category: str
    complexity: int
    template_used: Optional[str] = None
    validation_result: Optional[ValidationResult] = None
    generation_time_ms: float = 0.0
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        result = asdict(self)
        if self.validation_result:
            result['validation_result'] = asdict(self.validation_result)
        return result
    
    def is_valid(self) -> bool:
        """检查函数是否有效"""
        if self.validation_result is None:
            return True  # 如果没有验证，认为有效
        return self.validation_result.is_valid

class FunctionGenerator:
    """函数生成器"""
    
    def __init__(self, 
                 enable_validation: bool = True,
                 config_file: Optional[str] = None,
                 llm_client: Optional[LLMClient] = None):
        """
        初始化函数生成器
        
        Args:
            enable_validation: 是否启用验证
            config_file: LLM配置文件路径，默认为 "llm_config.json"
            llm_client: 直接提供的LLM客户端实例（用于高级用法）
            
        Examples:
            # 使用默认配置文件
            generator = FunctionGenerator()
            
            # 使用指定配置文件
            generator = FunctionGenerator(config_file="custom_config.json")
            
            # 直接传入客户端（高级用法）
            generator = FunctionGenerator(llm_client=my_client)
            
        Raises:
            ValueError: 当配置文件不存在或格式错误时
        """
        self.template_library = TemplateLibrary()
        self.prompt_generator = PromptGenerator()
        self.enable_validation = enable_validation
        
        if enable_validation:
            self.validator = RustValidator()
        else:
            self.validator = None
        
        # 配置LLM客户端
        if llm_client:
            # 直接使用提供的客户端
            self.llm_client = llm_client
        else:
            # 从配置文件加载
            if config_file is None:
                # 默认配置文件路径
                config_file = os.path.join(os.path.dirname(__file__), '..', 'llm_config.json')
            
            if not os.path.exists(config_file):
                raise ValueError(
                    f"配置文件不存在: {config_file}\n\n"     
                )
            
            try:
                # 读取配置文件
                with open(config_file, 'r', encoding='utf-8') as f:
                    config_data = json.load(f)
                
                console.print(f"[green]从配置文件加载LLM配置: {config_file}[/green]")
                console.print(f"[blue]LLM类型: {config_data.get('llm_type')}, 模型: {config_data.get('model')}[/blue]")
                
                # 提取基本参数
                llm_type = config_data.pop('llm_type')
                model = config_data.pop('model', None)
                api_key = config_data.pop('api_key', None)
                base_url = config_data.pop('base_url', None)
                
                # 其余参数作为模型参数
                model_params = config_data
                
                # 创建LLM客户端
                self.llm_client = create_llm_client(
                    llm_type=llm_type,
                    model=model,
                    api_key=api_key,
                    base_url=base_url,
                    **model_params
                )
                
            except json.JSONDecodeError as e:
                raise ValueError(f"配置文件格式错误: {e}")
            except Exception as e:
                raise ValueError(f"加载配置失败: {e}")
    
    async def generate_function(self, template: FunctionTemplate) -> GeneratedFunction:
        """
        生成单个函数
        
        Args:
            template: 函数模板
            
        Returns:
            GeneratedFunction: 生成的函数
        """
        start_time = time.time()
        
        # 生成prompt
        prompt = self.prompt_generator.generate_basic_prompt(template)
        
        console.print(f"正在生成函数: [cyan]{template.name}[/cyan]")
        
        # 调用LLM生成代码
        response = await self.llm_client.generate(prompt)
        function_code = self._extract_function_code(response)
        
        generation_time = (time.time() - start_time) * 1000
        
        # 验证生成的函数
        validation_result = None
        if self.enable_validation and self.validator:
            console.print("  验证中...", end=" ")
            validation_result = self.validator.validate_function(function_code)
            
            if validation_result.is_valid:
                console.print("[green] 有效[/green]")
            else:
                console.print("[red] 无效[/red]")
        
        return GeneratedFunction(
            name=template.name,
            code=function_code,
            category=template.category,
            complexity=template.complexity,
            template_used=template.name,
            validation_result=validation_result,
            generation_time_ms=generation_time
        )
    
    async def generate_batch(self, 
                           category: str, 
                           count: int,
                           max_attempts: Optional[int] = None) -> List[GeneratedFunction]:
        """
        批量生成函数
        
        Args:
            category: 函数类别
            count: 生成数量
            max_attempts: 最大尝试次数
            
        Returns:
            List[GeneratedFunction]: 生成的函数列表
        """
        templates = self.template_library.get_templates(category)
        if not templates:
            raise ValueError(f"没有找到类别 '{category}' 的模板")
        
        if max_attempts is None:
            max_attempts = count * 3  # 默认最多尝试3倍
        
        functions = []
        valid_functions = []
        attempts = 0
        
        console.print(f"[bold blue]开始批量生成 {count} 个 {category} 函数...[/bold blue]")
        
        with Progress() as progress:
            task = progress.add_task(f"生成 {category} 函数", total=count)
            
            while len(valid_functions) < count and attempts < max_attempts:
                template = random.choice(templates)
                attempts += 1
                
                try:
                    func = await self.generate_function(template)
                    functions.append(func)
                    
                    # 如果启用验证，只保留有效函数
                    if self.enable_validation:
                        if func.is_valid():
                            valid_functions.append(func)
                            progress.update(task, advance=1)
                    else:
                        valid_functions.append(func)
                        progress.update(task, advance=1)
                        
                except Exception as e:
                    console.print(f"[red]生成失败 (尝试 {attempts}): {e}[/red]")
        
        if self.enable_validation:
            success_rate = len(valid_functions) / len(functions) if functions else 0
            console.print(f"\n生成完成! 总计: {len(functions)}, 有效: {len(valid_functions)} "
                         f"(成功率: {success_rate:.1%})")
        else:
            console.print(f"\n生成完成! 总计: {len(valid_functions)} 个函数")
        
        return valid_functions
    
    async def generate_with_constraints(self,
                                      template: FunctionTemplate,
                                      constraints: List[str]) -> GeneratedFunction:
        """
        使用额外约束生成函数
        
        Args:
            template: 函数模板
            constraints: 额外约束
            
        Returns:
            GeneratedFunction: 生成的函数
        """
        start_time = time.time()
        
        # 生成带约束的prompt
        prompt = self.prompt_generator.generate_constrained_prompt(template, constraints)
        
        console.print(f"正在生成约束函数: [cyan]{template.name}[/cyan]")
        
        # 调用LLM生成代码
        response = await self.llm_client.generate(prompt)
        function_code = self._extract_function_code(response)
        
        generation_time = (time.time() - start_time) * 1000
        
        # 验证生成的函数
        validation_result = None
        if self.enable_validation and self.validator:
            validation_result = self.validator.validate_function(function_code)
        
        return GeneratedFunction(
            name=f"{template.name}_constrained",
            code=function_code,
            category=template.category,
            complexity=template.complexity + 1,
            template_used=template.name,
            validation_result=validation_result,
            generation_time_ms=generation_time
        )
    
    def _extract_function_code(self, response: str) -> str:
        """从LLM响应中提取函数代码"""
        # 尝试提取```rust代码块
        rust_block = re.search(r'```rust\s*(.*?)\s*```', response, re.DOTALL)
        if rust_block:
            return rust_block.group(1).strip()
        
        # 尝试提取```代码块
        generic_block = re.search(r'```\s*(.*?)\s*```', response, re.DOTALL)
        if generic_block:
            code = generic_block.group(1).strip()
            # 简单检查是否是Rust代码
            if 'fn ' in code or 'struct ' in code or 'impl ' in code:
                return code
        
        # 查找函数定义
        fn_match = re.search(r'(fn\s+\w+.*?\{.*?\})', response, re.DOTALL)
        if fn_match:
            return fn_match.group(1)
        
        # 如果都找不到，返回整个响应
        return response.strip()
    
    def save_functions(self, 
                      functions: List[GeneratedFunction], 
                      output_dir: Path):
        """
        保存生成的函数到文件
        
        Args:
            functions: 函数列表
            output_dir: 输出目录
        """
        output_dir.mkdir(parents=True, exist_ok=True)
        
        for i, func in enumerate(functions, 1):
            # 生成文件名
            filename = f"{i:03d}_{func.name}.rs"
            filepath = output_dir / filename
            
            # 生成文件内容
            validation_info = ""
            if func.validation_result:
                validation_info = f"""// 验证结果: {"✅ 有效" if func.is_valid() else "❌ 无效"}
// 编译状态: {"成功" if func.validation_result.compilation_success else "失败"}
// 验证时间: {func.validation_result.validation_time_ms:.2f}ms
// 警告数量: {len(func.validation_result.warnings)}
"""
            
            content = f"""// 生成的函数: {func.name}
// 类别: {func.category}
// 复杂度: {func.complexity}
// 模板: {func.template_used}
// 生成时间: {func.generation_time_ms:.2f}ms
{validation_info}
{func.code}
"""
            
            filepath.write_text(content, encoding='utf-8')
        
        console.print(f"[green]已保存 {len(functions)} 个函数到 {output_dir}[/green]")
    
    def save_functions_json(self,
                           functions: List[GeneratedFunction],
                           output_file: Path):
        """
        保存函数为JSON格式
        
        Args:
            functions: 函数列表
            output_file: 输出文件
        """
        data = [func.to_dict() for func in functions]
        
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        
        console.print(f"[green]已保存函数元数据到 {output_file}[/green]")


if __name__ == "__main__":
    import asyncio
    
    async def test_generator():
        """测试函数生成器"""
        try:
            console.print("[blue]使用配置文件初始化生成器...[/blue]")
            generator = FunctionGenerator(enable_validation=True)
            
            # 测试单个函数生成
            templates = generator.template_library.get_templates("arithmetic")
            if templates:
                console.print("[cyan]测试单个函数生成...[/cyan]")
                func = await generator.generate_function(templates[0])
                console.print(f"\n[bold green]生成的函数:[/bold green]\n{func.code}")
                console.print(f"有效性: {'✅' if func.is_valid() else '❌'}")
            
            # 测试批量生成
            console.print("\n[bold blue]测试批量生成:[/bold blue]")
            functions = await generator.generate_batch("arithmetic", 2)
            
            console.print(f"\n生成了 {len(functions)} 个函数:")
            for func in functions:
                status = "✅" if func.is_valid() else "❌"
                console.print(f"  {status} {func.name}")
                
        except Exception as e:
            console.print(f"[red]❌ 测试失败: {e}[/red]")
            console.print("\n请确保:")
            console.print("1. 配置文件 llm_config.json 存在")
            console.print("2. 配置文件格式正确")
            console.print("3. API密钥等信息已正确设置")
    
    # 运行测试
    asyncio.run(test_generator())