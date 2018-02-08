# deface

![Crates.io](https://img.shields.io/crates/v/deface.svg)

Deface is a lightweight and *fast* markup language similar to markdown. It shares most of the same syntax, but it plays fast and loose with some of the rules

#### Usage:

Download and install the [Rust toolchain](https://www.rustup.rs/)

Install from crates.io
```
cargo install deface
```

Command line arguments:
```
deface <input file> ...
```

Input files are markup files (default extension ".md"), and will be translated into an HTML file.
Multiple input files can be provided, like so
```
deface README.md index.md resume.md about.md
```
