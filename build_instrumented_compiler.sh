#!/bin/bash
set -e

echo "=== Building Instrumented Rust Compiler for MIR Transform Coverage ==="

# Configuration
RUST_SRC="/users/hangye/compiler/rust-nightly"
BUILD_DIR="$RUST_SRC/build"
OUTPUT_DIR="/users/hangye/compiler/rustc_cov"

echo "Cleaning previous build..."
cd "$RUST_SRC"
rm -rf "$BUILD_DIR"

echo "Configuring build with coverage instrumentation..."

# Create config.toml with coverage settings
cat > config.toml << 'EOF'
# Configuration for building Rust with coverage instrumentation

[build]
# Build configuration
build-stage = 2
doc-stage = 2
compiler-docs = false
build-dir = "build"

# Enable debug info and instrumentation
debug = true
debug-assertions = true
debug-logging = true

# Use system LLVM if available, otherwise use bundled
llvm-config = "/usr/bin/llvm-config-15"

[rust]
# Enable coverage instrumentation
codegen-units-std = 1
incremental = false
debug = true
debug-assertions = true
overflow-checks = true

# Enable LLVM coverage
use-lld = false
thin-lto = false
rpath = true

[target.x86_64-unknown-linux-gnu]
# Target-specific configuration
cc = "clang-15"
cxx = "clang++-15"
linker = "clang-15"

[llvm]
# LLVM configuration for coverage
optimize = false
release-debuginfo = true
assertions = true
ccache = false
static-libstdcpp = false

# Enable coverage instrumentation in LLVM
link-shared = false
use-libcxx = false
download-ci-llvm = false

EOF

echo "Starting build with coverage instrumentation..."

# Set environment variables for coverage instrumentation
export RUSTFLAGS_BOOTSTRAP="-C instrument-coverage"
export RUSTFLAGS_NOT_BOOTSTRAP="-C instrument-coverage"
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-C instrument-coverage"

# Additional coverage-specific environment variables
export LLVM_PROFILE_FILE="rustc_coverage_%p_%m.profraw"
export RUSTC_BOOTSTRAP=1

echo "Building stage0..."
python3 x.py build --stage 0 library/std

echo "Building stage1 with instrumentation..."
python3 x.py build --stage 1 --keep-stage 0 \
    compiler/rustc_driver \
    compiler/rustc_mir_transform

echo "Building stage2 with full instrumentation..."
python3 x.py build --stage 2 --keep-stage 1 \
    compiler/rustc_driver \
    compiler/rustc_mir_transform

echo "Installing instrumented compiler..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Copy the instrumented compiler
cp -r "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/"* "$OUTPUT_DIR/"

echo "Creating wrapper script..."
cat > "$OUTPUT_DIR/bin/rustc_instrumented" << 'EOF'
#!/bin/bash

# Wrapper script for instrumented rustc
export LLVM_PROFILE_FILE="${LLVM_PROFILE_FILE:-rustc_coverage_%p_%m.profraw}"
export RUSTC_BOOTSTRAP=1

# Run the actual rustc with coverage
exec "$(dirname "$0")/rustc" "$@"
EOF

chmod +x "$OUTPUT_DIR/bin/rustc_instrumented"

echo "=== Build Complete ==="
echo "Instrumented compiler installed at: $OUTPUT_DIR"
echo "Wrapper script: $OUTPUT_DIR/bin/rustc_instrumented"
echo "To use: export RUSTC=$OUTPUT_DIR/bin/rustc_instrumented"