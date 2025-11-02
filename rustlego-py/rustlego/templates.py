#!/usr/bin/env python3
"""
模板系统

提供Rust函数生成的模板和约束系统。
"""

from dataclasses import dataclass
from typing import List, Dict, Optional
from enum import Enum

class FunctionCategory(Enum):
    """函数类别"""
    ARITHMETIC = "arithmetic"
    MEMORY = "memory" 
    CONTROL_FLOW = "control_flow"
    TYPE_CONVERSION = "type_conversion"
    STRING_OPS = "string_ops"

@dataclass
class FunctionTemplate:
    """函数模板"""
    category: str
    name: str
    description: str
    constraints: List[str]
    example_signature: str
    return_type: str
    parameters: List[str]
    complexity: int
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "category": self.category,
            "name": self.name, 
            "description": self.description,
            "constraints": self.constraints,
            "example_signature": self.example_signature,
            "return_type": self.return_type,
            "parameters": self.parameters,
            "complexity": self.complexity
        }

class TemplateLibrary:
    """模板库"""
    
    def __init__(self):
        self.templates: Dict[str, List[FunctionTemplate]] = {}
        self._initialize_templates()
    
    def _initialize_templates(self):
        """初始化模板"""
        
        # 算术模板
        self._add_template("arithmetic", FunctionTemplate(
            category="arithmetic",
            name="add_numbers",
            description="安全地添加两个数字并处理溢出",
            constraints=["必须处理溢出", "返回Result类型"],
            example_signature="fn add_numbers(a: i32, b: i32) -> Result<i32, String>",
            return_type="Result<i32, String>",
            parameters=["i32", "i32"],
            complexity=2
        ))
        
        self._add_template("arithmetic", FunctionTemplate(
            category="arithmetic", 
            name="multiply_safe",
            description="安全的乘法运算",
            constraints=["防止溢出", "处理零值"],
            example_signature="fn multiply_safe(a: i32, b: i32) -> Option<i32>",
            return_type="Option<i32>",
            parameters=["i32", "i32"],
            complexity=3
        ))
        
        self._add_template("arithmetic", FunctionTemplate(
            category="arithmetic",
            name="divide_checked",
            description="检查除法运算",
            constraints=["检查除零", "返回Result"],
            example_signature="fn divide_checked(a: i32, b: i32) -> Result<i32, &'static str>",
            return_type="Result<i32, &'static str>",
            parameters=["i32", "i32"],
            complexity=2
        ))
        
        # 内存操作模板
        self._add_template("memory", FunctionTemplate(
            category="memory",
            name="process_slice",
            description="处理切片数据并进行边界检查",
            constraints=["必须验证边界", "处理空切片"],
            example_signature="fn process_slice(data: &mut [u8], start: usize, end: usize) -> bool",
            return_type="bool",
            parameters=["&mut [u8]", "usize", "usize"],
            complexity=4
        ))
        
        self._add_template("memory", FunctionTemplate(
            category="memory",
            name="safe_index",
            description="安全的数组索引访问",
            constraints=["边界检查", "返回Option"],
            example_signature="fn safe_index<T>(slice: &[T], index: usize) -> Option<&T>",
            return_type="Option<&T>",
            parameters=["&[T]", "usize"],
            complexity=2
        ))
        
        # 控制流模板
        self._add_template("control_flow", FunctionTemplate(
            category="control_flow",
            name="conditional_process",
            description="基于复杂条件处理输入",
            constraints=["多条件分支", "早期返回"],
            example_signature="fn conditional_process(input: Option<i32>) -> Result<String, &'static str>",
            return_type="Result<String, &'static str>",
            parameters=["Option<i32>"],
            complexity=5
        ))
        
        # 类型转换模板
        self._add_template("type_conversion", FunctionTemplate(
            category="type_conversion",
            name="safe_convert",
            description="安全的数值类型转换",
            constraints=["处理转换错误", "无数据丢失"],
            example_signature="fn safe_convert(input: f64) -> Result<i32, String>",
            return_type="Result<i32, String>",
            parameters=["f64"],
            complexity=3
        ))
        
        # 字符串操作模板
        self._add_template("string_ops", FunctionTemplate(
            category="string_ops",
            name="format_and_validate", 
            description="格式化字符串并验证",
            constraints=["验证输入字符串", "处理Unicode"],
            example_signature="fn format_and_validate(template: &str, args: &[String]) -> Result<String, &'static str>",
            return_type="Result<String, &'static str>",
            parameters=["&str", "&[String]"],
            complexity=3
        ))
    
    def _add_template(self, category: str, template: FunctionTemplate):
        """添加模板"""
        if category not in self.templates:
            self.templates[category] = []
        self.templates[category].append(template)
    
    def get_templates(self, category: str) -> List[FunctionTemplate]:
        """获取指定类别的模板"""
        return self.templates.get(category, [])
    
    def get_all_templates(self) -> Dict[str, List[FunctionTemplate]]:
        """获取所有模板"""
        return self.templates.copy()
    
    def get_template_count(self, category: str) -> int:
        """获取指定类别的模板数量"""
        return len(self.templates.get(category, []))
    
    def get_categories(self) -> List[str]:
        """获取所有类别"""
        return list(self.templates.keys())

