import json
from pathlib import Path
import sys

def process_rs_file(file_path: Path) -> dict:
    """
    读取单个 .rs 文件，返回 JSON 对象
    """
    code = file_path.read_text(encoding='utf-8')
    return {
        "original_name": file_path.stem,
        "original_code": code,
        "source_file": str(file_path)
    }

def process_directory(input_dir: str, output_file: str):
    input_path = Path(input_dir)
    if not input_path.is_dir():
        print(f"错误: {input_dir} 不是目录")
        return

    rs_files = list(input_path.rglob("*.rs"))  # 递归查找所有 .rs
    if not rs_files:
        print(f"未找到任何 .rs 文件")
        return

    with open(output_file, 'w', encoding='utf-8') as fout:
        for rs_file in rs_files:
            try:
                data = process_rs_file(rs_file)
                fout.write(json.dumps(data, ensure_ascii=False) + "\n")
            except Exception as e:
                print(f"[ERROR] 处理 {rs_file} 出错: {e}")

    print(f"处理完成，共写入 {len(rs_files)} 行到 {output_file}")


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("用法: python3 rs_to_jsonl.py <输入目录> <输出文件.jsonl>")
        sys.exit(1)

    input_dir = sys.argv[1]
    output_file = sys.argv[2]

    process_directory(input_dir, output_file)
