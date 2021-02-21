use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

const TEST_STRINGS: &'static [&'static str] = &["hack", "frost", "snow", "arena"];

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

/// Defines server state
///
/// Currently the state is locked by the server all operations on this object. Exploring some internal
/// mutability structures should increase performance, particularly with longer running tasks on the server.
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
                .map(|v| (*v, drain.next().unwrap(), 0, 0, Vec::new()))
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
        for (_, _, _, _, ids) in &mut self.hashes {
            ids.drain_filter(|tid| *tid == id);
        }
    }

    /// Assigns a hash to a node for computation
    /// Tasks should be divided into some form of shared work, but we will just serve simple hashes
    pub fn request(&mut self, id: usize) -> Vec<u8> {
        let hashes = &mut self.hashes;
        let len = hashes.len();

        // Get random task
        let mut rng = thread_rng();
        let (string, hash, served, _, nodes) = &mut hashes[rng.gen_range(0..len)];

        // Assign node to task
        *served += 1;
        nodes.push(id);

        println!("Sending: {} as {:?}", string, hash);

        // Return task
        hash.clone()
    }

    /// Checks if a result is correct and removes node from task
    pub fn resolve(&mut self, id: usize, result: String) {
        println!("got result {}", result);
        let hash = self.hashes.iter_mut().find(|(string, _, _, _, _)| **string == result);
        if let Some(mut hash) = hash {
            let (_, _, _, completed, ids) = &mut hash;
            *completed += 1;
            ids.drain_filter(|tid| *tid == id);
        } else {
            // Error validating result
            for (_, _, _, _, ids) in &mut self.hashes {
                ids.drain_filter(|tid| *tid == id);
            }
        }
    }
}
