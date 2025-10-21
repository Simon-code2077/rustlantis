#!/usr/bin/env python3
"""
函数组合器

将多个生成的函数组合成复杂的程序。
"""

import re
import random
from typing import List, Dict, Optional, Set
from dataclasses import dataclass
from pathlib import Path
from rich.console import Console

try:
    from .validator import RustValidator, ValidationResult
    from .generator import GeneratedFunction
except ImportError:
    # 处理直接运行时的导入
    import sys
    sys.path.append(str(Path(__file__).parent.parent))
    from rustlego.validator import RustValidator, ValidationResult
    from rustlego.generator import GeneratedFunction

console = Console()

@dataclass
class ComposedProgram:
    """组合的程序"""
    name: str
    code: str
    functions: List[GeneratedFunction]
    complexity: int
    main_function: str
    validation_result: Optional[ValidationResult] = None
    
    def is_valid(self) -> bool:
        """检查程序是否有效"""
        if self.validation_result is None:
            return True
        return self.validation_result.is_valid

class FunctionCombiner:
    """函数组合器"""
    
    def __init__(self, enable_validation: bool = True):
        """
        初始化组合器
        
        Args:
            enable_validation: 是否启用验证
        """
        self.enable_validation = enable_validation
        if enable_validation:
            self.validator = RustValidator()
        else:
            self.validator = None
        
        # 类型兼容性规则
        self.type_compatibility = {
            "i32": ["i64", "f32", "f64"],
            "i64": ["i32", "f64"], 
            "f32": ["f64", "i32"],
            "f64": ["f32", "i64"],
            "String": ["&str"],
            "&str": ["String"],
            "Vec<T>": ["&[T]"],
            "&[T]": ["Vec<T>"],
        }
    
    def combine_functions(self, functions: List[GeneratedFunction]) -> ComposedProgram:
        """
        组合函数成程序
        
        Args:
            functions: 要组合的函数列表
            
        Returns:
            ComposedProgram: 组合的程序
        """
        if not functions:
            raise ValueError("函数列表不能为空")
        
        console.print(f"[blue]正在组合 {len(functions)} 个函数...[/blue]")
        
        # 解决命名冲突
        renamed_functions = self._resolve_name_conflicts(functions)
        
        # 生成组合代码
        combined_code = self._generate_combined_code(renamed_functions)
        
        # 创建主函数
        main_function = self._create_main_function(renamed_functions)
        complete_code = combined_code + "\n\n" + main_function
        
        # 计算复杂度
        total_complexity = sum(f.complexity for f in functions)
        
        program = ComposedProgram(
            name=f"combined_{len(functions)}_functions",
            code=complete_code,
            functions=renamed_functions,
            complexity=total_complexity,
            main_function=main_function
        )
        
        # 验证组合的程序
        if self.enable_validation and self.validator:
            console.print("  验证组合程序...", end=" ")
            validation_result = self.validator.validate_program(complete_code)
            program.validation_result = validation_result
            
            if validation_result.is_valid:
                console.print("[green]✅ 有效[/green]")
            else:
                console.print("[red]❌ 无效[/red]")
        
        return program
    
    def create_chained_composition(self, functions: List[GeneratedFunction]) -> ComposedProgram:
        """
        创建链式组合（函数调用链）
        
        Args:
            functions: 要链式组合的函数列表
            
        Returns:
            ComposedProgram: 链式组合的程序
        """
        if len(functions) < 2:
            raise ValueError("链式组合至少需要2个函数")
        
        console.print(f"[blue]正在创建 {len(functions)} 个函数的链式组合...[/blue]")
        
        # 分析函数签名以找到兼容的链
        signatures = []
        for func in functions:
            sig = self._extract_function_signature(func.code)
            signatures.append(sig)
        
        # 找到兼容的函数链
        chain_indices = self._find_compatible_chain(signatures)
        
        if len(chain_indices) < 2:
            console.print("[yellow]⚠️ 未找到兼容的函数链，使用简单组合[/yellow]")
            return self.combine_functions(functions)
        
        # 构建链式函数
        chained_functions = [functions[i] for i in chain_indices]
        combined_code = self._generate_combined_code(chained_functions)
        main_function = self._create_chained_main(chained_functions)
        complete_code = combined_code + "\n\n" + main_function
        
        total_complexity = sum(f.complexity for f in chained_functions) + 2  # +2 for chaining
        
        program = ComposedProgram(
            name=f"chained_{len(chained_functions)}_functions",
            code=complete_code,
            functions=chained_functions,
            complexity=total_complexity,
            main_function=main_function
        )
        
        # 验证链式程序
        if self.enable_validation and self.validator:
            validation_result = self.validator.validate_program(complete_code)
            program.validation_result = validation_result
        
        return program
    
    def _resolve_name_conflicts(self, functions: List[GeneratedFunction]) -> List[GeneratedFunction]:
        """解决函数名冲突"""
        used_names: Set[str] = set()
        renamed_functions = []
        
        for i, func in enumerate(functions):
            original_name = self._extract_function_name(func.code)
            
            # 如果名称冲突，生成新名称
            if original_name in used_names:
                new_name = f"{original_name}_{i}"
                new_code = self._rename_function(func.code, original_name, new_name)
                
                # 创建新的函数对象
                new_func = GeneratedFunction(
                    name=new_name,
                    code=new_code,
                    category=func.category,
                    complexity=func.complexity,
                    template_used=func.template_used,
                    validation_result=func.validation_result,
                    generation_time_ms=func.generation_time_ms
                )
                renamed_functions.append(new_func)
                used_names.add(new_name)
            else:
                renamed_functions.append(func)
                used_names.add(original_name)
        
        return renamed_functions
    
    def _generate_combined_code(self, functions: List[GeneratedFunction]) -> str:
        """生成组合的函数代码"""
        code_parts = []
        
        for func in functions:
            code_parts.append(func.code)
        
        return "\n\n".join(code_parts)
    
    def _create_main_function(self, functions: List[GeneratedFunction]) -> str:
        """创建主函数"""
        main_body = []
        main_body.append("fn main() {")
        main_body.append('    println!("Running combined functions:");')
        main_body.append("")
        
        for func in functions:
            func_name = self._extract_function_name(func.code)
            main_body.append(f"    // Call {func_name}")
            main_body.append(f"    // {func_name}();")
            main_body.append("")
        
        main_body.append('    println!("All functions completed.");')
        main_body.append("}")
        
        return "\n".join(main_body)
    
    def _create_chained_main(self, functions: List[GeneratedFunction]) -> str:
        """创建链式主函数"""
        main_body = []
        main_body.append("fn main() {")
        main_body.append('    println!("Running chained functions:");')
        main_body.append("")
        
        for i, func in enumerate(functions):
            func_name = self._extract_function_name(func.code)
            main_body.append(f"    // Step {i+1}: {func_name}")
            main_body.append(f"    // let result_{i} = {func_name}(/* appropriate params */);")
            main_body.append("")
        
        main_body.append('    println!("Chain execution completed.");')
        main_body.append("}")
        
        return "\n".join(main_body)
    
    def _extract_function_name(self, code: str) -> str:
        """从代码中提取函数名"""
        match = re.search(r'fn\s+(\w+)\s*\(', code)
        if match:
            return match.group(1)
        return "unknown_function"
    
    def _rename_function(self, code: str, old_name: str, new_name: str) -> str:
        """重命名函数"""
        pattern = rf'\bfn\s+{re.escape(old_name)}\b'
        return re.sub(pattern, f'fn {new_name}', code)
    
    def _extract_function_signature(self, code: str) -> Dict[str, any]:
        """提取函数签名"""
        # 简化的签名提取
        match = re.search(r'fn\s+(\w+)\s*\((.*?)\)\s*(?:->\s*([^{]+))?', code)
        if match:
            name = match.group(1)
            params_str = match.group(2) or ""
            return_type = (match.group(3) or "()").strip()
            
            # 解析参数（简化版）
            params = []
            if params_str.strip():
                for param in params_str.split(','):
                    if ':' in param:
                        param_type = param.split(':')[1].strip()
                        params.append(param_type)
            
            return {
                "name": name,
                "params": params,
                "return_type": return_type
            }
        
        return {"name": "unknown", "params": [], "return_type": "()"}
    
    def _find_compatible_chain(self, signatures: List[Dict]) -> List[int]:
        """找到兼容的函数链"""
        # 简化的链查找：如果有函数，就返回前两个的索引
        if len(signatures) >= 2:
            return [0, 1]
        return []
    
    def save_program(self, program: ComposedProgram, output_file: Path):
        """
        保存组合的程序到文件
        
        Args:
            program: 组合的程序
            output_file: 输出文件路径
        """
        validation_info = ""
        if program.validation_result:
            validation_info = f"""// 验证结果: {"✅ 有效" if program.is_valid() else "❌ 无效"}
// 编译状态: {"成功" if program.validation_result.compilation_success else "失败"}
// 验证时间: {program.validation_result.validation_time_ms:.2f}ms
// 警告数量: {len(program.validation_result.warnings)}
"""
        
        content = f"""// 组合程序: {program.name}
// 函数数量: {len(program.functions)}
// 复杂度: {program.complexity}
{validation_info}
{program.code}
"""
        
        output_file.parent.mkdir(parents=True, exist_ok=True)
        output_file.write_text(content, encoding='utf-8')
        
        console.print(f"[green]已保存组合程序到 {output_file}[/green]")


if __name__ == "__main__":
    # 测试组合器
    console.print("[bold blue]测试函数组合器[/bold blue]")
    
    # 创建一些示例函数用于测试
    func1 = GeneratedFunction(
        name="add_numbers",
        code="""fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}""",
        category="arithmetic",
        complexity=2
    )
    
    func2 = GeneratedFunction(
        name="multiply_by_two", 
        code="""fn multiply_by_two(x: i32) -> i32 {
    x * 2
}""",
        category="arithmetic",
        complexity=1
    )
    
    # 测试组合
    combiner = FunctionCombiner(enable_validation=True)
    program = combiner.combine_functions([func1, func2])
    
    console.print(f"\n[green]组合程序生成成功![/green]")
    console.print(f"名称: {program.name}")
    console.print(f"复杂度: {program.complexity}")
    console.print(f"有效性: {'✅' if program.is_valid() else '❌'}")
    
    console.print(f"\n[cyan]生成的代码:[/cyan]")
    console.print(program.code)