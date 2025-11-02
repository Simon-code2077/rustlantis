import sys
import json
import tempfile
import subprocess
import random
import re
from pathlib import Path


def mutate_code_type(code: str) -> str:
    # type mutation: i32 -> i64
    code = code.replace("i32", "i64")

    ops = ['+', '-', '*', '/']

    # pattern to match binary operations
    pattern = re.compile(
        r'(?<!-)\b([^\s\+\-\*/]+)\b\s*([\+\-\*/])\s*\b([^\s\+\-\*/]+)\b(?!>)',
        re.MULTILINE | re.DOTALL
    )

    def replace_op(match):
        # print("Matched operation")
        left, op, right = match.groups()
        new_op = random.choice(ops)
        swap = False
        if random.random() < 0.5:
            left, right = right.strip(), left.strip()
            swap = True
        replaced_expr = f"{left} {new_op} {right}"
        # if replaced_expr != match.group(0):
            # print(f"[replace_op] '{match.group(0)}' -> '{replaced_expr}' (swap={swap})")
        return replaced_expr

    code = pattern.sub(replace_op, code)
    literal_pattern = re.compile(r'\b(-?\d+(\.\d+)?)\b')
    def replace_literal(match):
        old_val = match.group(1)
        if '.' in old_val:
            new_val = round(random.uniform(0, 100), 2)
        else:
            new_val = random.randint(0, 1000)
        # print(f"[replace_literal] {old_val} -> {new_val}")
        return str(new_val)

    code = literal_pattern.sub(replace_literal, code)
    return code

def mutate_loops(code: str) -> str:
    """
    - for i in a..b
    - for i in a..=b
    """
    # for i in 0..=n {
    for_pattern = re.compile(
        r'for\s+(\w+)\s+in\s+(\d+)\s*(\.\.=|\.\.)\s*(\w+)\s*\{',
        re.MULTILINE
    )

    # let mut i = a; while i < b { ... i += 1; }
    while_pattern = re.compile(
        r'let\s+mut\s+(\w+)\s*=\s*(\d+);\s*while\s+\1\s*([<]=?)\s*(\w+)\s*\{',
        re.MULTILINE | re.DOTALL
    )

    # for -> while
    def for_to_while(match):
        var, start, dots, end = match.groups()
        if random.random() < 0.5:
            inclusive = dots == "..="
            cond = "<=" if inclusive else "<"
            if var == "_":
                var = f"_tmp{random.randint(0,999)}"
            print(f"[for->while] for {var} in {start}{dots}{end}")
            return f"let mut {var} = {start};\nwhile {var} {cond} {end} {{"
        else:
            return match.group(0)

    code = for_pattern.sub(for_to_while, code)

    # while -> for
    def while_to_for(match):
        var, start, comp, end = match.groups()
        if random.random() < 0.5:
            inclusive = comp == "<="
            dots = "..=" if inclusive else ".."
            print(f"[while->for] while {var} {comp} {end}")
            return f"for {var} in {start}{dots}{end} {{"
        else:
            return match.group(0)

    code = while_pattern.sub(while_to_for, code)

    return code

def mutate_code(code: str) -> str:
    code = mutate_code_type(code)
    code = mutate_loops(code)
    return code

def cargo_check_rust_code(code: str) -> bool:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        src_path = tmp_path / "src"
        src_path.mkdir()
        lib_file = src_path / "lib.rs"
        lib_file.write_text(code)


        subprocess.run(["cargo", "init", "--lib", "--quiet"], cwd=tmp_path, check=True)


        result = subprocess.run(
            ["cargo", "check", "--quiet"],
            cwd=tmp_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )
        if result.returncode != 0:
            print("--------- Cargo 输出 ---------")
            print(result.stderr.decode())
            return False
        return True


def process_json_file(input_file, output_file):
    # Read input JSONL, mutate code, validate with cargo check, write valid ones to output JSONL
    with open(input_file, 'r') as fin, open(output_file, 'a') as fout:
        for line_num, line in enumerate(fin, start=1):
            if not line.strip():
                continue
            try:
                data = json.loads(line)
                code = data.get("original_code", "")
                mutated = mutate_code(code)

                if cargo_check_rust_code(mutated):
                    data["mutated_code"] = mutated
                    fout.write(json.dumps(data, ensure_ascii=False) + "\n")
                    fout.flush()
                    print(f"[OK] Line {line_num} Passed validation and written to output.")
                else:
                    print(f"[FAILED] Line {line_num} Failed validation.\n")
                    print("----- Failed Rust Code -----")
                    print(mutated)
                    print("--------------------------")
                    sys.exit(1)

            except Exception as e:
                print(f"[EXCEPTION] Line {line_num}: {e}")
                sys.exit(1)


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("Usage: python3 easy_mutation.py <input.jsonl> <output.jsonl>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]
    process_json_file(input_file, output_file)
