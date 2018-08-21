<head><link rel="stylesheet" type="text/css" href="style.css"></head>

![Crates.io](https://img.shields.io/crates/v/deface.svg)

# This is a first level header
## Second
### Third
#### Fourth
##### Fifth

- This is an unordered list
- which must be across 
- consecutive lines
1. ~This is a numbered list~
120. *(a.k.a an ordered list)*

Lines starting with < are included as-is, allowing HTML or CSS attributes to be embedded inline
while
> Lines starting with > are ~blockquotes~,  
> like what you would see in an email.  
> - author

code block's are delimited by 3 back ticks
```
let x = 10;
let q = 20;
let z = x * q;
```

`{This creates an anchor point}`, and 
`[This is how you create a link](#anchor)`

Sentences without an empty line
will be grouped into a paragraph
Although you can add a line break by ending a line
with two spaces  
like this

{paragraph}This is a separate paragraph which has a [self-referential link](#paragraph)
backticks escape what they contain `[`, as do backslashes "\\"

`===` makes a horizontal rule  
===

*`*asterisks*`* make things *bold!*

~`~tilde's~`~ make things ~italic~

`_underscores_` _underline_  