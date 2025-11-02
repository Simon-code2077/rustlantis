#!/usr/bin/env python3
"""
Rust编译验证器

提供Rust代码的编译验证功能，支持：
- 单个函数验证
- 完整程序验证  
- 批量验证
- 详细的错误报告和统计
"""

import subprocess
import tempfile
import json
import time
import os
from pathlib import Path
from typing import Optional, List, Dict, Tuple
from dataclasses import dataclass
from rich.console import Console
from rich.table import Table

console = Console()

@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    compilation_success: bool
    error_message: Optional[str] = None
    warnings: List[str] = None
    validation_time_ms: float = 0.0
    miri_success: Optional[bool] = None
    miri_error_message: Optional[str] = None
    
    def __post_init__(self):
        if self.warnings is None:
            self.warnings = []

@dataclass 
class ValidationStats:
    """验证统计信息"""
    total_functions: int
    valid_functions: int
    compiled_functions: int
    functions_with_warnings: int
    success_rate: float
    compilation_rate: float
    average_validation_time_ms: float
    
    def __str__(self) -> str:
        return f"""验证统计:
  总函数数: {self.total_functions}
  有效函数: {self.valid_functions} ({self.success_rate:.1%})
  可编译函数: {self.compiled_functions} ({self.compilation_rate:.1%})
  有警告函数: {self.functions_with_warnings}
  平均验证时间: {self.average_validation_time_ms:.2f}ms"""

