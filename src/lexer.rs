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
    let mut tokens: Vec<Token> = Vec::new();
    let mut temp_string = String::new();

    for ch in program.chars() {
        match ch {
            '(' => {
                finalize_token(&mut tokens, &mut temp_string);
                tokens.push(Token::LParen);
            }
            ')' => {
                finalize_token(&mut tokens, &mut temp_string);
                tokens.push(Token::RParen);
            }
            '\'' => {
                finalize_token(&mut tokens, &mut temp_string);
                tokens.push(Token::Quote);
            }
            whitespace if whitespace.is_whitespace() => {
                finalize_token(&mut tokens, &mut temp_string)
            }
            _ => temp_string.push(ch),
        }
    }
    finalize_token(&mut tokens, &mut temp_string);
    tokens
}

fn finalize_token(tokens: &mut Vec<Token>, temp_string: &mut String) {
    if !temp_string.is_empty() {
        let token = temp_string
            .parse::<i64>()
            .map(Token::Integer)
            .or_else(|_| temp_string.parse::<f64>().map(Token::Float))
            .unwrap_or_else(|_| Token::String(temp_string.clone()));
        tokens.push(token);
        temp_string.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let tokens_int = lex("(+ 1 2)");
        assert_eq!(
            tokens_int,
            vec![
                Token::LParen,
                Token::String("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );

        let tokens_float = lex("(+ 1.0 2.5)");
        assert_eq!(
            tokens_float,
            vec![
                Token::LParen,
                Token::String("+".to_string()),
                Token::Float(1.0),
                Token::Float(2.5),
                Token::RParen,
            ]
        );

        let tokens_complex = lex("(cos (* 3.14159 1))");
        assert_eq!(
            tokens_complex,
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

        let tokens_quote = lex("'(1 2 3)");
        assert_eq!(
            tokens_quote,
            vec![
                Token::Quote,
                Token::LParen,
                Token::Integer(1),
                Token::Integer(2),
                Token::Integer(3),
                Token::RParen,
            ]
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
        assert_eq!(
            tokens_lot,
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
