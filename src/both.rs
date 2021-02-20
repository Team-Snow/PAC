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

    pub fn start() -> Self {
        Self {
            event: EventType::Start,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    Start,
    Stop,
    Update,
    Resolved,
    Request,
}