class PromptGenerator:
    """Prompt生成器"""
    
    def __init__(self):
        pass
    
    def generate_basic_prompt(self, template: FunctionTemplate) -> str:
        """生成基础prompt"""
        return f"""生成一个Rust函数，遵循以下规范:

函数类别: {template.category}
函数名称: {template.name}
描述: {template.description}
示例签名: {template.example_signature}
返回类型: {template.return_type}
参数: {', '.join(template.parameters)}
约束: {', '.join(template.constraints)}

要求:
1. 只编写函数实现，不要额外的解释
2. 遵循Rust最佳实践和习惯
3. 在适当的地方包含错误处理
4. 确保函数能够编译且安全
5. 为复杂逻辑添加内联注释
6. 使用适当的Rust类型和模式
7. 如果返回类型是Result<T, E>，确保正确处理Ok/Err分支
8. 不要在函数中重复定义同名函数

请生成完整的函数实现:"""
    
    def generate_constrained_prompt(self, template: FunctionTemplate, constraints: List[str]) -> str:
        """生成带额外约束的prompt"""
        basic_prompt = self.generate_basic_prompt(template)
        
        additional_constraints = "\n附加约束:\n"
        for i, constraint in enumerate(constraints, 1):
            additional_constraints += f"{i}. {constraint}\n"
        
        return basic_prompt + additional_constraints
    
    def generate_chaining_prompt(self, prev_return_type: str, next_param_types: List[str]) -> str:
        """生成用于链式调用的prompt
        
        确保生成的代码能够正确处理Result类型的转换
        
        Args:
            prev_return_type: 前一个函数的返回类型
            next_param_types: 下一个函数的参数类型列表
        """
        type_handling = self._generate_type_handling_guide(prev_return_type, next_param_types)
        
        return f"""当生成能够链式调用的Rust函数时，请遵循以下类型处理规则:

前一个函数返回类型: {prev_return_type}
下一个函数需要的参数类型: {', '.join(next_param_types)}

{type_handling}

重要提示:
1. 如果前一个函数返回 Result<T, E>，在传递给下一个函数前必须解包
2. 在实际的main函数中，使用match表达式处理Result类型
3. 不要生成无法编译的代码
4. 确保类型之间的兼容性"""
    
    def _generate_type_handling_guide(self, return_type: str, param_types: List[str]) -> str:
        """生成类型处理指南"""
        guide = "类型处理指南:\n"
        
        if "Result<" in return_type:
            inner_type = return_type.split("Result<")[1].split(",")[0]
            guide += f"✓ 返回类型 {return_type} 应该通过match解包以获得 {inner_type}\n"
            
            for param_type in param_types:
                if param_type == inner_type:
                    guide += f"✓ {inner_type} 可以直接用作下一个函数的参数\n"
                elif "i32" in param_type and "i32" in inner_type:
                    guide += f"✓ {inner_type} 与 {param_type} 兼容\n"
        elif "Option<" in return_type:
            inner_type = return_type.split("Option<")[1].rstrip(">")
            guide += f"✓ 返回类型 {return_type} 应该通过match/unwrap_or解包以获得 {inner_type}\n"
        
        return guide


if __name__ == "__main__":
    # 测试模板系统
    from rich.console import Console
    from rich.table import Table
    
    console = Console()
    library = TemplateLibrary()
    prompt_gen = PromptGenerator()
    
    # 显示所有类别和模板
    table = Table(title="函数模板库")
    table.add_column("类别", style="cyan")
    table.add_column("模板数量", style="magenta")
    table.add_column("模板名称", style="green")
    
    for category in library.get_categories():
        templates = library.get_templates(category)
        template_names = [t.name for t in templates]
        table.add_row(
            category,
            str(len(templates)),
            ", ".join(template_names)
        )
    
    console.print(table)
    
    # 生成示例prompt
    console.print("\n[bold blue]示例Prompt (arithmetic/add_numbers):[/bold blue]")
    templates = library.get_templates("arithmetic")
    if templates:
        prompt = prompt_gen.generate_basic_prompt(templates[0])
        console.print(prompt)