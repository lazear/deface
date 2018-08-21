use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::iter::Peekable;
use std::str;

pub struct Lexer<'a> {
    remaining: Peekable<str::Lines<'a>>,
    current_line: Option<Peekable<str::Chars<'a>>>,

    output: String,
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl<'a> Lexer<'a> {
    pub fn lex(s: &'a str) -> Result<String, Error> {
        let mut l = Lexer {
            remaining: s.lines().peekable(),
            current_line: None,
            output: String::new(),
            line: 0,
            col: 0,
        };
        l.parse()?;
        Ok(l.output)
    }

    fn peek_char(&mut self) -> Option<char> {
        match self.current_line {
            Some(ref mut c) => c.peek().cloned(),
            None => None,
        }
    }

    fn peek_line(&mut self) -> Option<&str> {
        self.remaining.peek().map(|s| *s)
    }

    // Returns the next line in the iterator
    fn next_line(&mut self) -> Option<String> {
        let next = self.remaining.next();
        if let Some(line) = next {
            self.line += 1;
            self.col = 0;
            self.current_line = Some(line.chars().peekable());
        }
        next.map(|s| s.into())
    }

    // Returns the next line in the iterator
    fn next_char(&mut self) -> Option<char> {
        match self.current_line {
            Some(ref mut iter) => match iter.next() {
                Some(c) => {
                    self.col += 1;
                    Some(c)
                }
                None => None,
            },
            None => None,
        }
    }

    fn consume_while_char<F: Fn(char) -> bool>(&mut self, lambda: F) -> Option<String> {
        let mut s = String::new();
        let mut escape = false;
        while let Some(c) = self.peek_char() {
            // Don't escape backticks if that's what we're trying to parse out!
            if c == '`' && lambda('`') {
                escape = !escape;
                self.next_char();
                continue;
            }
            if lambda(c) || escape {
                s.push(self.next_char().unwrap());
            } else {
                return Some(s);
            }
        }
        if s == "" {
            None
        } else {
            Some(s)
        }
    }

    fn expect_char(&mut self, c: char) -> Result<(), Error> {
        if self.next_char() == Some(c) {
            Ok(())
        } else {
            Err(Error {
                message: format!(
                    "Expecting character {}, on line {} column {}",
                    c, self.line, self.col
                ),
            })
        }
    }

    fn emit(&mut self, s: &str) {
        if s != "" {
            self.output.push_str(s);
            self.output.push('\n');
        }
    }

    fn transform(&mut self, mut escaped: bool) -> Result<String, Error> {
        let mut out = String::new();

        while let Some(c) = self.next_char() {
            if escaped {
                if c == '`' {
                    escaped = false;
                } else {
                    out.push(c);
                }
            } else {
                match c {
                    '`' => {
                        escaped = !escaped;
                        if escaped {
                            if let Some(text) = self.consume_while_char(|c| c != '`') {
                                out.push_str(&text);
                            }
                        }
                    }
                    '[' => {
                        // Parse out link
                        if let Some(linktext) = self.consume_while_char(|c| c != ']') {
                            self.expect_char(']')?;
                            self.expect_char('(')?;
                            if let Some(target) = self.consume_while_char(|c| c != ')') {
                                self.expect_char(')')?;
                                out.push_str(&format!(r#"<a href="{}">{}</a>"#, target, linktext));
                            }
                        }
                    }
                    '*' => {
                        if let Some(peek) = self.peek_char() {
                            if peek.is_whitespace() {
                                out.push(c);
                            } else {
                                if let Some(emph) = self.consume_while_char(|c| c != '*') {
                                    out.push_str(&format!("<strong>{}</strong>", emph));
                                }
                                self.expect_char('*')?;
                            }
                        }
                    }
                    '~' => {
                        if let Some(peek) = self.peek_char() {
                            if peek.is_whitespace() {
                                out.push(c);
                            } else {
                                if let Some(emph) = self.consume_while_char(|c| c != '~') {
                                    out.push_str(&format!("<em>{}</em>", emph));
                                }
                                self.expect_char('~')?;
                            }
                        }
                    }
                    '_' => {
                        if let Some(peek) = self.peek_char() {
                            if peek.is_whitespace() {
                                out.push(c);
                            } else {
                                if let Some(emph) = self.consume_while_char(|c| c != '_') {
                                    out.push_str(&format!("<u>{}</u>", emph));
                                }
                                self.expect_char('_')?;
                            }
                        }
                    }
                    '{' => {
                        if let Some(anchor) = self.consume_while_char(|c| c != '}') {
                            self.expect_char('}')?;
                            out.push_str(&format!(r#"<a name="{}"></a>"#, anchor))
                        }
                    }
                    '\\' => {
                        if let Some(c) = self.next_char() {
                            out.push(c);
                        }
                    }
                    '=' => {
                        if let Some(hr) = self.consume_while_char(|c| c == '=') {
                            if hr.len() >= 2 {
                                out.push_str("<hr>");
                            } else {
                                out.push_str(&hr);
                            }
                        } else {
                            out.push(c);
                        }
                    }
                    _ => {
                        out.push(c);
                    }
                }
            }
        }
        // double space = line break
        if out.ends_with("  ") {
            out.pop().unwrap();
            out.pop().unwrap();
            out.push_str("<br />");
        }
        Ok(out)
    }

    fn parse(&mut self) -> Result<(), Error> {
        let mut in_list = false;
        let mut in_block = false;
        let mut paragraph = true;
        self.emit("<!DOCTYPE html>");
        self.emit("<html>");
        while let Some(_) = self.next_line() {
            if let Some(s) = self.peek_char() {
                match s {
                    '<' => {
                        if let Some(html) = self.consume_while_char(|_| true) {
                            self.emit(&html);
                        }
                    }
                    '#' => {
                        let depth = match self.consume_while_char(|c| c == '#') {
                            Some(s) => s.len(),
                            None => 1,
                        };
                        let content = self.transform(false)?;
                        self.emit(&format!("<h{}>{}</h{}>", depth, content, depth));
                    }
                    '-' => {
                        if !in_list {
                            self.emit("<ul>");
                            in_list = true;
                        }
                        self.expect_char('-')?;
                        self.consume_while_char(char::is_whitespace);
                        let content = self.transform(false)?;
                        self.emit(&format!("<li>{}</li>", content));

                        // Is the next line also a list?
                        if let Some(next) = self.peek_line() {
                            if !next.starts_with('-') {
                                in_list = false;
                            }
                        }
                        if !in_list {
                            self.emit("</ul>\n");
                        }
                    }
                    '>' => {
                        if !paragraph {
                            self.emit("<p>");
                        }
                        paragraph = true;
                        if !in_block {
                            self.emit("<blockquote>");
                            in_block = true;
                        }
                        self.expect_char('>')?;
                        self.consume_while_char(char::is_whitespace);
                        let content = self.transform(false)?;
                        self.emit(&content);
                        if let Some(next) = self.peek_line() {
                            if !next.starts_with('>') {
                                in_block = false;
                            }
                        }
                        if !in_block {
                            self.emit("</blockquote>");
                        }
                    }
                    '`' => {
                        if !paragraph {
                            self.emit("<p>");
                        }
                        paragraph = true;
                        let mut buffer = String::new();
                        while let Some(c) = self.peek_char() {
                            if c == '`' {
                                buffer.push(c);
                                self.next_char();
                            } else {
                                break;
                            }
                        }
                        if buffer.len() == 3 {
                            let mut v: Vec<String> = Vec::new();

                            while let Some(line) = self.next_line() {
                                if line == "```" {
                                    break;
                                } else {
                                    v.push(line);
                                }
                            }
                            self.emit("<pre><code>");
                            for l in v {
                                self.emit(&l);
                            }
                            self.emit("</pre></code>");
                        } else {
                            let s = self.transform(true)?;
                            self.emit(&s);
                        }
                    }
                    _ => {
                        if !paragraph {
                            self.emit("<p>");
                        }
                        paragraph = true;
                        if s.is_digit(10) {
                            if !in_list {
                                self.emit("<ol>");
                                in_list = true;
                            }
                            self.consume_while_char(|c| c.is_digit(10));
                            self.expect_char('.')?;
                            let content = self.transform(false)?;
                            self.emit(&format!("<li>{}</li>", content));
                            // Is the next line also a list?
                            if let Some(next) = self.peek_line() {
                                if !next.starts_with(|c| char::is_digit(c, 10)) {
                                    in_list = false;
                                }
                            }
                            if !in_list {
                                self.emit("</ol>");
                            }
                        }
                        let s = self.transform(false)?;
                        self.emit(&s);
                    }
                }
            } else {
                if paragraph {
                    self.emit("</p>");
                }
                paragraph = false;
            }
        }
        self.emit("</html>");
        Ok(())
    }
}

fn convert(input: &str, output: &str) -> Result<(), Error> {
    match (File::open(input), File::create(output)) {
        (Ok(mut f), Ok(mut out)) => {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).map_err(|_| Error {
                message: format!("Could not read {}", input),
            })?;
            out.write_all(Lexer::lex(&buffer)?.as_bytes())
                .map_err(|_| Error {
                    message: format!("Could not write to {}", output),
                })?;
            Ok(())
        }
        (Ok(_), _) => Err(Error {
            message: format!("Error opening file {}", output),
        }),
        (_, Ok(_)) => Err(Error {
            message: format!("Error opening file {}", input),
        }),
        (_, _) => Err(Error {
            message: format!("Error opening files: {}, {}", input, output),
        }),
    }
}

fn main() {
    let mut args = env::args();
    match args.len() {
        1 => {
            println!(
                "usage: {} <input file> <input file 2> ...",
                args.nth(0).unwrap()
            );
            println!("Markdown files will be converted to HTML");
            return;
        }
        _ => {
            for v in args.skip(1) {
                if v.contains(".md") {
                    let input = v.clone();
                    let output = v.replace(".md", ".html");
                    match convert(&input, &output) {
                        Ok(_) => println!("Success: {} => {}", input, output),
                        Err(e) => println!("Failure: {} => {}: {}", input, output, e.message),
                    };
                }
            }
        }
    };
}
