use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
    Semicolon,
    RedirectOut(String),   // > file
    RedirectAppend(String), // >> file
    RedirectIn(String),    // < file
}

pub fn tokenize(input: &str, last_exit_code: i32) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    let flush = |current: &mut String, tokens: &mut Vec<Token>| {
        if !current.is_empty() {
            tokens.push(Token::Word(std::mem::take(current)));
        }
    };

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            '\\' if !in_single => {
                if let Some(nc) = chars.next() {
                    current.push(nc);
                }
            }
            '$' if !in_single => {
                let mut name = String::new();
                if chars.peek() == Some(&'{') {
                    chars.next();
                    while let Some(&ch) = chars.peek() {
                        if ch == '}' { chars.next(); break; }
                        name.push(ch);
                        chars.next();
                    }
                } else if chars.peek() == Some(&'?') {
                    chars.next();
                    current.push_str(&last_exit_code.to_string());
                    continue;
                } else {
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            name.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                current.push_str(&env::var(&name).unwrap_or_default());
            }
            '~' if !in_single && !in_double && current.is_empty() => {
                let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
                current.push_str(&home);
            }
            '|' if !in_single && !in_double => {
                flush(&mut current, &mut tokens);
                tokens.push(Token::Pipe);
            }
            ';' if !in_single && !in_double => {
                flush(&mut current, &mut tokens);
                tokens.push(Token::Semicolon);
            }
            '>' if !in_single && !in_double => {
                flush(&mut current, &mut tokens);
                let append = chars.peek() == Some(&'>');
                if append { chars.next(); }
                // Collect the filename (skip spaces)
                while chars.peek() == Some(&' ') { chars.next(); }
                let mut file = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == ' ' || ch == '|' || ch == ';' || ch == '<' || ch == '>' { break; }
                    file.push(ch);
                    chars.next();
                }
                let file = expand_tilde(&file);
                if append {
                    tokens.push(Token::RedirectAppend(file));
                } else {
                    tokens.push(Token::RedirectOut(file));
                }
            }
            '<' if !in_single && !in_double => {
                flush(&mut current, &mut tokens);
                while chars.peek() == Some(&' ') { chars.next(); }
                let mut file = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == ' ' || ch == '|' || ch == ';' || ch == '<' || ch == '>' { break; }
                    file.push(ch);
                    chars.next();
                }
                tokens.push(Token::RedirectIn(expand_tilde(&file)));
            }
            ' ' | '\t' if !in_single && !in_double => {
                flush(&mut current, &mut tokens);
            }
            _ => current.push(c),
        }
    }

    flush(&mut current, &mut tokens);
    tokens
}

fn expand_tilde(s: &str) -> String {
    if s == "~" || s.starts_with("~/") {
        let home = env::var("HOME").unwrap_or_default();
        format!("{}{}", home, &s[1..])
    } else {
        s.to_string()
    }
}
