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
deface <input file> ...
```

Input file is a markup file (default extension ".md"), and will be translated into an HTML file.
Multiple input files can be provided, like so
```
deface README.md index.md resume.md about.md
```
