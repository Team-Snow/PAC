use rand::{thread_rng, Rng};
use serde::Serialize;
use sha2::{Digest, Sha256};

const TEST_STRINGS: &'static [&'static str] = &[
    "hack", "frost", "snow", "arena", "slack", "rust", "wasm", "pop",
];

/// Converts test strings into sha256
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
#[derive(Serialize)]
pub struct HashState {
    text: &'static str,
    hash: Vec<u8>,
    served: usize,
    completed: usize,
    nodes: Vec<usize>,
}

/// Defines server state
///
/// Todo: Currently the state is locked by the server during all operations on this object.
///     Exploring some internal mutability structures should increase performance, particularly with longer running tasks on the server.
#[derive(Serialize)]
pub struct State {
    ids: usize,
    pub nodes: usize,
    pub hashes: Vec<HashState>,
}

/// Contains the state of the sever and tasks and functionality for distributing tasks
impl State {
    pub fn new() -> Self {
        let mut hashes = string_to_hash();
        let mut drain = hashes.drain(0..);
        Self {
            ids: 0,
            nodes: 0,
            hashes: TEST_STRINGS
                .iter()
                .map(|v| HashState {
                    text: *v,
                    hash: drain.next().unwrap(),
                    served: 0,
                    completed: 0,
                    nodes: Vec::new(),
                })
                .collect(),
        }
    }

    /// Creates a ID for new nodes
    pub fn connect(&mut self) -> usize {
        self.nodes += 1;
        self.ids += 1;
        println!("Node connected and assigned id {}", self.ids);
        self.ids
    }

    /// Disconnects node and frees tasks
    pub fn disconnect(&mut self, id: usize) {
        self.nodes -= 1;
        for hash in &mut self.hashes {
            hash.nodes.drain_filter(|tid| *tid == id);
        }
    }

    /// Assigns a hash to a node for computation
    /// Tasks should be divided into some form of shared work, but we will just serve simple hashes
    pub fn request(&mut self, id: usize) -> Vec<u8> {
        let hashes = &mut self.hashes;
        let len = hashes.len();

        // Get random task
        let mut rng = thread_rng();
        let task = &mut hashes[rng.gen_range(0..len)];

        // Assign node to task
        task.served += 1;
        task.nodes.push(id);

        // println!(
        //     "Sending \"{}\" to node {} as {:?}",
        //     task.text, id, task.hash
        // );

        // Return task
        task.hash.clone()
    }

    /// Checks if a result is correct and removes node from task
    pub fn resolve(&mut self, id: usize, result: String) {
        let hash = self.hashes.iter_mut().find(|hash| *hash.text == result);
        if let Some(mut hash) = hash {
            // println!("Node {} solved {}", id, result);
            hash.completed += 1;
            hash.nodes.drain_filter(|tid| *tid == id);
        } else {
            // Error validating result
            for hash in &mut self.hashes {
                hash.nodes.drain_filter(|tid| *tid == id);
            }
        }
    }
}