class RustValidator:
    """Rust代码编译验证器"""
    
    def __init__(self, 
                 validate_with_warnings: bool = False,
                 timeout_seconds: int = 30,
                 use_miri: bool = False):
        """
        初始化验证器
        
        Args:
            validate_with_warnings: 是否将有警告的代码视为无效
            timeout_seconds: 编译超时时间
            use_miri: 是否运行 miri 来检查未定义行为
        """
        self.validate_with_warnings = validate_with_warnings
        self.timeout_seconds = timeout_seconds
        self.use_miri = use_miri
        
    def validate_function(self, function_code: str) -> ValidationResult:
        """
        验证单个函数
        
        Args:
            function_code: Rust函数代码
            
        Returns:
            ValidationResult: 验证结果
        """
        start_time = time.time()
        
        # 检查是否已经是完整程序
        if self._is_complete_program(function_code):
            # 如果已经包含 main 函数，直接验证
            result = self.validate_program(function_code)
        else:
            # 创建完整的Rust程序
            complete_program = self._wrap_function_for_validation(function_code)
            result = self.validate_program(complete_program)
        
        # 更新验证时间
        validation_time = (time.time() - start_time) * 1000
        result.validation_time_ms = validation_time
        
        return result
        
    def validate_program(self, program_code: str) -> ValidationResult:
        """
        验证完整的Rust程序
        
        Args:
            program_code: 完整的Rust程序代码
            
        Returns:
            ValidationResult: 验证结果
        """
        start_time = time.time()
        
        try:
            with tempfile.TemporaryDirectory() as temp_dir:
                temp_path = Path(temp_dir)
                
                # 创建Cargo项目
                project_dir = self._create_temp_project(temp_path, program_code)
                
                # 运行cargo check
                result = self._run_cargo_check(project_dir)
                
                # 如果启用了 miri 且 cargo check 通过，运行 miri check
                if self.use_miri and result.compilation_success:
                    miri_result = self._run_miri_check(project_dir)
                    result.miri_success = miri_result['success']
                    result.miri_error_message = miri_result['error_message']
                    # 如果 miri 发现问题，标记为无效
                    if not miri_result['success']:
                        result.is_valid = False
                
                validation_time = (time.time() - start_time) * 1000
                result.validation_time_ms = validation_time
                
                return result
                
        except Exception as e:
            return ValidationResult(
                is_valid=False,
                compilation_success=False,
                error_message=f"验证过程出错: {str(e)}",
                validation_time_ms=(time.time() - start_time) * 1000
            )
    
    def validate_functions_batch(self, functions: List[str]) -> List[ValidationResult]:
        """
        批量验证函数
        
        Args:
            functions: 函数代码列表
            
        Returns:
            List[ValidationResult]: 验证结果列表
        """
        results = []
        
        console.print(f"[bold blue]开始批量验证 {len(functions)} 个函数...[/bold blue]")
        
        for i, function in enumerate(functions, 1):
            console.print(f"验证函数 {i}/{len(functions)}...", end=" ")
            
            result = self.validate_function(function)
            
            if result.is_valid:
                console.print("[green]✅ 有效[/green]")
            else:
                console.print(f"[red]❌ 无效: {result.error_message or '未知错误'}[/red]")
                
            results.append(result)
            
        return results
    
    def _is_complete_program(self, code: str) -> bool:
        """
        检查代码是否已经是完整的程序（包含main函数）
        
        Args:
            code: 代码字符串
            
        Returns:
            bool: 如果包含main函数则返回True
        """
        import re
        # 检查是否包含 main 函数定义
        main_pattern = r'\bfn\s+main\s*\('
        return bool(re.search(main_pattern, code))
    
    def _wrap_function_for_validation(self, function_code: str) -> str:
        """为验证包装函数代码"""
        return f"""// 自动生成的验证包装器
#![allow(unused)]

{function_code}

fn main() {{
    // 这个main函数仅用于编译测试
    println!("Function validation wrapper");
}}
"""
    
    def _create_temp_project(self, temp_dir: Path, program_code: str) -> Path:
        """创建临时Cargo项目"""
        project_dir = temp_dir / "validation_project"
        
        # 创建项目结构
        project_dir.mkdir()
        src_dir = project_dir / "src"
        src_dir.mkdir()
        
        # 创建Cargo.toml
        cargo_toml = """[package]
name = "validation_project"
version = "0.1.0"
edition = "2021"

[dependencies]
"""
        (project_dir / "Cargo.toml").write_text(cargo_toml)
        
        # 创建main.rs
        (src_dir / "main.rs").write_text(program_code)
        
        return project_dir
    
    def _run_cargo_check(self, project_dir: Path) -> ValidationResult:
        """运行cargo check"""
        try:
            result = subprocess.run(
                ["cargo", "check", "--message-format=json"],
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=self.timeout_seconds
            )
            
            compilation_success = result.returncode == 0
            error_message = None
            warnings = []
            
            # 解析JSON输出
            if not compilation_success:
                error_message = self._parse_compilation_error(result.stdout, result.stderr)
            
            warnings = self._parse_warnings(result.stdout)
            
            # 判断是否有效
            is_valid = compilation_success and (
                not self.validate_with_warnings or len(warnings) == 0
            )
            
            return ValidationResult(
                is_valid=is_valid,
                compilation_success=compilation_success,
                error_message=error_message,
                warnings=warnings
            )
            
        except subprocess.TimeoutExpired:
            return ValidationResult(
                is_valid=False,
                compilation_success=False,
                error_message="编译超时"
            )
        except Exception as e:
            return ValidationResult(
                is_valid=False,
                compilation_success=False,
                error_message=f"编译过程出错: {str(e)}"
            )
    
    def _run_miri_check(self, project_dir: Path) -> Dict:
        """运行 miri check 来检测未定义行为"""
        try:
            result = subprocess.run(
                ["cargo", "+nightly", "miri", "run"],
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=self.timeout_seconds,
                env={**os.environ, "MIRIFLAGS": "-Zmiri-strict-provenance"}
            )
            
            success = result.returncode == 0
            error_message = None
            
            if not success:
                error_message = self._parse_miri_error(result.stdout, result.stderr)
            
            return {
                'success': success,
                'error_message': error_message
            }
            
        except subprocess.TimeoutExpired:
            return {
                'success': False,
                'error_message': "Miri 检查超时"
            }
        except FileNotFoundError:
            return {
                'success': False,
                'error_message': "未找到 Miri，请运行: rustup +nightly component add miri"
            }
        except Exception as e:
            return {
                'success': False,
                'error_message': f"Miri 检查出错: {str(e)}"
            }
    
    def _parse_miri_error(self, stdout: str, stderr: str) -> str:
        """解析 miri 错误信息"""
        # 优先使用 stderr
        if stderr.strip():
            return stderr
        
        if stdout.strip():
            return stdout
        
        return "未知 Miri 错误"
    
    def _parse_compilation_error(self, stdout: str, stderr: str) -> str:
        """解析编译错误"""
        # 尝试从JSON输出中提取错误信息
        for line in stdout.split('\n'):
            if line.strip():
                try:
                    data = json.loads(line)
                    if data.get('reason') == 'compiler-message':
                        message = data.get('message', {})
                        if message.get('level') == 'error':
                            rendered = message.get('rendered', '')
                            if rendered:
                                return rendered
                except json.JSONDecodeError:
                    continue
        
        # 后备方案：使用stderr
        if stderr.strip():
            return stderr
        
        return "未知编译错误"
    
    def _parse_warnings(self, stdout: str) -> List[str]:
        """解析编译警告"""
        warnings = []
        
        for line in stdout.split('\n'):
            if line.strip():
                try:
                    data = json.loads(line)
                    if data.get('reason') == 'compiler-message':
                        message = data.get('message', {})
                        if message.get('level') == 'warning':
                            rendered = message.get('rendered', '')
                            if rendered:
                                warnings.append(rendered)
                except json.JSONDecodeError:
                    continue
        
        return warnings
    
    @staticmethod
    def get_validation_stats(results: List[ValidationResult]) -> ValidationStats:
        """计算验证统计信息"""
        total = len(results)
        if total == 0:
            return ValidationStats(0, 0, 0, 0, 0.0, 0.0, 0.0)
        
        valid = sum(1 for r in results if r.is_valid)
        compiled = sum(1 for r in results if r.compilation_success)
        with_warnings = sum(1 for r in results if r.warnings)
        
        avg_time = sum(r.validation_time_ms for r in results) / total
        
        return ValidationStats(
            total_functions=total,
            valid_functions=valid,
            compiled_functions=compiled,
            functions_with_warnings=with_warnings,
            success_rate=valid / total,
            compilation_rate=compiled / total,
            average_validation_time_ms=avg_time
        )
    
    def print_validation_stats(self, results: List[ValidationResult]):
        """打印验证统计信息"""
        stats = self.get_validation_stats(results)
        
        table = Table(title="验证统计信息")
        table.add_column("指标", style="cyan")
        table.add_column("值", style="magenta")
        table.add_column("百分比", style="green")
        
        table.add_row("总函数数", str(stats.total_functions), "100%")
        table.add_row("有效函数", str(stats.valid_functions), f"{stats.success_rate:.1%}")
        table.add_row("可编译函数", str(stats.compiled_functions), f"{stats.compilation_rate:.1%}")
        table.add_row("有警告函数", str(stats.functions_with_warnings), "")
        table.add_row("平均验证时间", f"{stats.average_validation_time_ms:.2f}ms", "")
        
        console.print(table)


