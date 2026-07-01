//! Encode module - handles Encoding of input text
//! for the Kymus compression protocol.

use colored::Colorize;
use crate::codebook::{Codebook, Token, CODEBOOK};

/// Represents a single encoded word in the Kymus protocol.
/// Either a 16-bit dictionary token or a raw UTF-8 fallback
/// for words not found in the codebook.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EncodedWord {
    Tokenized(u16),
    Raw(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Encoder {
    pub text: Vec<String>,
    pub text_tokenized: Vec<EncodedWord>,
    pub text_tokenized_tostring: Option<Vec<String>>,
}

// Encoder handles encoding and decoding of the raw string or tokenized payloads.
impl Encoder {
    pub fn new(text: Option<&str>, codebook_path: Option<&str>) -> Self {
        // Setting codebook singleton reference
        match codebook_path {
            Some(path) => {
                CODEBOOK.set(Codebook::new(Some(path))).ok();
            }
            None => {CODEBOOK.set(Codebook::new(None)).ok();}
        }

        match text {
            Some(text) => {
                Encoder{
                text: text.split_whitespace().map(|t| t.to_string()).collect(),
                text_tokenized: Vec::new(),
                text_tokenized_tostring: None,
                }
            }

            None => {
                Encoder{
                    text: Vec::new(),
                    text_tokenized: Vec::new(),
                    text_tokenized_tostring: None,
                }
            }
        }
    }

    pub fn load_text(&mut self, text: &str) -> bool{
        if !text.is_empty(){
            self.text = text.split_whitespace().map(|t| t.to_string()).collect();
            self.text_tokenized.clear();
            return true
        }
        false // returns if empty
    }

    // Uses the value in the Encoder struct
    pub fn encode(&mut self) -> Vec<EncodedWord> {
        for word in self.text.iter(){
            match CODEBOOK.get().unwrap().get_token(word.as_str()) {
                Some(token) => {
                    self.text_tokenized.push(EncodedWord::Tokenized(token.0));
                },
                None => {
                    println!("{}", format!("[WARN] Not in dictionary: {}", word.as_str()).yellow().bold());
                    self.text_tokenized.push(EncodedWord::Raw(word.clone()));
                },
            }
        }
        self.text_tokenized.clone()
    }

    pub fn decode(&self) -> Vec<String> {
        let mut decoded: Vec<String> = Vec::new();
        for token in self.text_tokenized.iter() {
            match token {
                EncodedWord::Tokenized(t) => {
                    match CODEBOOK.get().unwrap().get_word(Token(*t)) {
                        Some(word) => {
                            decoded.push(word.to_string());
                        }
                        None => println!("{}", format!("[WARN] Not in dictionary: {}", t).yellow())
                    }
                }
                EncodedWord::Raw(word) => {
                    decoded.push(word.clone());
                }
            }
        }
        decoded
    }

    pub fn tokenized_tostring(&mut self) -> Vec<String> {
        if !self.text_tokenized_tostring.is_none() {
            return self.text_tokenized_tostring.clone().unwrap();
        }

        let mut words: Vec<String> = Vec::new();
        for word in self.text_tokenized.iter() {
            match word {
                EncodedWord::Tokenized(t) => {
                    words.push(t.to_string());
                }
                EncodedWord::Raw(word) => {
                    words.push(word.clone());
                }
            }
        }
        self.text_tokenized_tostring = Some(words.clone());
        words
    }
}

//     ---------------------- TESTS ----------------------

#[cfg(test)]
mod tests {
    use super::*;

    const UNFOUND_RAW_WORD: &str = "mansd09";
    #[test]
    fn encode_test(){
        let input = format!("test it today {}", UNFOUND_RAW_WORD);
        let mut encoder = Encoder::new(Some(&input), None);
        encoder.encode();

        println!("{:?}", encoder.text_tokenized);

        assert_eq!(encoder.text_tokenized[0], EncodedWord::Tokenized(514));
        assert_eq!(encoder.text_tokenized[1], EncodedWord::Tokenized(11));
        assert_eq!(encoder.text_tokenized[2], EncodedWord::Tokenized(592));
        assert_eq!(encoder.text_tokenized[3], EncodedWord::Raw(UNFOUND_RAW_WORD.to_string()));
    }

    #[test]
    fn decode_test(){
        let mut tokens: Vec<EncodedWord> = Vec::new();
        tokens.push(EncodedWord::Tokenized(514));   // Expect: test
        tokens.push(EncodedWord::Tokenized(11));    // Expect: it
        tokens.push(EncodedWord::Tokenized(592));   // Expect: today
        tokens.push(EncodedWord::Raw(UNFOUND_RAW_WORD.to_string()));

        let mut encoder = Encoder::new(None, None);
        encoder.text_tokenized = tokens;

        let list = encoder.decode();

        println!("{:?}",  list);

        assert_eq!(list[0], "test");
        assert_eq!(list[1], "it");
        assert_eq!(list[2], "today");
        assert_eq!(list[3], "mansd09");
    }

    #[test]
    fn load_text_test(){
        let mut encoder = Encoder::new(None, None);
        encoder.load_text("test it tomorrow");
        encoder.encode();
        println!("{:?}", encoder.text_tokenized);
    }
}
