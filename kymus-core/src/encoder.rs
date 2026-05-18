//! Encode module - handles Encoding of input text
//! for the Kymus compression protocol.

use crate::codebook::{Codebook, Token, CODEBOOK};

pub struct Encoder {
    pub text: Vec<String>,
    pub text_tokenized: Vec<u16>,
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
                }
            }

            None => {
                Encoder{
                    text: Vec::new(),
                    text_tokenized: Vec::new(),
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

        println!("HITT{}", "ohno");
        false // returns if empty
    }

    // Uses the value in the Encoder struct
    pub fn encode(&mut self) -> Vec<u16> {
        for word in self.text.iter(){
            match CODEBOOK.get().unwrap().get_token(word.as_str()) {
                Some(token) => self.text_tokenized.push(token.0),
                None => println!( "Word does not exist in Map: {}", word.as_str() ),
            }
        }
        self.text_tokenized.clone()
    }

    pub fn decode(&mut self) -> Vec<String> {
        let mut decoded: Vec<String> = Vec::new();
        for token in self.text_tokenized.iter() {
            decoded.push(CODEBOOK.get().unwrap().get_word(Token(*token)).unwrap().to_string());
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
        let mut encoder = Encoder::new(Some("test it today"), None);
        encoder.encode();

        println!("{:?}", encoder.text_tokenized);


        assert_eq!(encoder.text_tokenized[0], 514);
        assert_eq!(encoder.text_tokenized[1], 11);
        assert_eq!(encoder.text_tokenized[2], 592);
    }

    #[test]
    fn decode_test(){
        let mut tokens: Vec<u16> = Vec::new();
        tokens.push(514); // Expect: test
        tokens.push(11); // Expect: it
        tokens.push(592); // Expect: today

        let mut encoder = Encoder::new(None, None);
        encoder.text_tokenized = tokens;

        let list = encoder.decode();

        println!("{:?}",  list);

        assert_eq!(list[0], "test");
        assert_eq!(list[1], "it");
        assert_eq!(list[2], "today");
    }

    #[test]
    fn load_text_test(){
        let mut encoder = Encoder::new(None, None);
        encoder.load_text("test it tomorrow");
        encoder.encode();
        println!("{:?}", encoder.text_tokenized);
    }
}
