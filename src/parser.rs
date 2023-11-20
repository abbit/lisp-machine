mod lexer;
use lexer::*;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Expr>),
    Quote(Box<Expr>),
    Void
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    LexError(LexicalError),
    UnexpectedToken,
}

pub fn parse(tokens: &[Result<Token, LexicalError>]) -> Result<(Expr, &[Result<Token, LexicalError>]), ParseError> {
    if tokens.is_empty() {
        return Ok((Expr::Void, tokens));
    }
    let (item, rest) = tokens.split_at(1);
    match item {
        [Ok(Token::Integer(i))] => Ok((Expr::Integer(*i), rest)),
        [Ok(Token::Float(f))] => Ok((Expr::Float(*f), rest)),
        [Ok(Token::String(s))] => Ok((Expr::String(s.clone()), rest)),
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
        [Err(LexicalError::UnexpectedEOF)] => Err(ParseError::LexError(LexicalError::UnexpectedEOF)),
        [Err(LexicalError::UnclosedString)] => Err(ParseError::LexError(LexicalError::UnclosedString)),
        [Err(LexicalError::UnexpectedRParen)] => Err(ParseError::LexError(LexicalError::UnexpectedRParen)),
        _ => Err(ParseError::UnexpectedToken)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complex() {
        let tokens = lex("(cos (* 3.14159 1))");
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("cos".to_string()),
                Expr::List(vec![
                    Expr::String("*".to_string()),
                    Expr::Float(3.14159),
                    Expr::Integer(1),
                ]),
            ])
        );
    }

    #[test]
    fn test_parse_quote() {
        let tokens = lex("'(1 2 3)");
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
    fn test_parse_long() {
        let tokens = lex("(+ (* 3
            (+ (* 2 4)
               (+ 3 5)
            )
         )
         (+ (- 10 7)
            6
         )
        )");
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("+".to_string()),
                Expr::List(vec![
                    Expr::String("*".to_string()),
                    Expr::Integer(3),
                    Expr::List(vec![
                        Expr::String("+".to_string()),
                        Expr::List(vec![
                            Expr::String("*".to_string()),
                            Expr::Integer(2),
                            Expr::Integer(4),
                        ]),
                        Expr::List(vec![
                            Expr::String("+".to_string()),
                            Expr::Integer(3),
                            Expr::Integer(5),
                        ]),
                    ]),
                ]),
                Expr::List(vec![
                    Expr::String("+".to_string()),
                    Expr::List(vec![
                        Expr::String("-".to_string()),
                        Expr::Integer(10),
                        Expr::Integer(7),
                    ]),
                    Expr::Integer(6),
                ]),
            ])
        );
    }

    #[test]
    fn test_unexpected_eof() {
        let tokens = lex("(+ 1 2");
        let parsed = parse(&tokens);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnexpectedEOF))
        );
    }

    #[test]
    fn test_string() {
        let tokens = lex(r#"(display "Hello, world!")"#);
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("display".to_string()),
                Expr::String("Hello, world!".to_string()),
            ])
        );
    }

    #[test]
    fn test_unclosed_string() {
        let tokens = lex(r#"(display "hello)"#);
        let parsed = parse(&tokens);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnclosedString))
        );
    }

    #[test]
    fn test_unexpected_close_paren() {
        let tokens = lex("(+ 1 2))");
        let (parsed, remaining) = parse(&tokens).unwrap();
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("+".to_string()),
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
    fn test_void() {
        let tokens = lex("");
        let parsed = parse(&tokens).unwrap().0;
        assert_eq!(
            parsed,
            Expr::Void
        );
    }
}