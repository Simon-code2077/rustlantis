## Personal Notes
We support all primitive integer and floating point types, bool, char, arrays, tuples, raw pointers, shared and mutable references, structs, and enums. This indicates the all types in MIR.

For a future extension of Rustlantis it could be interesting to consider generating generic code.

It is possible to uphold this validity constraint using the known value information, but we have not yet implemented this fine-grained filtering. For now, we simply prevent transmutations to bool and char types.


# User Guide for Rustlantis
Every changes are made under : https://github.com/Simon-code2077/rustlantis/tree/dev
For a general Fuzz test, run:

```
sudo apt update
sudo apt install pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## select 1 during install
. "$HOME/.cargo/env" 
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
# Usually in file src/bootstrap/src/core/builder/cargo.rs, it belongs to function "fn cargo". Find the function, add the following assignments somewhere nearby (apply coverage flags to all modules except std):
if mode != Mode::Std {
    rustflags.arg("-Cinstrument-coverage");
} 
# Change the defaults install location under src/bootstrap/src/core/build_steps/install.rs for access permission
let prefix = default_path(&builder.config.prefix, "path/to/install");
let sysconfdir = prefix.join(default_path(&builder.config.sysconfdir, "path/to/etc"));
# Build the rustc bins
cd path/to/rust-nightly
./x build && ./x install
# It usually takes 1 hour and 50GB storage to install
# Run the coverage measurement
# Don't forget to change the path in bash script to your own path
./cov.sh
```

