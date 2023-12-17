use core::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Comment(String),
    Integer(i64),
    Float(f64),
    Symbol(String),
    LParen,
    RParen,
    Quote,
    Boolean(bool),
    String(String),
    Char(char),
    Dot // " . "
}

#[derive(PartialEq, Debug)]
pub enum LexicalError {
    UnexpectedEOF,
    UnexpectedRParen,
    UnclosedString,
    UnexpectedChar,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            LexicalError::UnexpectedEOF => write!(f, "unexpected EOF"),
            LexicalError::UnexpectedRParen => write!(f, "unexpected ')'"),
            LexicalError::UnclosedString => write!(f, "unclosed string"),
            LexicalError::UnexpectedChar => write!(f, "unexpected char"),
        }
    }
}

pub type LexResult = Result<Token, LexicalError>;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    open_paren_count: i32,
    has_error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Lexer {
            chars: program.chars().peekable(),
            open_paren_count: 0,
            has_error: false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_error {
            return None;
        }
    
        match self.chars.peek() {
            Some(&ch) => {
                let result = match ch {
                    '(' => {
                        self.open_paren_count += 1;
                        self.chars.next();
                        Ok(Token::LParen)
                    }
                    ')' => {
                        self.open_paren_count -= 1;
                        self.chars.next();
                        if self.open_paren_count < 0 {
                            Err(LexicalError::UnexpectedRParen)
                        } else {
                            Ok(Token::RParen)
                        }
                    }
                    '.' => match self.chars.clone().nth(1) {
                        Some(next_ch) if next_ch.is_whitespace() => {
                            self.chars.next();
                            Ok(Token::Dot)
                        }
                        _ => finalize_token(&mut self.chars),
                    },
                    '\'' => {
                        self.chars.next();
                        Ok(Token::Quote)
                    }
                    ';' => {
                        self.chars.next();
                        let comment = consume_until_newline(&mut self.chars);
                        Ok(Token::Comment(comment))
                    }
                    _ if ch.is_whitespace() => {
                        self.chars.next();
                        return self.next();
                    }
                    _ => finalize_token(&mut self.chars),
                };
                if result.is_err() {
                    self.has_error = true;
                }
                Some(result)
            }
            None => match (self.open_paren_count > 0, self.open_paren_count < 0) {
                (true, _) => {
                    self.has_error = true;
                    Some(Err(LexicalError::UnexpectedEOF))
                }
                (_, true) => {
                    self.has_error = true;
                    Some(Err(LexicalError::UnexpectedRParen))
                }
                _ => None,
            },
        }
    }
    
}

fn finalize_token(chars: &mut Peekable<Chars>) -> LexResult {
    let mut token_string = String::new();
    let mut is_string = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                if is_string {
                    chars.next();
                    return Ok(Token::String(token_string));
                } else {
                    is_string = true;
                }
            }
            _ if is_string => {
                token_string.push(ch);
            }
            _ if ch.is_whitespace() || ch == '(' || ch == ')' || ch == '\'' => {
                break;
            }
            _ => {
                token_string.push(ch);
            }
        }
        chars.next();
    }

    if is_string {
        Err(LexicalError::UnclosedString)
    } else {
        match token_string.trim() {
            "#t" => Ok(Token::Boolean(true)),
            "#f" => Ok(Token::Boolean(false)),
            _ if token_string.starts_with("#\\") && token_string.len() == 3 => {
                Ok(Token::Char(token_string.chars().nth(2).unwrap()))
            }
            _ if token_string.starts_with("#\\") && token_string.len() > 3 => {
                handle_char_literal(&token_string)
            }
            _ => Ok(token_string
                .parse::<i64>()
                .map(Token::Integer)
                .or_else(|_| token_string.parse::<f64>().map(Token::Float))
                .unwrap_or(Token::Symbol(token_string))),
        }
    }
}

fn consume_until_newline(chars: &mut Peekable<Chars>) -> String {
    let mut comment = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '\n' {
            break;
        }
        comment.push(ch);
        chars.next();
    }
    comment
}

