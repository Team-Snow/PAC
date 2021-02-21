use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Pack {}

impl Pack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self, hash: Vec<u8>) -> Option<String> {
        // Initialize our starting string
        let mut start = b"a".to_vec();

        // Search for secret by incrementally hashing our string
        for _ in 0..15000000 {
            let mut hasher = Sha256::new();
            hasher.update(&start);
            let result = hasher.finalize();
            let result_vec = result.to_vec();
            if result_vec == hash {
                return Some(String::from_utf8(start).unwrap());
            }
            shift(&mut start);
        }

        None
    }
}

/// Increments a string from a-z
pub fn shift(vec: &mut Vec<u8>) {
    let mut first_z = *vec.first().unwrap() == b'z';
    let mut iter = vec.iter_mut();
    while let Some(item) = iter.next() {
        match item {
            b'z' => {
                *item -= b'z' - b'a';
            },
            _ => {
                *item += 1;
                first_z = false;
                break;
            },
        }
    }

    if first_z {
        vec.push(b'a');
    }
}