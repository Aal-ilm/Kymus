//! Codebook module - handles word-to-token mapping
//! for the Kymus compression protocol.
use std::collections::HashMap;

const DEFAULT_WORDLIST: &str = include_str!("../../codebooks/english-60k.txt");

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token(pub u16);

#[derive(Debug, PartialEq, Clone)]
pub struct Codebook{
    word_to_token: HashMap<String, u16>,
    token_to_word: HashMap<u16, String>
}

impl Codebook{
    pub fn new(wordlist: Option<&str>) -> Self { // Constructor
        match wordlist {
            Some(wordlist) => Self::tokenize_words(wordlist),
            None => Self::tokenize_words(DEFAULT_WORDLIST),
        }
    }

    // Tokenizes the word list into a hashmap
    pub fn tokenize_words(wordlist: &str) -> Self{
        let total_words = wordlist.lines().count();
        let mut word_to_token = HashMap::with_capacity(total_words);
        let mut token_to_word = HashMap::with_capacity(total_words);

        // setting the hashmap to words as tokens
        for(i, word) in wordlist.lines().enumerate(){
            let word_token = (i +1) as u16;
            word_to_token.insert(word.to_string(), word_token);
            token_to_word.insert(word_token, word.to_string());
        }
        Codebook { word_to_token, token_to_word}
    }

    pub fn get_token(&self, word: &str) -> Option<Token> {
        self.word_to_token.get(word).copied().map(Token)
    }

    pub fn get_word(&self, token: Token) -> Option<&str> {
        self.token_to_word.get(&token.0).map(|s| s.as_str())
    }
}

//     ---------------------- TESTS ----------------------

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn get_token_test() {
        let words = "hello\nworld\nthis\nis\na\ntest";
        let book = Codebook::new(Some(words));

        assert_eq!(book.get_token("hello"), Some(Token(1)));  // Pass
        assert_eq!(book.get_token("world"), Some(Token(2)));  // Pass
        assert_eq!(book.get_token("this"),  Some(Token(3)));  // Pass
        assert_eq!(book.get_token("is"),    Some(Token(4)));  // Pass
        assert_ne!(book.get_token("hello"), Some(Token(5)));  // Fail
    }

    #[test]
    fn get_word_test() {
        let words = "hello\nworld\nthis\nis\na\ntest";
        let book = Codebook::new(Some(words));
        assert_eq!(book.get_word(Token(1)), Some("hello")); // Pass
        assert_ne!(book.get_word(Token(2)), Some("test"));  // Fail
    }

}
