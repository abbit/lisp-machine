use core::fmt;

use crate::{ast::Expr, lexer::*};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    LexError(LexicalError),
    UnexpectedToken,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::LexError(e) => write!(f, "{}", e),
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
        }
    }
}

pub type ParseResult<'a> = Result<(Expr, &'a [Result<Token, LexicalError>]), ParseError>;

pub fn parse(tokens: &[Result<Token, LexicalError>]) -> ParseResult {
    if tokens.is_empty() {
        return Ok((Expr::Void, tokens));
    }
    let (item, rest) = tokens.split_at(1);
    match item {
        [Ok(Token::Integer(i))] => Ok((Expr::Integer(*i), rest)),
        [Ok(Token::Float(f))] => Ok((Expr::Float(*f), rest)),
        [Ok(Token::Symbol(s))] => Ok((Expr::Symbol(s.clone()), rest)),
        [Ok(Token::String(s))] => Ok((Expr::String(s.clone()), rest)),
        [Ok(Token::Char(c))] => Ok((Expr::Char(*c), rest)),
        [Ok(Token::LParen)] => {
            let mut sub_exprs = vec![];
            let mut rem = rest;
            while !matches!(rem.first(), Some(Ok(Token::RParen)) | None) {
                let (expr, remaining) = parse(rem)?;
                sub_exprs.push(expr);
                rem = remaining;
            }
            if let [Ok(Token::RParen)] = rem.split_at(1).0 {
                Ok((Expr::List(sub_exprs), &rem[1..]))
            } else {
                Err(ParseError::LexError(LexicalError::UnexpectedEOF))
            }
        }
        [Ok(Token::Quote)] => {
            let (expr, remaining) = parse(rest)?;
            Ok((Expr::Quote(Box::new(expr)), remaining))
        }
        [Ok(Token::Boolean(b))] => Ok((Expr::Boolean(*b), rest)), // Handle Boolean literals
        [Err(LexicalError::UnexpectedEOF)] => {
            Err(ParseError::LexError(LexicalError::UnexpectedEOF))
        }
        [Err(LexicalError::UnclosedString)] => {
            Err(ParseError::LexError(LexicalError::UnclosedString))
        }
        [Err(LexicalError::UnexpectedRParen)] => {
            Err(ParseError::LexError(LexicalError::UnexpectedRParen))
        }
        _ => Err(ParseError::UnexpectedToken),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_complex() {
        let lexer = Lexer::new("(cos (* 3.14159 1))");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("cos".to_string()),
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Float(3.14159),
                    Expr::Integer(1),
                ]),
            ])
        );
    }

    #[test]
    fn parse_quote() {
        let lexer = Lexer::new("'(1 2 3)");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::Quote(Box::new(Expr::List(vec![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3),
            ])))
        );
    }

    #[test]
    fn parse_long() {
        let lexer = Lexer::new("(+ (* 3 (+ (* 2 4)  (+ 3 5))) (+ (- 10 7) 6))");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Integer(3),
                    Expr::List(vec![
                        Expr::Symbol("+".to_string()),
                        Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Integer(2),
                            Expr::Integer(4),
                        ]),
                        Expr::List(vec![
                            Expr::Symbol("+".to_string()),
                            Expr::Integer(3),
                            Expr::Integer(5),
                        ]),
                    ]),
                ]),
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::List(vec![
                        Expr::Symbol("-".to_string()),
                        Expr::Integer(10),
                        Expr::Integer(7),
                    ]),
                    Expr::Integer(6),
                ]),
            ])
        );
    }

    #[test]
    fn parse_unexpected_eof() {
        let lexer = Lexer::new("(+ 1 2");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnexpectedEOF))
        );
    }

    #[test]
    fn parse_string() {
        let lexer = Lexer::new(r#"(display "Hello, world!")"#);
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("display".to_string()),
                Expr::String("Hello, world!".to_string()),
            ])
        );
    }

    #[test]
    fn parse_unclosed_string() {
        let lexer = Lexer::new(r#"(display "hello)"#);
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnclosedString))
        );
    }

    #[test]
    fn parse_unexpected_close_paren() {
        let lexer = Lexer::new("(+ 1 2))");
        let tokens: Vec<_> = lexer.collect();
        let (parsed, remaining) = parse(&tokens).unwrap();
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Integer(1),
                Expr::Integer(2),
            ])
        );
        assert_eq!(
            remaining.first(),
            Some(&Err(LexicalError::UnexpectedRParen))
        );
    }

    #[test]
    fn parse_void() {
        let lexer = Lexer::new("");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(parsed, Expr::Void);
    }

    #[test]
    fn parse_two_parens() {
        let lexer = Lexer::new(
            "(define pi 314)
                                                    (+ pi 1)",
        );
        let tokens: Vec<_> = lexer.collect();
        let (parsed1, remaining1) = parse(&tokens).unwrap();
        let (parsed2, _) = parse(remaining1).unwrap();
        assert_eq!(
            parsed1,
            Expr::List(vec![
                Expr::Symbol("define".to_string()),
                Expr::Symbol("pi".to_string()),
                Expr::Integer(314),
            ])
        );
        assert_eq!(
            parsed2,
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("pi".to_string()),
                Expr::Integer(1),
            ])
        );
    }

    #[test]
    fn parse_boolean() {
        let lexer = Lexer::new("(not #t)");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("not".to_string()),
                Expr::Boolean(true),
            ])
        );
    }

    #[test]
    fn parse_char() {
        let lexer = Lexer::new("(not #\\e)");
        let tokens: Vec<_> = lexer.collect();
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::Symbol("not".to_string()),
                Expr::Char('e'),
            ])
        );
    }
}
