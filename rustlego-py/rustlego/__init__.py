"""
RustLego-Py: Python版本的Rust代码生成器

这个包提供了生成、验证和组合Rust函数的工具。
"""

__version__ = "0.1.0"
__author__ = "RustLego Team"

from .validator import RustValidator, ValidationResult
from .generator import FunctionGenerator, GeneratedFunction
from .combiner import FunctionCombiner, ComposedProgram
from .templates import TemplateLibrary, FunctionTemplate

__all__ = [
    "RustValidator",
    "ValidationResult", 
    "FunctionGenerator",
    "GeneratedFunction",
    "FunctionCombiner",
    "ComposedProgram",
    "TemplateLibrary",
    "FunctionTemplate",
]