#!/usr/bin/env python3
"""
验证函数命令行工具
"""

import sys
from pathlib import Path
import click
from rich.console import Console

# 添加项目根目录到Python路径
sys.path.append(str(Path(__file__).parent.parent))

from rustlego.validator import RustValidator
from rustlego.generator import GeneratedFunction

console = Console()

def load_rust_files(input_path: Path) -> list[str]:
    """加载Rust文件内容"""
    rust_codes = []
    
    if input_path.is_file():
        # 单个文件
        if input_path.suffix == '.rs':
            content = input_path.read_text(encoding='utf-8')
            rust_codes.append(extract_rust_code(content))
    elif input_path.is_dir():
        # 目录中的所有.rs文件
        for rust_file in input_path.glob("*.rs"):
            try:
                content = rust_file.read_text(encoding='utf-8')
                code = extract_rust_code(content)
                if code:
                    rust_codes.append(code)
                    
            except Exception as e:
                console.print(f"[yellow]⚠️ 跳过文件 {rust_file}: {e}[/yellow]")
    
    return rust_codes

def extract_rust_code(content: str) -> str:
    """
    从文件内容中提取Rust代码
    
    智能处理：
    1. 如果包含main函数，返回完整内容（去掉注释头）
    2. 如果只是函数，返回函数代码
    """
    # 检查是否包含main函数
    if 'fn main(' in content:
        # 完整程序，去掉开头的注释
        lines = content.split('\n')
        code_lines = []
        found_code = False
        
        for line in lines:
            # 跳过开头的注释，但保留代码中的注释
            if not found_code and line.strip().startswith('//'):
                continue
            else:
                found_code = True
                code_lines.append(line)
        
        return '\n'.join(code_lines).strip()
    else:
        # 只是函数，提取函数代码（去掉注释）
        code_lines = []
        in_comment = True
        for line in content.split('\n'):
            if not line.strip().startswith('//') and line.strip():
                in_comment = False
            if not in_comment:
                code_lines.append(line)
        
        return '\n'.join(code_lines).strip()

@click.command()
@click.option('--input', '-i', required=True,
              help='要验证的Rust文件或目录')
@click.option('--with-warnings/--no-warnings', default=False,
              help='将有警告的代码视为无效')
@click.option('--timeout', default=30,
              help='编译超时时间（秒）')
@click.option('--stats/--no-stats', default=True,
              help='显示详细统计信息')
@click.option('--verbose', '-v', is_flag=True,
              help='显示详细的验证过程')
def validate(input, with_warnings, timeout, stats, verbose):
    """验证Rust代码编译性"""
    
    console.print("[bold blue]RustLego-Py 编译验证器[/bold blue]")
    console.print("=" * 30)
    console.print(f"输入: {input}")
    console.print(f"处理警告: {'视为错误' if with_warnings else '忽略'}")
    console.print(f"超时时间: {timeout}秒")
    console.print()
    
    try:
        # 加载文件
        input_path = Path(input)
        if not input_path.exists():
            console.print(f"[red]❌ 输入路径不存在: {input}[/red]")
            sys.exit(1)
        
        rust_codes = load_rust_files(input_path)
        
        if not rust_codes:
            console.print(f"[red]❌ 没有找到任何Rust代码[/red]")
            sys.exit(1)
        
        console.print(f"[green]✅ 加载了 {len(rust_codes)} 个代码片段[/green]")
        
        # 创建验证器
        validator = RustValidator(
            validate_with_warnings=with_warnings,
            timeout_seconds=timeout
        )
        
        # 验证代码
        if verbose:
            console.print("\n[blue]开始详细验证过程:[/blue]")
        
        if len(rust_codes) == 1:
            # 单个代码验证
            result = validator.validate_program(rust_codes[0])
            
            if result.is_valid:
                console.print("[green]✅ 代码验证通过![/green]")
            else:
                console.print("[red]❌ 代码验证失败![/red]")
                if result.error_message:
                    console.print(f"错误信息: {result.error_message}")
            
            if result.warnings and verbose:
                console.print(f"\n警告信息:")
                for warning in result.warnings:
                    console.print(f"  ⚠️ {warning}")
            
            console.print(f"验证时间: {result.validation_time_ms:.2f}ms")
            
        else:
            # 批量验证
            results = validator.validate_functions_batch(rust_codes)
            
            # 显示统计信息
            if stats:
                console.print("\n")
                validator.print_validation_stats(results)
            
            # 显示个别失败的详情
            if verbose:
                console.print("\n[red]失败的代码详情:[/red]")
                for i, result in enumerate(results, 1):
                    if not result.is_valid:
                        console.print(f"代码 {i}: {result.error_message}")
        
        # 总结
        if len(rust_codes) > 1:
            valid_count = sum(1 for result in results if result.is_valid)
            success_rate = valid_count / len(results)
            
            console.print(f"\n[bold]总结:[/bold]")
            console.print(f"总代码数: {len(results)}")
            console.print(f"有效代码: {valid_count}")
            console.print(f"成功率: {success_rate:.1%}")
            
            if success_rate == 1.0:
                console.print("[green]🎉 所有代码都通过验证![/green]")
                sys.exit(0)
            elif success_rate >= 0.8:
                console.print("[yellow]⚠️ 大部分代码通过验证[/yellow]")
                sys.exit(0)
            else:
                console.print("[red]❌ 许多代码未通过验证[/red]")
                sys.exit(1)
        else:
            if result.is_valid:
                sys.exit(0)
            else:
                sys.exit(1)
        
    except Exception as e:
        console.print(f"[red]❌ 验证失败: {e}[/red]")
        sys.exit(1)

if __name__ == '__main__':
    validate()