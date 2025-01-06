use crate::tokenizer::tokenizer::{Token, TokenType};
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;
use stop_words::get as stop_words;

pub trait TokenFilter {
    fn filter(&self, input: String) -> Vec<Token>;
}

pub struct StemmerTokenFilter {
    stemmer: Stemmer,
}

impl StemmerTokenFilter {
    pub fn new() -> Self {
        Self {
            stemmer: Stemmer::create(Algorithm::English),
        }
    }
}

impl TokenFilter for StemmerTokenFilter {
    fn filter(&self, input: String) -> Vec<Token> {
        let stemmed_token = self.stemmer.stem(&input);
        vec![Token {
            token: stemmed_token.to_string(),
            token_type: TokenType::Term,
        }]
    }
}

pub struct StopWordRemover {
    stopwords: HashSet<String>,
}

impl StopWordRemover {
    pub fn new() -> Self {
        let words = stop_words(stop_words::LANGUAGE::English);
        let stopwords = words.into_iter().collect::<HashSet<String>>();
        Self { stopwords }
    }
}

impl TokenFilter for StopWordRemover {
    fn filter(&self, input: String) -> Vec<Token> {
        if self.stopwords.contains(&input) {
            vec![]
        } else {
            vec![Token {
                token: input,
                token_type: TokenType::Term,
            }]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stemmer_token_filter() {
        let filter = StemmerTokenFilter::new();
        let input = "running".to_string();
        let tokens = filter.filter(input);
        assert_eq!(
            tokens,
            vec![Token {
                token: "run".to_string(),
                token_type: TokenType::Term
            }]
        );
    }

    #[test]
    fn test_stopword_removal() {
        let stopword_filter = StopWordRemover::new();

        let result = stopword_filter.filter("the".to_string());
        assert!(result.is_empty(),);

        let result: Vec<Token> = stopword_filter.filter("term".to_string());
        assert_eq!(result[0].token_type, TokenType::Term);
    }
}
