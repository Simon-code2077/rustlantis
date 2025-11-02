#!/usr/bin/env python3
"""
生成函数命令行工具
"""

import asyncio
import sys
from pathlib import Path
import click
from rich.console import Console

# 添加项目根目录到Python路径
sys.path.append(str(Path(__file__).parent.parent))

from rustlego.generator import FunctionGenerator
from rustlego.validator import RustValidator

console = Console()

@click.command()
@click.option('--category', '-c', required=True, 
              help='函数类别 (arithmetic, memory, control_flow, type_conversion, string_ops)')
@click.option('--count', '-n', default=5, 
              help='生成函数数量')
@click.option('--output', '-o', default='examples/generated',
              help='输出目录')
@click.option('--validate/--no-validate', default=True,
              help='是否启用编译验证')
@click.option('--stats/--no-stats', default=False,
              help='显示详细统计信息')
@click.option('--format', 'output_format', default='rust',
              type=click.Choice(['rust', 'json', 'both']),
              help='输出格式')
def generate(category, count, output, validate, stats, output_format):
    """生成Rust函数"""
    
    console.print("[bold blue]RustLego-Py 函数生成器[/bold blue]")
    console.print("=" * 30)
    console.print(f"类别: {category}")
    console.print(f"数量: {count}")
    console.print(f"输出: {output}")
    console.print(f"验证: {'✅ 启用' if validate else '❌ 禁用'}")
    console.print(f"格式: {output_format}")
    console.print()
    
    async def run_generation():
        try:
            # 创建生成器
            generator = FunctionGenerator(enable_validation=validate)
            
            # 生成函数
            functions = await generator.generate_batch(category, count)
            
            if not functions:
                console.print("[red]❌ 没有生成任何函数[/red]")
                return False
            
            # 保存函数
            output_dir = Path(output)
            
            if output_format in ['rust', 'both']:
                generator.save_functions(functions, output_dir)
            
            if output_format in ['json', 'both']:
                json_file = output_dir / f"{category}_functions.json"
                generator.save_functions_json(functions, json_file)
            
            # 显示统计信息
            if stats or validate:
                if validate:
                    results = [f.validation_result for f in functions if f.validation_result]
                    if results:
                        validator = RustValidator()
                        validator.print_validation_stats(results)
                
                console.print(f"\n[bold green]✅ 成功生成 {len(functions)} 个函数![/bold green]")
                
                # 性能统计
                avg_generation_time = sum(f.generation_time_ms for f in functions) / len(functions)
                console.print(f"平均生成时间: {avg_generation_time:.2f}ms")
                
                if validate:
                    valid_count = sum(1 for f in functions if f.is_valid())
                    success_rate = valid_count / len(functions) if functions else 0
                    console.print(f"验证成功率: {success_rate:.1%}")
                    
            
            return True
            
        except Exception as e:
            console.print(f"[red]❌ 生成失败: {e}[/red]")
            return False
    
    # 运行异步生成
    success = asyncio.run(run_generation())
    
    if success:
        console.print(f"\n[green]✅ 生成完成! 结果保存在: {output}[/green]")
        sys.exit(0)
    else:
        sys.exit(1)

if __name__ == '__main__':
    generate()