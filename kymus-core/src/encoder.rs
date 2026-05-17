//! Encode module - handles Encoding of input text
//! for the Kymus compression protocol.

use std::ptr::{null, null_mut};
use crate::codebook::{Codebook, Token};

pub struct Encoder {
    pub text: Vec<String>,
    pub text_tokenized: Vec<u16>,
    pub codebook: Codebook,
}

impl Encoder {
    pub fn new(text: &str) -> Self {
        Encoder{
            text: text.split_whitespace().map(|t| t.to_string()).collect(),
            text_tokenized: Vec::new(),
            codebook: Codebook::new(None),
        }
    }

    // Uses the value in the Encoder struct
    pub fn encode(&mut self) -> Vec<u16> {
        for word in self.text.iter(){
            match self.codebook.get_token(word.as_str()) {
                Some(token) => self.text_tokenized.push(token.0),
                None => println!( "Word does not exist in Map: {}", word.as_str() ),
            }
            // self.text_tokenized.push(self.codebook.get_token(word.as_str()).unwrap().0)
        }
        self.text_tokenized.clone()
    }

    pub fn decode(&mut self) -> Vec<String> {
        let mut decoded: Vec<String> = Vec::new();
        for token in self.text_tokenized.iter(){
            decoded.push(self.codebook.get_word(Token(*token)).unwrap().to_string());
        }
        decoded
    }
}

//     ---------------------- TESTS ----------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test(){
        let mut encoder = Encoder::new("test it today");
        encoder.encode();

        for item in encoder.text_tokenized.iter(){
            println!("{}", item);
        }
    }
}
