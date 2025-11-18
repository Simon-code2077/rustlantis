#!/usr/bin/env python3
"""
简化的覆盖率测试脚本
先测试基础覆盖率收集，再分析区域级覆盖率
"""

import os
import subprocess
import json
import logging
from pathlib import Path

# 设置日志
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class SimpleCoverageTest:
    def __init__(self):
        self.rustc_path = os.environ.get('RUSTC_PATH', '/users/hangye/.cargo/bin/rustc')
        self.source_dir = Path('./synthesized')
        self.coverage_dir = Path('./coverage')
        self.output_dir = Path('./coverage_output')
        
        # 创建必要的目录
        self.source_dir.mkdir(exist_ok=True)
        self.coverage_dir.mkdir(exist_ok=True)
        self.output_dir.mkdir(exist_ok=True)
        
    def create_test_files(self):
        """创建简单的测试文件"""
        logger.info("创建测试文件...")
        
        # 基础测试文件
        basic_test = """
// 基础函数测试
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn conditional_func(x: i32) -> i32 {
    if x > 0 {
        x * 2
    } else if x < 0 {
        x * -1
    } else {
        0
    }
}

fn main() {
    let result1 = add(5, 3);
    let result2 = multiply(4, 6);
    let result3 = conditional_func(10);
    println!("Results: {}, {}, {}", result1, result2, result3);
}
"""
        
        # 泛型测试文件
        generic_test = """
// 泛型和trait测试
use std::fmt::Debug;

trait Processable {
    fn process(&self) -> String;
}

struct Container<T> where T: Debug {
    value: T,
}

impl<T: Debug> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T: Debug> Processable for Container<T> {
    fn process(&self) -> String {
        format!("{:?}", self.value)
    }
}

fn generic_function<T: Debug>(item: T) -> Container<T> {
    Container::new(item)
}

fn main() {
    let int_container = generic_function(42);
    let str_container = generic_function("hello");
    
    println!("{}", int_container.process());
    println!("{}", str_container.process());
}
"""
        
        # 错误处理测试文件
        error_test = """
// 错误处理测试
use std::fs::File;
use std::io::Read;

fn read_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // 测试正常情况
    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 测试错误情况
    match divide(10.0, 0.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 尝试读取文件
    match read_file_content("nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }
}
"""
        
        # 写入测试文件
        test_files = [
            ("basic_test.rs", basic_test),
            ("generic_test.rs", generic_test),
            ("error_test.rs", error_test),
        ]
        
        for filename, content in test_files:
            file_path = self.source_dir / filename
            with open(file_path, 'w') as f:
                f.write(content)
            logger.info(f"创建测试文件: {file_path}")
        
        return [self.source_dir / filename for filename, _ in test_files]
    
    def compile_with_coverage(self, source_files):
        """使用覆盖率标志编译源文件 - 基于cov.sh的方法"""
        logger.info("编译源文件并生成覆盖率数据...")
        
        compiled_files = []
        
        for source_file in source_files:
            filename = source_file.stem
            profile_file = self.coverage_dir / f"{filename}-%p-%m.profraw"
            
            logger.info(f"编译 {source_file}...")
            
            # 设置环境变量
            env = os.environ.copy()
            env['LLVM_PROFILE_FILE'] = str(profile_file)
            
            # 使用cov.sh中的rustc路径和方法
            rustc_path = "/users/hangye/compiler/bin/rustc"
            
            # 编译命令 - 生成llvm-bc以触发编译器覆盖率
            compile_cmd = [
                rustc_path,
                str(source_file),
                '--emit=llvm-bc',
                '-o', str(source_file.parent / f"{filename}.bc")
            ]
            
            try:
                result = subprocess.run(
                    compile_cmd,
                    env=env,
                    capture_output=True,
                    text=True,
                    timeout=30
                )
                
                if result.returncode == 0:
                    logger.info(f"编译成功: {filename}")
                    compiled_files.append({
                        'source': source_file,
                        'bc_file': source_file.parent / f"{filename}.bc",
                        'profile_file': profile_file
                    })
                else:
                    logger.error(f"编译失败 {filename}: {result.stderr}")
                    
            except Exception as e:
                logger.error(f"编译异常 {filename}: {e}")
        
        return compiled_files
    
    def process_coverage_data(self):
        """使用grcov处理覆盖率数据 - 基于cov.sh方法"""
        logger.info("处理覆盖率数据...")
        
        # 查找所有profraw文件
        profraw_files = list(self.coverage_dir.glob("*.profraw"))
        
        if not profraw_files:
            logger.error("没有找到覆盖率数据文件")
            return None
        
        logger.info(f"找到 {len(profraw_files)} 个覆盖率数据文件")
        
        # 使用cov.sh中的grcov命令
        output_file = self.output_dir / "cov.info"
        
        # 直接获取profraw文件列表，而不是使用通配符
        profraw_files = list(self.coverage_dir.glob("*.profraw"))
        if not profraw_files:
            logger.error("没有找到profraw文件")
            return None
        
        # 构建grcov命令，直接传递文件列表
        grcov_cmd = [
            'grcov'
        ] + [str(f) for f in profraw_files] + [
            '-s', '/users/hangye/compiler/rust-nightly',
            '-b', '/users/hangye/compiler/bin', 
            '--llvm-path', '/users/hangye/compiler/rust-nightly/build/x86_64-unknown-linux-gnu/ci-llvm/bin',
            '--keep-only=compiler/*',
            '-t', 'lcov',
            '-o', str(output_file)
        ]
        
        try:
            logger.info(f"运行grcov命令: {' '.join(grcov_cmd)}")
            result = subprocess.run(
                grcov_cmd,
                capture_output=True,
                text=True,
                timeout=600
            )
            
            if result.returncode == 0:
                logger.info("grcov处理成功")
                logger.info(f"覆盖率报告生成: {output_file}")
                return output_file
            else:
                logger.error(f"grcov处理失败: {result.stderr}")
                return None
                
        except Exception as e:
            logger.error(f"grcov处理异常: {e}")
            return None
    
    def analyze_coverage_regions(self, coverage_file):
        """分析区域级覆盖率 - 基于region_analysis.py的方法"""
        if not coverage_file or not Path(coverage_file).exists():
            logger.error("覆盖率文件不存在")
            return None
        
        logger.info("分析区域级覆盖率...")
        
        try:
            region_analysis = {
                'files': {},
                'summary': {
                    'total_files': 0,
                    'total_functions': 0,
                    'covered_functions': 0,
                    'total_lines': 0,
                    'covered_lines': 0,
                    'uncovered_functions': [],
                    'high_value_files': []
                }
            }
            
            current_file = None
            
            with open(coverage_file, 'r') as f:
                for line in f:
                    line = line.strip()
                    
                    if line.startswith('SF:'):  # Source File
                        filename = line[3:]
                        current_file = {
                            'filename': filename,
                            'functions': [],
                            'lines': {},
                            'stats': {
                                'functions_found': 0,
                                'functions_hit': 0,
                                'lines_found': 0,
                                'lines_hit': 0
                            }
                        }
                        region_analysis['summary']['total_files'] += 1
                    
                    elif current_file and line.startswith('FN:'):  # Function definition
                        parts = line[3:].split(',', 1)
                        if len(parts) >= 2:
                            line_num, func_name = parts[0], parts[1]
                            current_file['functions'].append({
                                'name': func_name,
                                'line': int(line_num),
                                'hit_count': 0,
                                'covered': False
                            })
                    
                    elif current_file and line.startswith('FNDA:'):  # Function data
                        parts = line[5:].split(',', 1)
                        if len(parts) >= 2:
                            hit_count, func_name = int(parts[0]), parts[1]
                            # 更新对应函数的命中次数
                            for func in current_file['functions']:
                                if func['name'] == func_name:
                                    func['hit_count'] = hit_count
                                    func['covered'] = hit_count > 0
                                    break
                    
                    elif current_file and line.startswith('FNF:'):  # Functions Found
                        current_file['stats']['functions_found'] = int(line[4:])
                    
                    elif current_file and line.startswith('FNH:'):  # Functions Hit
                        current_file['stats']['functions_hit'] = int(line[4:])
                    
                    elif current_file and line.startswith('DA:'):  # Line data
                        parts = line[3:].split(',')
                        if len(parts) >= 2:
                            line_num, hit_count = int(parts[0]), int(parts[1])
                            current_file['lines'][line_num] = hit_count
                    
                    elif current_file and line.startswith('LF:'):  # Lines Found
                        current_file['stats']['lines_found'] = int(line[3:])
                    
                    elif current_file and line.startswith('LH:'):  # Lines Hit  
                        current_file['stats']['lines_hit'] = int(line[3:])
                    
                    elif line == 'end_of_record' and current_file:
                        # 完成当前文件的处理
                        self.process_file_regions(current_file, region_analysis)
                        region_analysis['files'][current_file['filename']] = current_file
                        current_file = None
            
            # 计算总体摘要
            self.calculate_region_summary(region_analysis)
            
            # 保存分析结果
            analysis_file = self.output_dir / "region_analysis.json"
            with open(analysis_file, 'w') as f:
                json.dump(region_analysis, f, indent=2, default=str)
            
            logger.info(f"区域分析完成，结果保存到: {analysis_file}")
            
            # 打印详细区域分析
            self.print_region_coverage_summary(region_analysis)
            
            return region_analysis
            
        except Exception as e:
            logger.error(f"分析覆盖率异常: {e}")
            return None
    
    def process_file_regions(self, file_data, region_analysis):
        """处理单个文件的区域信息"""
        filename = file_data['filename']
        summary = region_analysis['summary']
        
        # 更新总体统计
        summary['total_functions'] += file_data['stats']['functions_found']
        summary['covered_functions'] += file_data['stats']['functions_hit']
        summary['total_lines'] += file_data['stats']['lines_found']
        summary['covered_lines'] += file_data['stats']['lines_hit']
        
        # 识别未覆盖的函数
        for func in file_data['functions']:
            if not func['covered']:
                summary['uncovered_functions'].append({
                    'file': filename,
                    'function': func['name'],
                    'line': func['line']
                })
        
        # 识别高价值文件（重要的编译器组件）
        high_value_patterns = [
            'rustc_middle/src/ty/',
            'rustc_middle/src/mir/',
            'rustc_mir_build/',
            'rustc_mir_transform/',
            'rustc_mir_dataflow/'
        ]
        
        for pattern in high_value_patterns:
            if pattern in filename:
                coverage_rate = (file_data['stats']['functions_hit'] / 
                               file_data['stats']['functions_found']) if file_data['stats']['functions_found'] > 0 else 0
                
                summary['high_value_files'].append({
                    'file': filename,
                    'pattern': pattern,
                    'functions_found': file_data['stats']['functions_found'],
                    'functions_hit': file_data['stats']['functions_hit'],
                    'function_coverage': coverage_rate,
                    'lines_found': file_data['stats']['lines_found'],
                    'lines_hit': file_data['stats']['lines_hit'],
                    'line_coverage': (file_data['stats']['lines_hit'] / 
                                    file_data['stats']['lines_found']) if file_data['stats']['lines_found'] > 0 else 0
                })
                break
    
    def calculate_region_summary(self, region_analysis):
        """计算区域摘要统计"""
        summary = region_analysis['summary']
        
        # 计算覆盖率百分比
        if summary['total_functions'] > 0:
            summary['function_coverage_percent'] = (summary['covered_functions'] / summary['total_functions']) * 100
        else:
            summary['function_coverage_percent'] = 0.0
            
        if summary['total_lines'] > 0:
            summary['line_coverage_percent'] = (summary['covered_lines'] / summary['total_lines']) * 100
        else:
            summary['line_coverage_percent'] = 0.0
        
        # 按覆盖率对高价值文件排序
        summary['high_value_files'].sort(key=lambda x: x['function_coverage'], reverse=True)

    def print_region_coverage_summary(self, region_analysis):
        """打印区域级覆盖率摘要"""
        summary = region_analysis['summary']
        
        logger.info("=" * 80)
        logger.info("RUG区域级覆盖率分析报告")
        logger.info("=" * 80)
        
        logger.info(f"\n📊 总体覆盖率统计:")
        logger.info(f"   文件总数: {summary['total_files']}")
        logger.info(f"   函数总数: {summary['total_functions']}")
        logger.info(f"   已覆盖函数: {summary['covered_functions']}")
        logger.info(f"   函数覆盖率: {summary['function_coverage_percent']:.1f}%")
        logger.info(f"   行总数: {summary['total_lines']}")
        logger.info(f"   已覆盖行: {summary['covered_lines']}")
        logger.info(f"   行覆盖率: {summary['line_coverage_percent']:.1f}%")
        
        logger.info(f"\n🎯 高价值文件覆盖率分析:")
        if summary['high_value_files']:
            for i, file_info in enumerate(summary['high_value_files'][:10], 1):
                logger.info(f"   {i}. {file_info['file'].split('/')[-1]}")
                logger.info(f"      模式: {file_info['pattern']}")
                logger.info(f"      函数覆盖: {file_info['functions_hit']}/{file_info['functions_found']} ({file_info['function_coverage']*100:.1f}%)")
                logger.info(f"      行覆盖: {file_info['lines_hit']}/{file_info['lines_found']} ({file_info['line_coverage']*100:.1f}%)")
        else:
            logger.info("   未找到高价值文件")
        
        logger.info(f"\n❌ 未覆盖函数区域 (前10个):")
        uncovered_count = len(summary['uncovered_functions'])
        logger.info(f"   总计未覆盖函数: {uncovered_count}")
        
        for i, func_info in enumerate(summary['uncovered_functions'][:10], 1):
            file_short = func_info['file'].split('/')[-1]
            func_short = func_info['function'][:50] + "..." if len(func_info['function']) > 50 else func_info['function']
            logger.info(f"   {i}. {file_short}:{func_info['line']} - {func_short}")
        
        if uncovered_count > 10:
            logger.info(f"   ... 还有 {uncovered_count - 10} 个未覆盖函数")
        
        logger.info("=" * 80)

    def run_complete_test(self):
        """运行完整的覆盖率测试"""
        logger.info("开始完整的覆盖率测试...")
        
        try:
            # 1. 创建测试文件
            source_files = self.create_test_files()
            
            # 2. 编译生成覆盖率 (使用cov.sh方法)
            compiled_files = self.compile_with_coverage(source_files)
            
            if not compiled_files:
                logger.error("没有成功编译的文件")
                return False
            
            # 3. 处理覆盖率数据 (编译过程已经生成了profraw文件)
            coverage_file = self.process_coverage_data()
            
            if not coverage_file:
                logger.error("覆盖率数据处理失败")
                return False
            
            # 4. 分析区域级覆盖率
            analysis = self.analyze_coverage_regions(coverage_file)
            
            if analysis:
                logger.info("覆盖率测试完成！")
                return True
            else:
                logger.error("覆盖率分析失败")
                return False
                
        except Exception as e:
            logger.error(f"测试过程异常: {e}")
            return False

def main():
    """主函数"""
    # 检查环境
    rustc_path = os.environ.get('RUSTC_PATH')
    if not rustc_path:
        print("错误: RUSTC_PATH 环境变量未设置")
        return 1
    
    if not Path(rustc_path).exists():
        print(f"错误: rustc 不存在于路径: {rustc_path}")
        return 1
    
    # 运行测试
    test = SimpleCoverageTest()
    success = test.run_complete_test()
    
    return 0 if success else 1

if __name__ == '__main__':
    exit(main())