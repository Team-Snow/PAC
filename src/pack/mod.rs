use sha2::{Digest, Sha256};

pub struct Pack {}

impl Pack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self, hash: Vec<u8>) -> Option<String> {
        None
    }
}
