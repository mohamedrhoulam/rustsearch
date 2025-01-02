use regex::Regex;

pub fn tokenizer(text: &str) -> Vec<String> {
    let re = match Regex::new(r"\w+") {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Failed to create regex: {}", err);
            return Vec::new();
        }
    };

    re.find_iter(&text.to_lowercase())
        .map(|mat| mat.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "Hello, World!";
        let tokens = tokenizer(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }
}
