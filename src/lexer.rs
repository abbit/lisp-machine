use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    LParen,
    RParen,
    Quote,
}

#[derive(PartialEq, Debug)]
pub enum LexicalError {
    UnexpectedEOF,
    UnexpectedRParen,
    UnclosedString,
}

pub fn lex(program: &str) -> Vec<Result<Token, LexicalError>> {
    let mut chars = program.chars().peekable();
    let mut tokens: Vec<Result<Token, LexicalError>> = Vec::new();
    let mut open_paren_count = 0;
    let mut has_error = false;
    while let Some(&ch) = chars.peek() {
        if has_error {
            break;
        }
        let result = match ch {
            '(' => {
                open_paren_count += 1;
                chars.next();
                Ok(Token::LParen)
            }
            ')' => {
                open_paren_count -= 1;
                chars.next();
                if open_paren_count < 0 {
                    Err(LexicalError::UnexpectedRParen)
                } else {
                    Ok(Token::RParen)
                }
            }
            '\'' => {
                chars.next();
                Ok(Token::Quote)
            }
            _ if ch.is_whitespace() => {
                chars.next();
                continue;
            }
            _ => finalize_token(&mut chars),
        };
        if let Err(_) = &result {
            has_error = true;
        }
        tokens.push(result);
    }

    if open_paren_count > 0 && !has_error {
        tokens.push(Err(LexicalError::UnexpectedEOF));
    }

    tokens
}

fn finalize_token(chars: &mut Peekable<Chars>) -> Result<Token, LexicalError> {
    let mut token_string = String::new();
    let mut is_string = false;
    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next();
            if is_string {
                is_string = false;
                break;
            } else {
                is_string = true;
                continue;
            }
        }
        if !is_string && (ch.is_whitespace() || ch == '(' || ch == ')' || ch == '\'') {
            break;
        }
        if is_string && ch.is_whitespace() {
            token_string.push(ch);
            chars.next();
            continue;
        }
        if is_string || (!is_string && !ch.is_whitespace()) {
            token_string.push(ch);
        }
        chars.next();
    }
    if is_string {
        return Err(LexicalError::UnclosedString);
    } else {
        if token_string
            .trim_start_matches('(')
            .trim_end_matches(')')
            .contains("\"")
        {
            return Err(LexicalError::UnclosedString);
        }

        Ok(token_string
            .parse::<i64>()
            .map(Token::Integer)
            .or_else(|_| token_string.parse::<f64>().map(Token::Float))
            .unwrap_or_else(|_| Token::String(token_string)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_complex() {
        let tokens = lex("(cos (* 3.14159 1))");
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("cos".to_string())),
                Ok(Token::LParen),
                Ok(Token::String("*".to_string())),
                Ok(Token::Float(3.14159)),
                Ok(Token::Integer(1)),
                Ok(Token::RParen),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn test_lex_quote() {
        let tokens = lex("'(1 2 3)");
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
    fn test_lex_long() {
        let tokens = lex("(+ (* 3 (+ (* 2 4)  (+ 3 5))) (+ (- 10 7) 6))");
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::String("*".to_string())),
                Ok(Token::Integer(3)),
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::String("*".to_string())),
                Ok(Token::Integer(2)),
                Ok(Token::Integer(4)),
                Ok(Token::RParen),
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::Integer(3)),
                Ok(Token::Integer(5)),
                Ok(Token::RParen),
                Ok(Token::RParen),
                Ok(Token::RParen),
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::LParen),
                Ok(Token::String("-".to_string())),
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
    fn test_unexpected_eof() {
        let result = lex("(+ 1 2");
        assert_eq!(
            result,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(2)),
                Err(LexicalError::UnexpectedEOF)
            ]
        );
    }

    #[test]
    fn test_string() {
        let result = lex(r#"(display "Hello, world!")"#);
        assert_eq!(
            result,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("display".to_string())),
                Ok(Token::String("Hello, world!".to_string())),
                Ok(Token::RParen),
            ]
        );
    }

    #[test]
    fn test_unclosed_string() {
        let result = lex(r#"(display "hello)"#);
        assert_eq!(
            result,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("display".to_string())),
                Err(LexicalError::UnclosedString),
            ]
        );
    }

    #[test]
    fn test_unexpected_close_paren() {
        let result = lex("(+ 1 2))");
        assert_eq!(
            result,
            vec![
                Ok(Token::LParen),
                Ok(Token::String("+".to_string())),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(2)),
                Ok(Token::RParen),
                Err(LexicalError::UnexpectedRParen),
            ]
        );
    }

    #[test]
    fn test_void() {
        let result = lex("");
        assert_eq!(
            result,
            vec![]
        );
    }
}
