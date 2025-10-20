## Personal Notes
We support all primitive integer and floating point types, bool, char, arrays, tuples, raw pointers, shared and mutable references, structs, and enums. This indicates the all types in MIR.

For a future extension of Rustlantis it could be interesting to consider generating generic code.

It is possible to uphold this validity constraint using the known value information, but we have not yet implemented this fine-grained filtering. For now, we simply prevent transmutations to bool and char types.


# User Guide for Rustlantis
Every changes are made under : https://github.com/Simon-code2077/rustlantis/tree/dev
For a general Fuzz test, run:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
rustup default nightly
cargo install grcov
./fuzz.sh
```
For a coverage test, run:
```
# install a customized rustc compiler
git clone --depth 1 https://github.com/rust-lang/rust rust-nightly
cd rust-nightly && echo "build.profiler = true" > config.toml
cd /path/to/rust-nightly/src/bootstrap
grep -Irnsw "rust_new_symbol_mangling"
# Usually in file src/bootstrap/src/core/builder/cargo.rs, Find the function "fn cargo", add the following assignments (apply coverage flags to all modules except std):
if mode != Mode::Std {
    rustflags.arg("-Cinstrument-coverage");
} 
# Change the defaults install location under src/bootstrap/src/core/build_steps/install.rs for access permission
let prefix = default_path(&builder.config.prefix, "path/to/install");
let sysconfdir = prefix.join(default_path(&builder.config.sysconfdir, "path/to/etc"));
# Build the rustc bins
./x build && ./x install
# Run the coverage measurement
./cov.sh
```

# Rustlantis
A Rust Mid-level Intermediate Representation fuzzer

It can generate [custom MIR](https://doc.rust-lang.org/std/intrinsics/mir/index.html) programs containing:
- All primitive integer and floating point types, `bool`, `char`, arrays,
tuples, references, raw pointers, structs, and enums.
- Functions containing multiple basic blocks
- Terminators: `Goto`, `Return`, `SwitchInt` (`match`), `Call`.
- Intrinsic functions: `arith_offset` (for pointer arithmetics), `transmute`,
`bswap`, `fmaf64`.
- Operators: all arithmetic, logical and bitwise operations on integers
and floating points, and checked arithmetic (Add, Sub, Mul) on integers
- All primitive literal expressions, as well as tuple, array, and struct
aggregate expressions
- Creating references and raw pointers, and dereferencing them
- Casts between integers, floating points, `char`, and `bool`

Generated programs are terminating, UB-free, and deterministic. A discrepancy between testing backends
always indicate a bug in them (or a bug in Rustlantis).

## Requirements
- Rust nightly
- rustup

## Config
Install Miri and Cranelift with Rustup `rustup component add miri rustc-codegen-cranelift-preview`, then copy `config.toml.example` to `config.toml`

## Usage

To generate and difftest one seed, run

```bash
./fuzz-one.sh <seed>
```

A program will be generated to `$TMPDIR` and tested. If difftest passes (no bug), it will exit with 0. If difftest spots a difference between testing backends, it will exit with 1 and save the reproduction file to `./repros/`.

To generate a program only, run `generate`
```
Usage: generate [OPTIONS] <seed>

Arguments:
  <seed>  generation seed

Options:
  -d, --debug                      generate a program where values are printed instead of hashed (slow)
      --call-syntax <call-syntax>  switch between different versions of Call syntaxes [default: v4] [possible values: v1, v2, v3, v4]
  -h, --help                       Print help
  -V, --version                    Print version
```

To difftest an existing program, run `difftest`
```
Usage: difftest <file>

Arguments:
  <file>  

Options:
  -h, --help  Print help
```

## Quirks
- Cranelift not supported on AArch64 macOS: https://github.com/bjorn3/rustc_codegen_cranelift/issues/1248
- `rustc_codegen_gcc` can be used as a backend, but it doesn't support enough language features yet to be usable

