//! Encode module - handles Encoding of input text
//! for the Kymus compression protocol.

use crate::codebook::{Codebook, Token};

pub struct Encoder {
    pub text: Vec<String>,
    pub text_tokenized: Vec<u16>,
    pub codebook: Codebook,
}

impl Encoder {
    pub fn new(text: Option<&str>) -> Self {
        match text {
            Some(text) => {
                Encoder{
                text: text.split_whitespace().map(|t| t.to_string()).collect(),
                text_tokenized: Vec::new(),
                codebook: Codebook::new(None),
                }
            }

            None => {
                Encoder{
                    text: Vec::new(),
                    text_tokenized: Vec::new(),
                    codebook: Codebook::new(None)
                }
            }
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
        for token in self.text_tokenized.iter() {
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
        let mut encoder = Encoder::new(Some("test it today"));
        encoder.encode();

        for item in encoder.text_tokenized.iter(){
            println!("{}", item);
        }
    }

    #[test]
    fn decode_test(){
        let mut tokens: Vec<u16> = Vec::new();
        tokens.push(514); // Expect: test
        tokens.push(11); // Expect: it
        tokens.push(592); // Expect: today

        let mut encoder = Encoder::new(None);
        encoder.text_tokenized = tokens;

        let list = encoder.decode();

        for item in list.clone() {
                println!("{}", item);
        }

        assert_eq!(list[0], "test");
        assert_eq!(list[1], "it");
        assert_eq!(list[2], "today");


    }
}
