# Playing with Rust

## Project Requirement
I needed a script to parse through a directory + subdirectories to find all swift files and see if there are any string quotes in the files.

## Download and Install Process
```bash
# Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source cargo env
source $HOME/.cargo/env
```

## Compile
```bash
# Compile and run immediately against a directory
cargo run -- /path/to/your/ios/project

# Compile for release. Release binary is compiled to `./target/release/{program_name}`
cargo build --release
```