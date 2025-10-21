#!/usr/bin/env python3
"""
组合函数命令行工具
"""

import asyncio
import sys
import json
from pathlib import Path
import click
from rich.console import Console

# 添加项目根目录到Python路径
sys.path.append(str(Path(__file__).parent.parent))

from rustlego.combiner import FunctionCombiner
from rustlego.generator import GeneratedFunction
from rustlego.validator import RustValidator

console = Console()

def load_functions_from_directory(input_dir: Path) -> list[GeneratedFunction]:
    """从目录加载生成的函数"""
    functions = []
    
    # 查找所有.rs文件
    for rust_file in input_dir.glob("*.rs"):
        try:
            content = rust_file.read_text(encoding='utf-8')
            
            # 从注释中提取元数据
            name = _extract_from_comment(content, "生成的函数:")
            category = _extract_from_comment(content, "类别:")
            complexity_str = _extract_from_comment(content, "复杂度:")
            template = _extract_from_comment(content, "模板:")
            
            complexity = int(complexity_str) if complexity_str.isdigit() else 1
            
            # 提取实际的函数代码（去掉注释）
            code_lines = []
            in_comment = True
            for line in content.split('\n'):
                if not line.strip().startswith('//') and line.strip():
                    in_comment = False
                if not in_comment:
                    code_lines.append(line)
            
            function_code = '\n'.join(code_lines).strip()
            
            if function_code:
                func = GeneratedFunction(
                    name=name or rust_file.stem,
                    code=function_code,
                    category=category or "unknown",
                    complexity=complexity,
                    template_used=template
                )
                functions.append(func)
                
        except Exception as e:
            console.print(f"[yellow]⚠️ 跳过文件 {rust_file}: {e}[/yellow]")
    
    return functions

def _extract_from_comment(content: str, prefix: str) -> str:
    """从注释中提取信息"""
    for line in content.split('\n'):
        if line.strip().startswith('//') and prefix in line:
            return line.split(prefix, 1)[1].strip()
    return ""

@click.command()
@click.option('--input', '-i', required=True,
              help='包含生成函数的输入目录')
@click.option('--output', '-o', default='composed',
              help='组合程序的输出目录')
@click.option('--count', '-c', default=3,
              help='要组合的程序数量')
@click.option('--functions-per-program', '-f', default=3,
              help='每个程序的函数数量')
@click.option('--chained/--combined', default=False,
              help='使用链式组合而不是简单组合')
@click.option('--validate/--no-validate', default=True,
              help='验证组合的程序')
@click.option('--stats/--no-stats', default=False,
              help='显示详细统计信息')
def compose(input, output, count, functions_per_program, chained, validate, stats):
    """组合函数成复杂程序"""
    
    console.print("[bold blue]RustLego-Py 函数组合器[/bold blue]")
    console.print("=" * 30)
    console.print(f"输入目录: {input}")
    console.print(f"输出目录: {output}")
    console.print(f"程序数量: {count}")
    console.print(f"每程序函数数: {functions_per_program}")
    console.print(f"组合模式: {'🔗 链式' if chained else '🔧 组合'}")
    console.print(f"验证: {'✅ 启用' if validate else '❌ 禁用'}")
    console.print()
    
    try:
        # 加载函数
        input_dir = Path(input)
        if not input_dir.exists():
            console.print(f"[red]❌ 输入目录不存在: {input}[/red]")
            sys.exit(1)
        
        functions = load_functions_from_directory(input_dir)
        
        if not functions:
            console.print(f"[red]❌ 在 {input} 中没有找到任何函数[/red]")
            sys.exit(1)
        
        console.print(f"[green]✅ 加载了 {len(functions)} 个函数[/green]")
        
        # 创建组合器
        combiner = FunctionCombiner(enable_validation=validate)
        
        # 创建输出目录
        output_dir = Path(output)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # 组合程序
        composed_programs = []
        valid_programs = 0
        
        console.print(f"\n[blue]开始组合 {count} 个程序...[/blue]")
        
        for i in range(count):
            # 随机选择函数
            import random
            if len(functions) < functions_per_program:
                selected_functions = functions.copy()
            else:
                selected_functions = random.sample(functions, functions_per_program)
            
            console.print(f"组合程序 {i+1}/{count}...")
            
            try:
                # 组合函数
                if chained:
                    program = combiner.create_chained_composition(selected_functions)
                else:
                    program = combiner.combine_functions(selected_functions)
                
                # 检查有效性
                is_valid = program.is_valid()
                if is_valid:
                    valid_programs += 1
                
                # 保存程序
                program_file = output_dir / f"{i+1:03d}_{program.name}.rs"
                combiner.save_program(program, program_file)
                
                status = "✅" if is_valid else "❌"
                console.print(f"  {status} 程序 {i+1}: {program.name} (有效: {is_valid})")
                
                composed_programs.append(program)
                
            except Exception as e:
                console.print(f"  [red]❌ 组合失败: {e}[/red]")
        
        # 显示统计信息
        console.print(f"\n[bold green]✅ 组合完成![/bold green]")
        console.print(f"总程序数: {len(composed_programs)}")
        console.print(f"有效程序: {valid_programs}")
        
        if composed_programs:
            success_rate = valid_programs / len(composed_programs)
            console.print(f"成功率: {success_rate:.1%}")
            
            if validate and stats:
                # 显示详细统计
                validation_results = [p.validation_result for p in composed_programs 
                                    if p.validation_result]
                if validation_results:
                    validator = RustValidator()
                    validator.print_validation_stats(validation_results)
            
            if validate and success_rate < 0.8:
                console.print("\n[yellow]⚠️ 警告: 组合成功率较低，建议:[/yellow]")
                console.print("   - 使用预验证的函数")
                console.print("   - 检查函数兼容性")
                console.print("   - 简化组合逻辑")
        
        console.print(f"\n[green]✅ 结果保存在: {output}[/green]")
        
    except Exception as e:
        console.print(f"[red]❌ 组合失败: {e}[/red]")
        sys.exit(1)

if __name__ == '__main__':
    compose()