if __name__ == "__main__":
    # 简单测试
    validator = RustValidator(use_miri=False)
    
    # 测试有效函数
    valid_function = '''
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
'''
    
    # 测试无效函数
    invalid_function = '''
fn invalid_function(a: i32) -> i32 {
    a + unknown_variable  // 这会导致编译错误
}
'''
    
    console.print("[bold green]测试有效函数:[/bold green]")
    result1 = validator.validate_function(valid_function)
    console.print(f"结果: {'✅ 有效' if result1.is_valid else '❌ 无效'}")
    
    console.print("[bold red]测试无效函数:[/bold red]")
    result2 = validator.validate_function(invalid_function)
    console.print(f"结果: {'✅ 有效' if result2.is_valid else '❌ 无效'}")
    console.print(f"错误信息: {result2.error_message}")
    
    # 测试 Miri 检查（需要 nightly 工具链）
    console.print("[bold blue]测试 Miri 检查:[/bold blue]")
    validator_with_miri = RustValidator(use_miri=True)
    result3 = validator_with_miri.validate_function(valid_function)
    console.print(f"Cargo Check 结果: {'✅' if result3.compilation_success else '❌'}")
    if result3.miri_success is not None:
        console.print(f"Miri 检查结果: {'✅' if result3.miri_success else '❌'}")
        if result3.miri_error_message:
            console.print(f"Miri 错误: {result3.miri_error_message}")