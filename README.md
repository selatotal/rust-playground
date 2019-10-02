# rust-playground
Rust repository for learning purposes.
Step-by-step of "The Rust Programming Language" Book, available in https://doc.rust-lang.org/stable/book

# Installation (Linux)

Use "rustop" to do the job
```bash
curl https://sh.rustup.rs -sSf | sh
```

Istall rustfmt to check code style
```bash
rustup component add rustfmt
```

# Using Cargo to manage Projects

## Create Project
```bash
cargo new <project_name>
```

## Build Project
```bash
cargo build
```

## Execute Project
```bash
./target/debug/<project_name>
```

## Build and Execute
```bash
cargo run
```

## Only compile
```bash
cargo check
```

## Release mode
```bash
cargo <command> --release
```