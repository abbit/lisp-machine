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

pub fn lex(program: &str) -> Vec<Token> {
    let mut chars = program.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '\'' => {
                tokens.push(Token::Quote);
                chars.next();
            }
            whitespace if whitespace.is_whitespace() => {
                chars.next();
            }
            _ => {
                tokens.push(finalize_token(&mut chars));
            }
        }
    }
    tokens
}

fn finalize_token(chars: &mut Peekable<Chars>) -> Token {
    let mut token_string = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() || ch == '(' || ch == ')' || ch == '\'' {
            break;
        }
        token_string.push(ch);
        chars.next();
    }
    token_string
        .parse::<i64>()
        .map(Token::Integer)
        .or_else(|_| token_string.parse::<f64>().map(Token::Float))
        .unwrap_or_else(|_| Token::String(token_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_integer() {
        let tokens = lex("(+ 1 2)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_lex_float() {
        let tokens = lex("(+ 1.0 2.5)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("+".to_string()),
                Token::Float(1.0),
                Token::Float(2.5),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_lex_complex() {
        let tokens = lex("(cos (* 3.14159 1))");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("cos".to_string()),
                Token::LParen,
                Token::String("*".to_string()),
                Token::Float(3.14159),
                Token::Integer(1),
                Token::RParen,
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_lex_quote() {
        let tokens = lex("'(1 2 3)");
        assert_eq!(
            tokens,
            vec![
                Token::Quote,
                Token::LParen,
                Token::Integer(1),
                Token::Integer(2),
                Token::Integer(3),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_lex_long() {
        let tokens = lex("(+ (* 3
            (+ (* 2 4)
               (+ 3 5)
            )
         )
         (+ (- 10 7)
            6
         )
      )");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::String("+".to_string()),
                Token::LParen,
                Token::String("*".to_string()),
                Token::Integer(3),
                Token::LParen,
                Token::String("+".to_string()),
                Token::LParen,
                Token::String("*".to_string()),
                Token::Integer(2),
                Token::Integer(4),
                Token::RParen,
                Token::LParen,
                Token::String("+".to_string()),
                Token::Integer(3),
                Token::Integer(5),
                Token::RParen,
                Token::RParen,
                Token::RParen,
                Token::LParen,
                Token::String("+".to_string()),
                Token::LParen,
                Token::String("-".to_string()),
                Token::Integer(10),
                Token::Integer(7),
                Token::RParen,
                Token::Integer(6),
                Token::RParen,
                Token::RParen,
            ]
        );
    }
}
