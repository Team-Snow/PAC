use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

const TEST_STRINGS: &'static [&'static str] = &["azd", "vvv", "hack", "frost", "snow"];

fn string_to_hash() -> Vec<Vec<u8>> {
    let mut hashes = Vec::new();
    for string in TEST_STRINGS {
        let mut hasher = Sha256::new();
        hasher.update(string);
        hashes.push(hasher.finalize().to_vec())
    }
    hashes
}

/// Contains the state of each hash
/// (Solution, Hash, Served, Completed, Workers)
/// Todo: Convert tuple to struct
type HashState = (&'static str, Vec<u8>, usize, usize, Vec<usize>);

pub struct State {
    pub nodes: usize,
    pub hashes: Vec<HashState>,
}

impl State {
    pub fn new() -> Self {
        let mut hashes = string_to_hash();
        let mut drain = hashes.drain(0..);
        Self {
            nodes: 0,
            hashes: TEST_STRINGS
                .iter()
                .map(|v| (*v, drain.next().unwrap(), 0, 0, Vec::new()))
                .collect(),
        }
    }

    pub fn connect(&mut self) -> usize {
        self.nodes += 1;
        self.nodes
    }

    pub fn disconnect(&mut self, id: usize) {
        self.nodes -= 1;
        for (_, _, _, _, ids) in &mut self.hashes {
            ids.drain_filter(|tid| *tid == id);
        }
    }

    pub fn request(&mut self, id: usize) -> Vec<u8> {
        let hashes = &mut self.hashes;
        let len = hashes.len();

        // Get random task
        let mut rng = thread_rng();
        let (_, hash, served, _, nodes) = &mut hashes[rng.gen_range(0..len)];

        // Assign node to task
        *served += 1;
        nodes.push(id);

        // Return task
        hash.clone()
    }
}
