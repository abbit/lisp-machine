use super::lexer::{LexResult, Lexer, LexicalError, Token};
use crate::expr::{exprs, Expr, Exprs, ListKind};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    LexError(LexicalError),
    UnexpectedToken(Token),
}

impl ParseError {
    fn unexpected_eof() -> Self {
        ParseError::UnexpectedToken(Token::Eof)
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::LexError(err) => write!(f, "lex error: {}", err),
            ParseError::UnexpectedToken(found) => {
                write!(f, "unexpected token: {:?}", found)
            }
        }
    }
}

pub type ParseExprResult = Result<Expr, ParseError>;

trait CheckEof {
    fn ok_or_unexpected_eof(self) -> ParseExprResult;
}

impl CheckEof for Option<ParseExprResult> {
    fn ok_or_unexpected_eof(self) -> ParseExprResult {
        self
            // if Option<ParseExprResult> returns None, we hit EOF
            .ok_or(ParseError::unexpected_eof())
            // transform Result<Result<Expr, ParseError>, ParseError> into Result<Expr, ParseError>
            .and_then(|res| res)
    }
}

macro_rules! to_quatation_call {
    ($parser:expr, $symbol:expr) => {{
        let quoted = $parser
            .parse_expr()
            .ok_or_unexpected_eof()
            // wrap expr in quotation call
            .map(|expr| Expr::new_proper_list(exprs![Expr::Symbol($symbol.to_string()), expr]));
        Some(quoted)
    }};
}

pub struct Parser<I: Iterator> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = LexResult>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_expr(&mut self) -> Option<ParseExprResult> {
        match self.tokens.next() {
            Some(Ok(tok)) => match tok {
                Token::Comment(_) => Some(Ok(Expr::Void)),
                Token::Boolean(boolean) => Some(Ok(Expr::Boolean(boolean))),
                Token::String(string) => Some(Ok(Expr::String(string))),
                Token::Symbol(symbol) => Some(Ok(Expr::Symbol(symbol))),
                Token::Integer(int) => Some(Ok(Expr::Integer(int))),
                Token::Float(float) => Some(Ok(Expr::Float(float))),
                Token::Char(char) => Some(Ok(Expr::Char(char))),
                Token::LParen => Some(self.parse_list()),
                // we consume right paren in `parse_list`, so seeing a right paren here is an error
                Token::RParen => Some(Err(ParseError::LexError(LexicalError::UnexpectedRParen))),
                // transform quotation tokens into quotation calls
                Token::Quote => to_quatation_call!(self, "quote"),
                Token::Quasiquote => to_quatation_call!(self, "quasiquote"),
                Token::Unquote => to_quatation_call!(self, "unquote"),
                Token::UnquoteSplicing => to_quatation_call!(self, "unquote-splicing"),
                // we handle dot in `parse_list`, so seeing a dot here is an error
                Token::Dot => Some(Err(ParseError::UnexpectedToken(Token::Dot))),
                Token::Eof => None,
            },
            Some(Err(err)) => Some(Err(ParseError::LexError(err))),
            None => None,
        }
    }

    fn parse_list(&mut self) -> ParseExprResult {
        let mut list = Exprs::new();
        loop {
            let peek_result = self.tokens.peek().ok_or(ParseError::unexpected_eof())?;
            let tok = match peek_result {
                Ok(tok) => tok,
                // if peeked token is an error, get next token and return the error
                // unwrap is safe because we just checked that peek_result is Some and is an error
                Err(_) => {
                    return Err(ParseError::LexError(
                        self.tokens.next().unwrap().unwrap_err(),
                    ))
                }
            };
            match tok {
                // if we see a right paren, we're done parsing the list
                Token::RParen => {
                    // consume the right paren
                    self.tokens.next();
                    break;
                }
                // if we see a dot, we're parsing a dotted list
                // or proper list (dotted list with null (empty list) at the end)
                Token::Dot => {
                    // consume the dot
                    self.tokens.next();
                    let tail_expr = self.parse_expr().ok_or_unexpected_eof()?;

                    if self.tokens.next() != Some(Ok(Token::RParen)) {
                        return Err(ParseError::UnexpectedToken(Token::Dot));
                    }

                    let kind = match tail_expr.into_list() {
                        Ok(tail_list) => {
                            let kind = tail_list.kind();
                            list.extend(tail_list);
                            kind
                        }
                        Err(expr) => {
                            list.push_back(expr);
                            ListKind::Dotted
                        }
                    };

                    return Ok(Expr::new_list(list, kind));
                }
                // otherwise, parse the next expression and add it to the list
                _ => match self
                    .parse_expr()
                    // if we hit EOF, we're missing a right paren
                    .ok_or_unexpected_eof()?
                {
                    // skip void expressions
                    Expr::Void => {}
                    expr => list.push_back(expr),
                },
            }
        }

        Ok(Expr::new_proper_list(list))
    }
}

impl<I: Iterator<Item = LexResult>> Iterator for Parser<I> {
    type Item = ParseExprResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expr()
    }
}

