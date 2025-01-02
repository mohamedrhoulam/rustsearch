use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Abbreviation,
    Possessive,
    Term,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token: String,
    token_type: TokenType,
}

pub fn classify_token(token: &str) -> TokenType {
    if Regex::new(r"^([a-zA-Z]+\.){2,}$").unwrap().is_match(token) {
        TokenType::Abbreviation
    } else if Regex::new(r"^[a-zA-Z0-9]+('[a-zA-Z]+)$").unwrap().is_match(token) {
        TokenType::Possessive
    } else if Regex::new(r"^[a-zA-Z0-9]+$").unwrap().is_match(token) {
        TokenType::Term
    } else {
        TokenType::Invalid
    }
}

pub fn transform_token(token: &str, token_type: &TokenType) -> String {
    match token_type {
        TokenType::Abbreviation => token.replace('.', ""), 
        TokenType::Possessive => token.split('\'').next().unwrap_or(token).to_string(), 
        _ => token.to_string(),
    }
}

pub fn tokenizer(text: &str) -> Vec<Token> {
    let re = match Regex::new(r"\w+('\w+)?|[a-zA-Z]+\.+") {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Failed to create regex: {}", err);
            return Vec::new();
        }
    };

    re.find_iter(&text.to_lowercase())
        .map(|mat| {
            let raw_token = mat.as_str();
            let token_type = classify_token(raw_token);
            let processed_token = transform_token(raw_token, &token_type);
            Token {
                token: processed_token,
                token_type,
            }
        })
        .filter(|token| token.token_type != TokenType::Invalid) 
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_possessive() {
        let text = "John's cat";
        let tokens = tokenizer(text);
        assert_eq!(
            tokens,
            vec![
                Token {
                    token: "john".to_string(),
                    token_type: TokenType::Possessive
                },
                Token {
                    token: "cat".to_string(),
                    token_type: TokenType::Term
                }
            ]
        );
    }

    #[test]
    fn test_tokenizer_abbreviation() {
        let text = "R.D.C. is a country in Africa.";
        let tokens = tokenizer(text);
        assert_eq!(
            tokens,
            vec![
                Token {
                    token: "RDC".to_string(),
                    token_type: TokenType::Abbreviation
                },
                Token {
                    token: "is".to_string(),
                    token_type: TokenType::Term
                },
                Token {
                    token: "a".to_string(),
                    token_type: TokenType::Term
                },
                Token {
                    token: "country".to_string(),
                    token_type: TokenType::Term
                },
                Token {
                    token: "in".to_string(),
                    token_type: TokenType::Term
                },
                Token {
                    token: "africa".to_string(),
                    token_type: TokenType::Term
                }
            ]
        );
    }
}