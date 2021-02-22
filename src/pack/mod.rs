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
        log(&format!("Attempting to find {}", u8_to_string(&hash)));
        // Initialize our starting string
        let mut start = b"a".to_vec();

        // Search for secret by incrementally hashing our string
        for _ in 0..15000000 {
            let mut hasher = Sha256::new();
            hasher.update(&start);
            let result = hasher.finalize();
            let result_vec = result.to_vec();
            if result_vec == hash {
                let result = String::from_utf8(start).unwrap();
                log(&format!(
                    "Solved! sha256({}) = {}",
                    result,
                    u8_to_string(&result_vec)
                ));
                return Some(result);
            }
            shift(&mut start);
        }

        None
    }
}

/// Converts a u8 vector into a nice hex string
fn u8_to_string(vec: &Vec<u8>) -> String {
    let mut string = String::new();
    for byte in vec {
        string.push_str(&format!("{:x}", byte))
    }
    string
}

/// Increments a string from a-z
pub fn shift(vec: &mut Vec<u8>) {
    let mut first_z = *vec.first().unwrap() == b'z';
    let mut iter = vec.iter_mut();
    while let Some(item) = iter.next() {
        match item {
            b'z' => {
                *item -= b'z' - b'a';
            }
            _ => {
                *item += 1;
                first_z = false;
                break;
            }
        }
    }

    if first_z {
        vec.push(b'a');
    }
}