pub fn parse_str(string: &str) -> Result<Exprs, ParseError> {
    let tokens = Lexer::new(string).peekable();
    parse(tokens)
}

fn parse<I: Iterator<Item = LexResult>>(tokens: Peekable<I>) -> Result<Exprs, ParseError> {
    let mut parser = Parser::new(tokens);
    let mut exprs = Exprs::new();

    while let Some(expr) = parser.parse_expr() {
        match expr {
            Ok(Expr::Void) => {}
            Ok(expr) => exprs.push_back(expr),
            Err(err) => return Err(err),
        }
    }

    if exprs.is_empty() {
        exprs.push_back(Expr::Void);
    }

    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_complex() {
        let lexer = Lexer::new("(cos (* 3.14159 1))");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("cos".to_string()),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    #[allow(clippy::approx_constant)]
                    Expr::Float(3.14159),
                    Expr::Integer(1),
                ]),
            ])]
        );
    }

    #[test]
    fn parse_quote() {
        let lexer = Lexer::new("'(1 2 3)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("quote".to_string()),
                Expr::new_proper_list(exprs![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)]),
            ])]
        );
    }

    #[test]
    fn parse_long() {
        let lexer = Lexer::new("(+ (* 3 (+ (* 2 4)  (+ 3 5))) (+ (- 10 7) 6))");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("+".to_string()),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    Expr::Integer(3),
                    Expr::new_proper_list(exprs![
                        Expr::Symbol("+".to_string()),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("*".to_string()),
                            Expr::Integer(2),
                            Expr::Integer(4),
                        ]),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("+".to_string()),
                            Expr::Integer(3),
                            Expr::Integer(5),
                        ]),
                    ]),
                ]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("+".to_string()),
                    Expr::new_proper_list(exprs![
                        Expr::Symbol("-".to_string()),
                        Expr::Integer(10),
                        Expr::Integer(7),
                    ]),
                    Expr::Integer(6),
                ]),
            ])]
        );
    }

    #[test]
    fn parse_unexpected_eof() {
        let lexer = Lexer::new("(+ 1 2");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnexpectedEOF))
        );
    }

    #[test]
    fn parse_string() {
        let lexer = Lexer::new(r#"(display "Hello, world!")"#);
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("display".to_string()),
                Expr::String("Hello, world!".to_string()),
            ])]
        );
    }

    #[test]
    fn parse_unclosed_string() {
        let lexer = Lexer::new(r#"(display "hello)"#);
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnclosedString))
        );
    }

    #[test]
    fn parse_unexpected_close_paren() {
        let lexer = Lexer::new("(+ 1 2))");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer);
        assert_eq!(
            parsed,
            Err(ParseError::LexError(LexicalError::UnexpectedRParen))
        );
    }

    #[test]
    fn parse_void() {
        let lexer = Lexer::new("");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(parsed, vec![Expr::Void]);
    }

    #[test]
    fn parse_two_parens() {
        let lexer = Lexer::new(
            "(define pi 314)
        (+ pi 1)",
        );
        let tokens: Vec<_> = lexer.collect();
        let answer1 = tokens.into_iter().peekable();
        let parsed1 = parse(answer1).unwrap();
        assert_eq!(
            parsed1,
            vec![
                Expr::new_proper_list(exprs![
                    Expr::Symbol("define".to_string()),
                    Expr::Symbol("pi".to_string()),
                    Expr::Integer(314),
                ]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("pi".to_string()),
                    Expr::Integer(1),
                ])
            ]
        );
    }

    #[test]
    fn parse_boolean() {
        let lexer = Lexer::new("(not #t)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("not".to_string()),
                Expr::Boolean(true),
            ])]
        );
    }

    #[test]
    fn parse_char() {
        let lexer = Lexer::new("(not #\\e)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("not".to_string()),
                Expr::Char('e'),
            ])]
        );
    }

    #[test]
    fn parse_comment() {
        let lexer = Lexer::new("; this is a comment\n(+ 1 2)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("+".to_string()),
                Expr::Integer(1),
                Expr::Integer(2),
            ])]
        );
    }

    #[test]
    fn parse_dotted_list() {
        let lexer = Lexer::new("(f x . y)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_dotted_list(exprs![
                Expr::Symbol("f".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Symbol("y".to_string()),
            ])]
        );
    }

    #[test]
    fn parse_quasiquote_with_unquote() {
        let lexer = Lexer::new("`(list ,(+ 1 2) 4)");
        let tokens: Vec<_> = lexer.collect();
        let answer = tokens.into_iter().peekable();
        let parsed = parse(answer).unwrap();
        assert_eq!(
            parsed,
            vec![Expr::new_proper_list(exprs![
                Expr::Symbol("quasiquote".to_string()),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("list".to_string()),
                    Expr::new_proper_list(exprs![
                        Expr::Symbol("unquote".to_string()),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("+".to_string()),
                            Expr::Integer(1),
                            Expr::Integer(2),
                        ]),
                    ]),
                    Expr::Integer(4),
                ]),
            ])]
        );
    }
}
