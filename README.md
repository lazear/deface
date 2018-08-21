# deface

![Crates.io](https://img.shields.io/crates/v/deface.svg)

Deface is a lightweight and *fast* markup language similar to markdown. It shares most of the same syntax, but it plays fast and loose with some of the rules

## Syntax

Run `deface example.md` to see this first hand

```markdown
![img alt text](./image_location.png)

{anchor-points} can be created and [linked to](#anchor-points)

# Headers are the same as markdown

> Lines starting with `>` 
> are blockquotes

- Unordered
- Lists
- Use Dashes

1. And ordered
23. lists use numbers
1234. (any numbers!)

<html> can be directly embedded. Any line starting with `<` is skipped by the parser </html>
~italic!~
*bold*
_underline_
=== horizontal rule
```


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
