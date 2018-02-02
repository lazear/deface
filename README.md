# deface

Deface is a lightweight and *fast* markup language similar to markdown. It shares most of the same syntax, but it plays fast and loose with some of the rules

#### Usage:

Download and install Rust
Clone the repository
```
cargo build --release
cargo run --release example.md
```

Command line arguments:
```
deface <input file> [output file]
```

Input file is a markup file (default extension ".md"). If no output file is provided, 's/md/html/g' will be used to create an output file