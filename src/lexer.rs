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
    let program2 = program.replace("(", " ( ").replace(")", " ) ").replace("'", " ' ");
    let words: Vec<String> = program2.split_whitespace().map(|s| s.to_string()).collect();

    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word.as_str() {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "'" => tokens.push(Token::Quote),
            _ => {
                let token = word
                    .parse::<i64>()
                    .map(Token::Integer)
                    .or_else(|_| word.parse::<f64>().map(Token::Float))
                    .unwrap_or_else(|_| Token::String(word.clone()));
                tokens.push(token);
            }
        }
    }
    tokens
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
    }
}
