
use crate::encoder::Encoder;

const DEFAULT_BUFFER_SIZE: u8 = 200;

struct Packet{
    encoder: Encoder,
    payload: Vec<u8>,
    buffer: u8
}

impl Packet {
    pub fn new(buffer: Option<u8>) -> Self {
        Packet{
            encoder: Encoder::new(None),
            payload: Vec::new(),
            buffer: buffer.unwrap_or(DEFAULT_BUFFER_SIZE)
        }
    }

    pub fn encode(&mut self, text: &str){
    
    }
    pub fn set_buffer(&mut self, buffer: u8) {
        self.buffer = buffer;
    }

    pub fn get_buffer(&self) -> u8 {
        self.buffer
    }
}


