use colored::Colorize;
use crate::encoder::Encoder;

const DEFAULT_BUFFER_SIZE: usize = 200; // Meshtastic max payload size of 200 bytes
const COMPRESSION_VERSION: u8 = 0x01;
const COMPRESSION_HEADER: u8 = 0xAA;

struct Packet{
    encoder: Encoder,
    payload: Vec<u8>,
    buffer: usize
}

impl Packet {
    pub fn new(buffer: Option<usize>) -> Self {
        let b_size = buffer.unwrap_or(DEFAULT_BUFFER_SIZE);
        Packet{
            encoder: Encoder::new(None, None),
            payload: vec![0u8; b_size],
            buffer: b_size
        }
    }

    pub fn construct(&mut self, text: &str) -> Option<Vec<u8>> {
        self.payload.clear(); //clear payload

        if text.split_whitespace().count() > self.buffer {
            println!("{}", format!("[ERROR] Text exceeds buffer: {} bytes > {} limit", text.len(), self.buffer).red().bold());
            return None;
        }

        self.encoder.load_text(text);
        self.encoder.encode();

        for token in self.encoder.tokenized_tostring(){
            println!("{}", token);
        }

        Some(self.payload.clone())
    }

    pub fn _set_buffer(&mut self, buffer: usize) {
        self.buffer = buffer;
    }

    pub fn _get_buffer(&self) -> usize {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;
    #[test]
    fn construct_test(){
        let mut packet = Packet::new(None);
        let pak =packet.construct("A hello how are you ejwr0, coo coo! !").unwrap();
        println!("{:?}", pak);
    }
}


