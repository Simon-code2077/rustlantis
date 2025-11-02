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
        """创建智能链式主函数，正确处理Result类型的转换"""
        main_body = []
        main_body.append("fn main() {")
        main_body.append('    println!("Running chained functions:");')
        main_body.append("")
        
        # 简化策略：收集所有步骤和类型信息
        steps = []
        for i, func in enumerate(functions):
            func_name = self._extract_function_name(func.code)
            sig = self._extract_function_signature(func.code)
            steps.append({
                'index': i,
                'name': func_name,
                'sig': sig,
                'return_type': sig['return_type'],
                'params': sig['params']
            })
        
        # 生成链式调用代码
        self._generate_chain_code(main_body, steps, 0, None)
        
        main_body.append("")
        main_body.append('    println!("Chain execution completed.");')
        main_body.append("}")
        
        return "\n".join(main_body)
    
    def _generate_chain_code(self, main_body: List[str], steps: List[Dict], 
                           step_idx: int, prev_var: Optional[str], indent_level: int = 1):
        """递归生成链式调用代码"""
        if step_idx >= len(steps):
            return
        
        indent = "    " * indent_level
        step = steps[step_idx]
        func_name = step['name']
        sig = step['sig']
        
        main_body.append(f"{indent}// Step {step_idx + 1}: {func_name}")
        
        # 生成函数调用
        if step_idx == 0:
            call_str = self._generate_function_call(func_name, sig, None)
        else:
            # 使用前一个结果
            call_str = self._generate_chained_function_call(
                func_name, sig, prev_var, 
                self._extract_result_inner_type(steps[step_idx - 1]['return_type'])
            )
        
        main_body.append(f"{indent}let result_{step_idx} = {call_str};")
        
        return_type = step['return_type']
        
        # 检查返回类型
        if 'Result<' in return_type:
            main_body.append(f"{indent}match result_{step_idx} {{")
            main_body.append(f"{indent}    Ok(value_{step_idx}) => {{")
            main_body.append(f'{indent}        println!("Result {step_idx + 1}: {{:?}}", result_{step_idx});')
            
            # 递归处理下一步
            if step_idx < len(steps) - 1:
                self._generate_chain_code(main_body, steps, step_idx + 1, 
                                        f"value_{step_idx}", indent_level + 2)
            
            main_body.append(f"{indent}    }}")
            main_body.append(f"{indent}    Err(e) => {{")
            main_body.append(f'{indent}        println!("Error in step {step_idx + 1}: {{}}", e);')
            main_body.append(f"{indent}    }}")
            main_body.append(f"{indent}}}")
        elif 'Option<' in return_type:
            # 处理 Option 类型
            main_body.append(f"{indent}match result_{step_idx} {{")
            main_body.append(f"{indent}    Some(value_{step_idx}) => {{")
            main_body.append(f'{indent}        println!("Result {step_idx + 1}: {{:?}}", result_{step_idx});')
            
            if step_idx < len(steps) - 1:
                self._generate_chain_code(main_body, steps, step_idx + 1,
                                        f"value_{step_idx}", indent_level + 2)
            
            main_body.append(f"{indent}    }}")
            main_body.append(f"{indent}    None => {{")
            main_body.append(f'{indent}        println!("Result {step_idx + 1}: None");')
            main_body.append(f"{indent}    }}")
            main_body.append(f"{indent}}}")
        else:
            # 普通类型，直接使用
            main_body.append(f'{indent}println!("Result {step_idx + 1}: {{:?}}", result_{step_idx});')
            
            if step_idx < len(steps) - 1:
                self._generate_chain_code(main_body, steps, step_idx + 1,
                                        f"result_{step_idx}", indent_level)
    
    def _extract_result_inner_type(self, result_type: str) -> str:
        """从Result<T, E>中提取T"""
        match = re.search(r'Result<([^,]+),', result_type)
        if match:
            return match.group(1).strip()
        return "i32"  # 默认类型
    
    def _generate_chained_function_call(self, func_name: str, signature: Dict, 
                                       prev_var: str, prev_type: str) -> str:
        """生成链式调用，正确处理类型转换"""
        params = signature.get('params', [])
        
        if not params:
            return f"{func_name}()"
        
        call_params = []
        
        for i, param_type in enumerate(params):
            if i == 0:
                # 第一个参数使用前一个结果
                # 检查是否需要类型转换
                param_normalized = self._normalize_type(param_type)
                prev_normalized = self._normalize_type(prev_type)
                
                if param_normalized == prev_normalized:
                    call_params.append(prev_var)
                else:
                    # 需要类型转换，使用 as 或其他方式
                    if self._is_numeric_type(param_normalized) and self._is_numeric_type(prev_normalized):
                        call_params.append(f"{prev_var} as {param_normalized}")
                    else:
                        call_params.append(prev_var)
            else:
                # 其他参数生成默认值
                default_val = self._generate_default_value(param_type)
                call_params.append(default_val)
        
        return f"{func_name}({', '.join(call_params)})"
    
    def _is_numeric_type(self, type_str: str) -> bool:
        """检查是否是数值类型"""
        numeric_types = {'i8', 'i16', 'i32', 'i64', 'isize', 'u8', 'u16', 'u32', 'u64', 'usize', 'f32', 'f64'}
        return type_str in numeric_types
    
    def _generate_function_call(self, func_name: str, signature: Dict, prev_result: Optional[str]) -> str:
        """生成智能的函数调用"""
        params = signature.get('params', [])
        
        if not params:
            return f"{func_name}()"
        
        call_params = []
        
        for i, param_type in enumerate(params):
            if i == 0 and prev_result:
                # 第一个参数使用前一个结果
                call_params.append(prev_result)
            else:
                # 为其他参数生成适当的默认值
                default_val = self._generate_default_value(param_type)
                call_params.append(default_val)
        
        return f"{func_name}({', '.join(call_params)})"
    
    def _generate_default_value(self, type_str: str) -> str:
        """为类型生成默认值"""
        normalized = self._normalize_type(type_str)
        
        # 数值类型
        if normalized in ['i8', 'i16', 'i32', 'i64', 'isize']:
            return str(random.randint(1, 100))
        elif normalized in ['u8', 'u16', 'u32', 'u64', 'usize']:
            return str(random.randint(1, 100))
        elif normalized in ['f32', 'f64']:
            return f"{random.uniform(1.0, 100.0):.2f}"
        
        # 布尔类型
        elif normalized == 'bool':
            return random.choice(['true', 'false'])
        
        # 字符串类型
        elif normalized in ['String', 'str']:
            words = ['hello', 'world', 'rust', 'code', 'test', 'data']
            return f'"{random.choice(words)}".to_string()'
        elif '&str' in type_str:
            words = ['hello', 'world', 'rust', 'code', 'test', 'data']
            return f'"{random.choice(words)}"'
        
        # Result 类型 - 生成 Ok 变体
        elif 'Result<' in type_str:
            inner_type = self._extract_generic_type(type_str).split(',')[0].strip()
            inner_val = self._generate_default_value(inner_type)
            return f"Ok({inner_val})"
        
        # Option 类型 - 生成 Some 变体
        elif 'Option<' in type_str:
            inner_type = self._extract_generic_type(type_str)
            inner_val = self._generate_default_value(inner_type)
            return f"Some({inner_val})"
        
        # 向量类型
        elif 'Vec<' in type_str:
            inner_type = self._extract_generic_type(type_str)
            inner_val = self._generate_default_value(inner_type)
            return f"vec![{inner_val}, {inner_val}]"
        
        # 复杂自定义类型 - 尝试构造函数
        elif any(word in type_str for word in ['Config', 'ConnectionPool', 'User']):
            return f"Default::default() /* {type_str} */"
        
        # 默认情况
        else:
            return f"todo!() /* {type_str} */"
    
    def _extract_generic_type(self, type_str: str) -> str:
        """提取泛型内部类型"""
        match = re.search(r'<([^>]+)>', type_str)
        if match:
            return match.group(1).strip()
        return "i32"  # 默认类型
    
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
    
    def _find_compatible_chain(self, signatures: List[Dict]) -> List[int]:
        """
        找到兼容的函数链 - 高效版本
        
        使用贪心算法快速找到较好的函数调用链
        """
        if len(signatures) < 2:
            return []
        
        # 对于大规模函数集，使用快速贪心算法
        if len(signatures) > 30:
            return self._find_chain_greedy(signatures)
        
        # 对于中小规模，使用优化的图搜索
        return self._find_chain_optimized(signatures)
    
    def _find_chain_greedy(self, signatures: List[Dict]) -> List[int]:
        """快速贪心链查找"""
        console.print(f"[yellow]  使用贪心算法处理 {len(signatures)} 个函数[/yellow]")
        
        # 构建简化的兼容性映射
        compatibility = {}
        for i, sig1 in enumerate(signatures):
            compatibility[i] = []
            output_type = self._normalize_type(sig1['return_type'])
            
            for j, sig2 in enumerate(signatures):
                if i != j and sig2['params']:
                    input_type = self._normalize_type(sig2['params'][0])
                    if self._types_compatible(output_type, input_type):
                        compatibility[i].append(j)
        
        # 贪心构建链
        best_chain = []
        
        # 尝试多个起始点
        start_candidates = sorted(compatibility.keys(), 
                                key=lambda x: len(compatibility[x]), reverse=True)[:5]
        
        for start in start_candidates:
            chain = self._greedy_build_chain(compatibility, start)
            if len(chain) > len(best_chain):
                best_chain = chain
        
        if best_chain:
            console.print(f"[green]✅ 贪心算法找到链: {len(best_chain)} 个函数[/green]")
        
        return best_chain
    
    def _greedy_build_chain(self, compatibility: Dict[int, List[int]], start: int) -> List[int]:
        """贪心构建链"""
        chain = [start]
        visited = {start}
        current = start
        
        while True:
            # 找到下一个最佳候选
            candidates = [node for node in compatibility[current] if node not in visited]
            if not candidates:
                break
            
            # 选择连接性最好的下一个节点
            next_node = max(candidates, key=lambda x: len(compatibility[x]))
            chain.append(next_node)
            visited.add(next_node)
            current = next_node
        
        return chain
    
    def _find_chain_optimized(self, signatures: List[Dict]) -> List[int]:
        """优化的中规模链查找"""
        # 构建函数兼容性图
        compatibility_graph = self._build_compatibility_graph(signatures)
        
        # 快速策略搜索
        best_chain = []
        max_attempts = 10  # 减少尝试次数
        
        for attempt in range(max_attempts):
            # 随机选择策略和起始点
            strategy = random.choice(['longest_path', 'highest_complexity'])
            chain = self._find_chain_with_strategy(
                compatibility_graph, signatures, strategy, attempt
            )
            
            if len(chain) > len(best_chain):
                best_chain = chain
                console.print(f"[cyan]  找到更长链: {len(chain)} 个函数[/cyan]")
            
            # 早期退出条件
            if len(chain) >= len(signatures) * 0.5:
                break
        
        if len(best_chain) >= 2:
            console.print(f"[green]✅ 成功构建函数链: {len(best_chain)} 个函数[/green]")
            self._log_chain_details(best_chain, signatures)
        
        return best_chain
    
    def _build_compatibility_graph(self, signatures: List[Dict]) -> Dict[int, List[int]]:
        """构建函数兼容性图"""
        graph = {i: [] for i in range(len(signatures))}
        
        for i, sig1 in enumerate(signatures):
            for j, sig2 in enumerate(signatures):
                if i != j and self._can_functions_chain(sig1, sig2):
                    graph[i].append(j)
        
        return graph
    
    def _can_functions_chain(self, func1: Dict, func2: Dict) -> bool:
        """检查两个函数是否可以链接"""
        # func1 的输出类型是否兼容 func2 的第一个输入参数
        if not func2['params']:
            return False
        
        output_type = self._normalize_type(func1['return_type'])
        input_type = self._normalize_type(func2['params'][0])
        
        return self._types_compatible(output_type, input_type)
    
    def _normalize_type(self, type_str: str) -> str:
        """标准化类型字符串"""
        # 移除空格和复杂的泛型
        normalized = type_str.strip()
        
        # 处理 Result 类型
        if normalized.startswith('Result<'):
            # 提取成功类型：Result<i32, String> -> i32
            match = re.search(r'Result<([^,]+),', normalized)
            if match:
                normalized = match.group(1).strip()
        
        # 处理 Option 类型
        if normalized.startswith('Option<'):
            # 提取内部类型：Option<i32> -> i32
            match = re.search(r'Option<([^>]+)>', normalized)
            if match:
                normalized = match.group(1).strip()
        
        # 处理引用类型
        normalized = normalized.replace('&mut ', '').replace('&', '')
        
        return normalized
    
    def _types_compatible(self, output_type: str, input_type: str) -> bool:
        """检查输出类型是否兼容输入类型"""
        if output_type == input_type:
            return True
        
        # 检查预定义的兼容性规则
        if output_type in self.type_compatibility:
            return input_type in self.type_compatibility[output_type]
        
        # 数值类型的灵活兼容
        numeric_types = {'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64', 'f32', 'f64', 'isize', 'usize'}
        if output_type in numeric_types and input_type in numeric_types:
            return True
        
        # 字符串类型的兼容
        string_types = {'String', 'str', '&str', '&String'}
        if output_type in string_types and input_type in string_types:
            return True
        
        return False
    
    def _find_chain_with_strategy(self, graph: Dict[int, List[int]], 
                                signatures: List[Dict], 
                                strategy: str, 
                                iteration: int) -> List[int]:
        """使用特定策略寻找函数链"""
        if strategy == 'longest_path':
            return self._find_longest_path(graph, iteration)
        elif strategy == 'highest_complexity':
            return self._find_highest_complexity_chain(graph, signatures, iteration)
        elif strategy == 'type_diversity':
            return self._find_type_diverse_chain(graph, signatures, iteration)
        else:
            return self._find_longest_path(graph, iteration)
    
    def _find_longest_path(self, graph: Dict[int, List[int]], seed: int) -> List[int]:
        """寻找最长路径 - 优化版本"""
        random.seed(seed)
        best_path = []
        
        # 只尝试度数较高的起始节点，提高效率
        node_degrees = [(node, len(neighbors)) for node, neighbors in graph.items()]
        node_degrees.sort(key=lambda x: x[1], reverse=True)
        
        # 限制尝试的起始节点数量
        max_start_nodes = min(5, len(graph))
        start_nodes = [node for node, _ in node_degrees[:max_start_nodes]]
        
        for start_node in start_nodes:
            path = self._dfs_longest_path_optimized(graph, start_node, set(), [], 0)
            if len(path) > len(best_path):
                best_path = path
            
            # 早期退出：如果找到了很长的路径
            if len(best_path) >= len(graph) * 0.7:
                break
        
        return best_path
    
    def _dfs_longest_path_optimized(self, graph: Dict[int, List[int]], 
                                  current: int, 
                                  visited: Set[int], 
                                  current_path: List[int],
                                  depth: int) -> List[int]:
        """优化的深度优先搜索，带深度限制和剪枝"""
        # 深度限制，防止无限递归
        if depth > 20 or current in visited:
            return current_path
        
        visited.add(current)
        current_path.append(current)
        
        best_path = current_path.copy()
        neighbors = graph[current]
        
        # 按度数排序邻居，优先探索连接性更好的节点
        neighbor_scores = [(n, len(graph[n])) for n in neighbors if n not in visited]
        neighbor_scores.sort(key=lambda x: x[1], reverse=True)
        
        # 限制探索的邻居数量
        max_neighbors = min(3, len(neighbor_scores))
        
        for neighbor, _ in neighbor_scores[:max_neighbors]:
            path = self._dfs_longest_path_optimized(
                graph, neighbor, visited.copy(), current_path.copy(), depth + 1
            )
            if len(path) > len(best_path):
                best_path = path
        
        return best_path
    
    def _find_highest_complexity_chain(self, graph: Dict[int, List[int]], 
                                     signatures: List[Dict], 
                                     seed: int) -> List[int]:
        """寻找复杂度最高的函数链"""
        random.seed(seed)
        
        # 计算每个函数的复杂度权重
        complexity_weights = {}
        for i, sig in enumerate(signatures):
            # 基于参数数量、返回类型复杂度等计算权重
            param_count = len(sig['params'])
            return_complexity = 1 if sig['return_type'] in ['i32', 'bool'] else 2
            complexity_weights[i] = param_count + return_complexity
        
        # 使用加权搜索
        return self._weighted_chain_search(graph, complexity_weights, seed)
    
    def _find_type_diverse_chain(self, graph: Dict[int, List[int]], 
                                signatures: List[Dict], 
                                seed: int) -> List[int]:
        """寻找类型多样性最高的函数链"""
        random.seed(seed)
        
        best_chain = []
        max_diversity = 0
        
        # 尝试多个随机起点
        for _ in range(min(10, len(signatures))):
            start = random.randint(0, len(signatures) - 1)
            chain = self._build_diverse_chain(graph, signatures, start)
            diversity = self._calculate_type_diversity(chain, signatures)
            
            if diversity > max_diversity:
                max_diversity = diversity
                best_chain = chain
        
        return best_chain
    
    def _weighted_chain_search(self, graph: Dict[int, List[int]], 
                             weights: Dict[int, int], 
                             seed: int) -> List[int]:
        """基于权重的链搜索"""
        random.seed(seed)
        
        # 按权重排序起始节点
        sorted_nodes = sorted(weights.keys(), key=lambda x: weights[x], reverse=True)
        
        best_chain = []
        for start_node in sorted_nodes[:5]:  # 只尝试前5个高权重节点
            chain = self._greedy_weighted_path(graph, weights, start_node)
            if len(chain) > len(best_chain):
                best_chain = chain
        
        return best_chain
    
    def _greedy_weighted_path(self, graph: Dict[int, List[int]], 
                            weights: Dict[int, int], 
                            start: int) -> List[int]:
        """贪心算法构建加权路径"""
        path = [start]
        visited = {start}
        current = start
        
        while True:
            # 找到权重最高且未访问的下一个节点
            next_candidates = [(node, weights[node]) for node in graph[current] 
                             if node not in visited]
            
            if not next_candidates:
                break
            
            # 按权重排序，选择最高权重的节点
            next_candidates.sort(key=lambda x: x[1], reverse=True)
            next_node = next_candidates[0][0]
            
            path.append(next_node)
            visited.add(next_node)
            current = next_node
        
        return path
    
    def _build_diverse_chain(self, graph: Dict[int, List[int]], 
                           signatures: List[Dict], 
                           start: int) -> List[int]:
        """构建类型多样化的链"""
        chain = [start]
        visited = {start}
        used_types = {signatures[start]['return_type']}
        current = start
        
        while True:
            # 寻找引入新类型的下一个节点
            best_next = None
            max_new_types = 0
            
            for next_node in graph[current]:
                if next_node not in visited:
                    next_sig = signatures[next_node]
                    new_types = set(next_sig['params'] + [next_sig['return_type']]) - used_types
                    
                    if len(new_types) > max_new_types:
                        max_new_types = len(new_types)
                        best_next = next_node
            
            if best_next is None:
                break
            
            chain.append(best_next)
            visited.add(best_next)
            next_sig = signatures[best_next]
            used_types.update(next_sig['params'] + [next_sig['return_type']])
            current = best_next
        
        return chain
    
    def _calculate_type_diversity(self, chain: List[int], signatures: List[Dict]) -> int:
        """计算链的类型多样性"""
        all_types = set()
        for idx in chain:
            sig = signatures[idx]
            all_types.update(sig['params'] + [sig['return_type']])
        return len(all_types)
    
    def _log_chain_details(self, chain: List[int], signatures: List[Dict]):
        """记录链的详细信息"""
        console.print("[cyan]  链详情:[/cyan]")
        for i, idx in enumerate(chain):
            sig = signatures[idx]
            params_str = ', '.join(sig['params']) if sig['params'] else 'none'
            console.print(f"    {i+1}. {sig['name']}({params_str}) -> {sig['return_type']}")
            
            if i < len(chain) - 1:
                # 显示类型匹配
                next_sig = signatures[chain[i+1]]
                if next_sig['params']:
                    output_type = self._normalize_type(sig['return_type'])
                    input_type = self._normalize_type(next_sig['params'][0])
                    console.print(f"       ↓ {output_type} → {input_type}")
        
        # 计算并显示统计信息
        diversity = self._calculate_type_diversity(chain, signatures)
        total_params = sum(len(signatures[idx]['params']) for idx in chain)
        console.print(f"  类型多样性: {diversity} 种不同类型")
        console.print(f"  总参数数: {total_params}")
        console.print(f"  链长度: {len(chain)}")
    
    def _extract_function_signature(self, code: str) -> Dict[str, any]:
        """提取函数签名 - 增强版本"""
        # 更复杂的签名提取，支持泛型和复杂类型
        match = re.search(r'fn\s+(\w+)(?:<[^>]*>)?\s*\((.*?)\)\s*(?:->\s*([^{]+))?', code, re.DOTALL)
        if match:
            name = match.group(1)
            params_str = match.group(2) or ""
            return_type = (match.group(3) or "()").strip()
            
            # 更智能的参数解析
            params = self._parse_function_parameters(params_str)
            
            return {
                "name": name,
                "params": params,
                "return_type": return_type,
                "complexity": self._calculate_signature_complexity(params, return_type)
            }
        
        return {"name": "unknown", "params": [], "return_type": "()", "complexity": 0}
    
    def _parse_function_parameters(self, params_str: str) -> List[str]:
        """解析函数参数"""
        if not params_str.strip():
            return []
        
        params = []
        current_param = ""
        paren_depth = 0
        angle_depth = 0
        
        for char in params_str:
            if char == '(' and angle_depth == 0:
                paren_depth += 1
            elif char == ')' and angle_depth == 0:
                paren_depth -= 1
            elif char == '<' and paren_depth == 0:
                angle_depth += 1
            elif char == '>' and paren_depth == 0:
                angle_depth -= 1
            elif char == ',' and paren_depth == 0 and angle_depth == 0:
                # 找到参数分隔符
                if ':' in current_param:
                    param_type = current_param.split(':')[1].strip()
                    params.append(param_type)
                current_param = ""
                continue
            
            current_param += char
        
        # 处理最后一个参数
        if current_param.strip() and ':' in current_param:
            param_type = current_param.split(':')[1].strip()
            params.append(param_type)
        
        return params
    
    def _calculate_signature_complexity(self, params: List[str], return_type: str) -> int:
        """计算签名复杂度"""
        complexity = 0
        
        # 参数数量贡献
        complexity += len(params)
        
        # 类型复杂度
        for param in params + [return_type]:
            if any(t in param for t in ['Vec', 'HashMap', 'Option', 'Result']):
                complexity += 2
            elif any(t in param for t in ['&', 'mut', '<', '>']):
                complexity += 1
        
        return complexity
    
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