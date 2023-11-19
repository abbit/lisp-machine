mod lexer;
use lexer::*;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<Expr>),
    Quote(Box<Expr>),
}

pub fn parse(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (item, rest) = tokens.split_at(1);
    match item {
        [Token::Integer(i)] => Ok((Expr::Integer(*i), rest)),
        [Token::Float(f)] => Ok((Expr::Float(*f), rest)),
        [Token::String(s)] => Ok((Expr::String(s.clone()), rest)),
        [Token::LParen] => {
            let mut sub_exprs = vec![];
            let mut rem = rest;
            while !matches!(rem.first(), Some(Token::RParen) | None) {
                let (expr, remaining) = parse(rem)?;
                sub_exprs.push(expr);
                rem = remaining;
            }
            if let [Token::RParen] = rem.split_at(1).0 {
                Ok((Expr::List(sub_exprs), &rem[1..]))
            } else {
                Err("Unmatched left parenthesis".to_string())
            }
        }
        [Token::Quote] => {
            let (expr, remaining) = parse(rest)?;
            Ok((Expr::Quote(Box::new(expr)), remaining))
        }
        _ => Err("Unexpected end of input".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens_int = lex("(+ 1 2)");
        let parsed = parse(&tokens_int).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("+".to_string()),
                Expr::Integer(1),
                Expr::Integer(2),
            ])
        );

        let tokens_float = lex("(+ 1.0 2.5)");
        let parsed = parse(&tokens_float).unwrap().0;
        assert_eq!(
            parsed,
            Expr::List(vec![
                Expr::String("+".to_string()),
                Expr::Float(1.0),
                Expr::Float(2.5),
            ])
        );

        let tokens_complex = lex("(cos (* 3.14159 1))");
        let parsed = parse(&tokens_complex).unwrap().0;
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

        let tokens_quote = lex("'(1 2 3)");
        let parsed = parse(&tokens_quote).unwrap().0;
        assert_eq!(
            parsed,
            Expr::Quote(Box::new(Expr::List(vec![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3),
            ])))
        );

        let tokens_lot = lex("(+ (* 3
            (+ (* 2 4)
               (+ 3 5)
            )
         )
         (+ (- 10 7)
            6
         )
        )");
        let parsed = parse(&tokens_lot).unwrap().0;
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
}
