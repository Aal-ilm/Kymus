//! Codebook module — handles word-to-token mapping
//! for the Kymus compression protocol.


use std::collections::HashMap;
use std::ptr::null;

const DEFAULT_WORDLIST: &str = include_str!("../../codebooks/english-60k.txt");

pub struct Token(pub u16);

pub struct Codebook{
    word_to_token: HashMap<String, u16>,
    token_to_word: HashMap<u16, String>
}

impl Codebook{
    pub fn new(wordlist: Option<&str>) -> Self {
        match wordlist {
            Some(wordlist) => Self::tokenize_words(wordlist),
            None => Self::tokenize_words(DEFAULT_WORDLIST),
        }
    }

    // Tokenizes the word list into a hashmap
    pub fn tokenize_words(wordlist: &str) -> Self{
        let total_words = DEFAULT_WORDLIST.lines().count();
        let mut word_to_token = HashMap::with_capacity(total_words);
        let mut token_to_word = HashMap::with_capacity(total_words);

        // setting the hashmap to words as tokens
        for(i, word) in DEFAULT_WORDLIST.lines().enumerate(){
            let word_token = (i +1) as u16;
            word_to_token.insert(word.to_string(), word_token);
            token_to_word.insert(word_token, word.to_string());
        }

        Codebook { word_to_token, token_to_word}
    }

    
    pub fn lookup_word(&self, word: &str) -> Option<Token> {
        self.word_to_token.get(word).copied().map(Token)
    }

}