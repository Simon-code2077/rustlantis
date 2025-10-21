#!/usr/bin/env python3
"""
函数生成器

使用LLM生成Rust函数，支持编译验证和批量生成。
"""

import re
import time
import json
from pathlib import Path
from typing import List, Optional, Dict, Any
from dataclasses import dataclass, asdict
from rich.console import Console
from rich.progress import Progress, TaskID

try:
    from .validator import RustValidator, ValidationResult
    from .templates import TemplateLibrary, FunctionTemplate, PromptGenerator
except ImportError:
    # 处理直接运行时的导入
    import sys
    from pathlib import Path
    sys.path.append(str(Path(__file__).parent.parent))
    from rustlego.validator import RustValidator, ValidationResult
    from rustlego.templates import TemplateLibrary, FunctionTemplate, PromptGenerator

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

class MockLLMClient:
    """模拟LLM客户端（用于演示）"""
    
    def __init__(self):
        # 预定义的一些示例函数，用于演示
        self.example_functions = {
            "add_numbers": """fn add_numbers(a: i32, b: i32) -> Result<i32, String> {
    match a.checked_add(b) {
        Some(result) => Ok(result),
        None => Err("Integer overflow occurred".to_string()),
    }
}""",
            "multiply_safe": """fn multiply_safe(a: i32, b: i32) -> Option<i32> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    a.checked_mul(b)
}""",
            "divide_checked": """fn divide_checked(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}""",
            "process_slice": """fn process_slice(data: &mut [u8], start: usize, end: usize) -> bool {
    if start >= end || end > data.len() {
        return false;
    }
    
    for i in start..end {
        data[i] = data[i].wrapping_add(1);
    }
    
    true
}""",
            "safe_index": """fn safe_index<T>(slice: &[T], index: usize) -> Option<&T> {
    if index < slice.len() {
        Some(&slice[index])
    } else {
        None
    }
}""",
            "conditional_process": """fn conditional_process(input: Option<i32>) -> Result<String, &'static str> {
    match input {
        Some(value) if value > 0 => Ok(format!("Positive: {}", value)),
        Some(value) if value < 0 => Ok(format!("Negative: {}", value)),
        Some(0) => Ok("Zero".to_string()),
        None => Err("No input provided"),
    }
}""",
            "safe_convert": """fn safe_convert(input: f64) -> Result<i32, String> {
    if input.is_nan() || input.is_infinite() {
        return Err("Invalid floating point value".to_string());
    }
    
    if input > i32::MAX as f64 || input < i32::MIN as f64 {
        return Err("Value out of i32 range".to_string());
    }
    
    Ok(input as i32)
}""",
            "format_and_validate": """fn format_and_validate(template: &str, args: &[String]) -> Result<String, &'static str> {
    if template.is_empty() {
        return Err("Template cannot be empty");
    }
    
    let placeholder_count = template.matches("{}").count();
    if placeholder_count != args.len() {
        return Err("Argument count mismatch");
    }
    
    let mut result = template.to_string();
    for arg in args {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos+2, arg);
        }
    }
    
    Ok(result)
}"""
        }
    
    async def generate(self, prompt: str) -> str:
        """生成函数代码"""
        # 模拟生成延迟
        await self._simulate_delay()
        
        # 从prompt中提取函数名
        function_name = self._extract_function_name_from_prompt(prompt)
        
        # 返回对应的示例函数，或生成一个简单的函数
        if function_name in self.example_functions:
            return f"```rust\n{self.example_functions[function_name]}\n```"
        else:
            # 生成一个简单的默认函数
            return f"""```rust
fn {function_name}(x: i32) -> i32 {{
    x + 1
}}
```"""
    
    async def _simulate_delay(self):
        """模拟网络延迟"""
        import asyncio
        await asyncio.sleep(0.1)  # 100ms延迟
    
    def _extract_function_name_from_prompt(self, prompt: str) -> str:
        """从prompt中提取函数名"""
        # 查找"函数名称:"后的内容
        match = re.search(r'函数名称:\s*(\w+)', prompt)
        if match:
            return match.group(1)
        return "generated_function"

class FunctionGenerator:
    """函数生成器"""
    
    def __init__(self, 
                 enable_validation: bool = True,
                 llm_client: Optional[Any] = None):
        """
        初始化函数生成器
        
        Args:
            enable_validation: 是否启用验证
            llm_client: LLM客户端（如果为None则使用模拟客户端）
        """
        self.template_library = TemplateLibrary()
        self.prompt_generator = PromptGenerator()
        self.enable_validation = enable_validation
        
        if enable_validation:
            self.validator = RustValidator()
        else:
            self.validator = None
            
        # 使用提供的LLM客户端或模拟客户端
        self.llm_client = llm_client or MockLLMClient()
    
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
                console.print("[green]✅ 有效[/green]")
            else:
                console.print("[red]❌ 无效[/red]")
        
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
                template = templates[attempts % len(templates)]
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
        generator = FunctionGenerator(enable_validation=True)
        
        # 测试单个函数生成
        templates = generator.template_library.get_templates("arithmetic")
        if templates:
            func = await generator.generate_function(templates[0])
            console.print(f"\n[bold green]生成的函数:[/bold green]\n{func.code}")
            console.print(f"有效性: {'✅' if func.is_valid() else '❌'}")
        
        # 测试批量生成
        console.print("\n[bold blue]测试批量生成:[/bold blue]")
        functions = await generator.generate_batch("arithmetic", 3)
        
        console.print(f"\n生成了 {len(functions)} 个函数:")
        for func in functions:
            status = "✅" if func.is_valid() else "❌"
            console.print(f"  {status} {func.name}")
    
    # 运行测试
    asyncio.run(test_generator())