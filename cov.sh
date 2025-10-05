
#!/bin/bash
set -e

RUSTC=/path/to/install/rustc
SRC_DIR=out
COV_DIR=coverage
rm -rf "$COV_DIR"
rm -rf cov.info
mkdir -p "$COV_DIR"

for file in "$SRC_DIR"/*.rs; do
    name=$(basename "$file" .rs)
    echo "Compiling $file ..."
    LLVM_PROFILE_FILE="$COV_DIR/${name}-%p-%m.profraw" \
        "$RUSTC" "$file" --emit=llvm-bc -o "$SRC_DIR/${name}.bc"
done

# Compile multiple test files to get broader coverage
# echo "Compiling test.rs..."
# LLVM_PROFILE_FILE="$COV_DIR/100-%p-%m.profraw" \
#     "$RUSTC" out/100.mir --emit=llvm-bc -o "100.bc" 

# echo "Compiling complex_test.rs..."
# LLVM_PROFILE_FILE="$COV_DIR/complex-%p-%m.profraw" \
#     "$RUSTC" complex_test.rs --emit=llvm-bc -o "complex_test.bc"

# # Also compile with different optimization levels to trigger more code paths
# echo "Compiling with optimizations..."
# LLVM_PROFILE_FILE="$COV_DIR/opt-%p-%m.profraw" \
#     "$RUSTC" -O complex_test.rs -o "complex_test_opt" 

echo "Processing coverage data..."
grcov coverage/*.profraw   \
    -s path/to/source/rust-nightly   \
    -b path/to/install   \
    --llvm-path path/to/source/rust-nightly/build/x86_64-unknown-linux-gnu/ci-llvm/bin   \
    --keep-only="compiler/rustc_mi*" \
    -t lcov   -o cov.info
lcov --summary cov.info

# grcov coverage/*.profraw -s /users/hangye/compiler/rust-nightly -b /users/hangye/compiler/rust-nightly/build/x86_64-unknown-linux-gnu/stage1-tools-bin/rustdoc_tool_binary --llvm-path /users/hangye/compiler/rust-nightly/build/x86_64-unknown-linux-gnu/ci-llvm/bin -t files