fn handle_char_literal(symbol: &str) -> LexResult {
    match symbol {
        "#\\alarm" => Ok(Token::Char('\x07')),
        "#\\backspace" => Ok(Token::Char('\x08')),
        "#\\delete" => Ok(Token::Char('\x7F')),
        "#\\escape" => Ok(Token::Char('\x1B')),
        "#\\newline" => Ok(Token::Char('\n')),
        "#\\null" => Ok(Token::Char('\0')),
        "#\\return" => Ok(Token::Char('\r')),
        "#\\space" => Ok(Token::Char(' ')),
        "#\\tab" => Ok(Token::Char('\t')),
        _ => Err(LexicalError::UnexpectedChar),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_complex() {
        let lexer = Lexer::new("(cos (* 3.14159 1))");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("cos".to_string())),
                Ok(Token::LParen),
                Ok(Token::Symbol("*".to_string())),
                Ok(Token::Float(3.14159)),
                Ok(Token::Integer(1)),
                Ok(Token::RParen),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_quote() {
        let lexer = Lexer::new("'(1 2 3)");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::Quote),
                Ok(Token::LParen),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(2)),
                Ok(Token::Integer(3)),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_long() {
        let lexer = Lexer::new("(+ (* 3 (+ (* 2 4)  (+ 3 5))) (+ (- 10 7) 6))");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::Symbol("*".to_string())),
                Ok(Token::Integer(3)),
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::Symbol("*".to_string())),
                Ok(Token::Integer(2)),
                Ok(Token::Integer(4)),
                Ok(Token::RParen),
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::Integer(3)),
                Ok(Token::Integer(5)),
                Ok(Token::RParen),
                Ok(Token::RParen),
                Ok(Token::RParen),
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::Symbol("-".to_string())),
                Ok(Token::Integer(10)),
                Ok(Token::Integer(7)),
                Ok(Token::RParen),
                Ok(Token::Integer(6)),
                Ok(Token::RParen),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_unexpected_eof() {
        let lexer = Lexer::new("(+ 1 2");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(2)),
                Err(LexicalError::UnexpectedEOF)
            ]
        );
    }

    #[test]
    fn lex_string() {
        let lexer = Lexer::new(r#"(display "Hello, world!")"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("display".to_string())),
                Ok(Token::String("Hello, world!".to_string())),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_unclosed_string() {
        let lexer = Lexer::new(r#"(display "hello)"#);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("display".to_string())),
                Err(LexicalError::UnclosedString),
            ]
        );
    }

    #[test]
    fn lex_unexpected_close_paren() {
        let lexer = Lexer::new("(+ 1 2))");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(2)),
                Ok(Token::RParen),
                Err(LexicalError::UnexpectedRParen),
            ]
        );
    }

    #[test]
    fn lex_void() {
        let lexer = Lexer::new("");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn lex_two_parens() {
        let lexer = Lexer::new(
            "(define pi 314)
                                                    (+ pi 1)",
        );
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("define".to_string())),
                Ok(Token::Symbol("pi".to_string())),
                Ok(Token::Integer(314)),
                Ok(Token::RParen),
                Ok(Token::LParen),
                Ok(Token::Symbol("+".to_string())),
                Ok(Token::Symbol("pi".to_string())),
                Ok(Token::Integer(1)),
                Ok(Token::RParen)
            ]
        );
    }

    #[test]
    fn lex_boolean() {
        let lexer = Lexer::new("(not #f)");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("not".to_string())),
                Ok(Token::Boolean(false)),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_char() {
        let lexer = Lexer::new("(not #\\e)");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("not".to_string())),
                Ok(Token::Char('e')),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_char_literal() {
        let lexer = Lexer::new("(not #\\alarm)");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::Symbol("not".to_string())),
                Ok(Token::Char('\x07')),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn lex_dot() {
        let lexer = Lexer::new("'(3 . 4)");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::Quote),
                Ok(Token::LParen),
                Ok(Token::Integer(3)),
                Ok(Token::Dot),
                Ok(Token::Integer(4)),
                Ok(Token::RParen)
            ]
        );
    }

    #[test]
    fn lex_multiple_comments() {
        let lexer = Lexer::new("; this is a comment\n; this is another comment");
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Ok(Token::Comment(" this is a comment".to_string())),
                Ok(Token::Comment(" this is another comment".to_string())),
            ]
        );
    }
}
