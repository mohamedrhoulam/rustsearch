use std::collections::VecDeque;
use std::iter::Iterator;
use std::option::Option;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token: String,
}

pub trait TokenStream {
    fn next(&mut self) -> Option<Token>;
}

pub struct EmptyTokenStream;

impl TokenStream for EmptyTokenStream {
    fn next(&mut self) -> Option<Token> {
        None
    }
}

pub struct SingleTokenStream {
    token: Option<Token>,
}

impl SingleTokenStream {
    pub fn new(token: String) -> Self {
        Self {
            token: Some(Token { token }),
        }
    }
}

impl TokenStream for SingleTokenStream {
    fn next(&mut self) -> Option<Token> {
        self.token.take()
    }
}

pub struct VecTokenStream {
    tokens: VecDeque<Token>,
}

impl VecTokenStream {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            tokens: tokens.into_iter().map(|token| Token { token }).collect(),
        }
    }
}

impl TokenStream for VecTokenStream {
    fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

pub struct TokenIterator<T: TokenStream> {
    tokenizer: T,
    pos: usize,
    current_token: Option<Token>,
}

impl<T: TokenStream> TokenIterator<T> {
    pub fn new(mut tokenizer: T) -> Self {
        let current_token: Option<Token> = tokenizer.next();
        Self {
            tokenizer,
            pos: 0,
            current_token,
        }
    }

    pub fn dereference(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn increment(&mut self) {
        if self.current_token.is_some() {
            self.current_token = self.tokenizer.next();
            self.pos += 1;
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        std::ptr::eq(&self.tokenizer, &other.tokenizer)
            && self.pos == other.pos
            && self.current_token == other.current_token
    }

    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl<T: TokenStream> Iterator for TokenIterator<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Option<Token> = self.current_token.clone();
        self.increment();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_token_stream() {
        let mut stream: EmptyTokenStream = EmptyTokenStream;
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_single_token_stream() {
        let mut stream: SingleTokenStream = SingleTokenStream::new("hello".to_string());
        assert_eq!(
            stream.next(),
            Some(Token {
                token: "hello".to_string()
            })
        );
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_vec_token_stream() {
        let mut stream: VecTokenStream =
            VecTokenStream::new(vec!["hello".to_string(), "world".to_string()]);

        assert_eq!(
            stream.next(),
            Some(Token {
                token: "hello".to_string()
            })
        );
        assert_eq!(
            stream.next(),
            Some(Token {
                token: "world".to_string()
            })
        );
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_token_iterator() {
        let stream: VecTokenStream =
            VecTokenStream::new(vec!["token1".to_string(), "token2".to_string()]);
        let mut iterator: TokenIterator<VecTokenStream> = TokenIterator::new(stream);

        assert_eq!(
            iterator.next(),
            Some(Token {
                token: "token1".to_string()
            })
        );
        assert_eq!(
            iterator.next(),
            Some(Token {
                token: "token2".to_string()
            })
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_token_iterator_equality() {
        let stream1: VecTokenStream =
            VecTokenStream::new(vec!["token1".to_string(), "token2".to_string()]);
        let stream2: VecTokenStream =
            VecTokenStream::new(vec!["token1".to_string(), "token2".to_string()]);

        let iter1: TokenIterator<VecTokenStream> = TokenIterator::new(stream1);
        let iter2: TokenIterator<VecTokenStream> = TokenIterator::new(stream2);

        assert!(iter1.not_equals(&iter2));
    }
}
