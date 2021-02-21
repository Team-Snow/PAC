use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PacEvent {
    pub event: EventType,
}

impl PacEvent {
    pub fn request() -> Self {
        Self {
            event: EventType::Request,
        }
    }

    pub fn start(hash: Vec<u8>) -> Self {
        Self {
            event: EventType::Start(hash),
        }
    }

    pub fn resolved(string: String) -> Self {
        Self {
            event: EventType::Resolved(string),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    Start(Vec<u8>),
    Stop,
    Resolved(String),
    Request,
}
