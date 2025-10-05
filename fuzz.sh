#!/bin/bash
cargo build --release
rm -rf out
export RUST_LOG=info

mkdir -p out
for i in {1..100}; do
    seed=$(date +%s%N)
    target/release/generate $seed > out/$seed.mir
    target/release/difftest out/$seed.mir
done