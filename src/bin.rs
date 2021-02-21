#![feature(drain_filter)]

#[allow(dead_code)]
mod both;
mod server;

use crate::both::*;
use crate::server::State;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::WebSocket;

pub fn main() {
    println!("Starting PAC server!");

    // Start server on localhost
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Initialize server state
    let state = Arc::new(Mutex::new(State::new()));

    // Loop over incoming connections
    for stream in server.incoming() {
        // Clone state to move into thread
        let state_ref = state.clone();

        // Spawn a thread for each connection
        // Todo: Connections should be moved to a async thread pool instead of using system threads
        spawn(move || {
            // Accept connection
            let mut websocket = accept(stream.unwrap()).unwrap();

            // Assign ID to node
            let mut id: usize = 0;

            // Continuously try to read messages from the connection
            loop {
                let next = websocket.read_message();
                if let Ok(ref msg) = next {
                    if msg.is_text() {
                        let msg_text = msg.to_text().unwrap();

                        // Allow browser clients to fetch state
                        // Todo: Information should be broadcasted instead of polled
                        if msg.to_text().unwrap() == "fetch" {
                            let lock = state_ref.lock().unwrap();
                            let state = serde_json::to_string(&*lock).unwrap();
                            websocket.write_message(state.into()).unwrap();
                            continue;
                        }

                        // Deserialize message into a PacEvent
                        // Todo: Unwrapping incorrect messages should kill the connection more gracefully
                        let msg: PacEvent = serde_json::from_str(&msg_text).unwrap();

                        // Respond to client events
                        match msg.event {
                            EventType::Request => {
                                // Assign ID to node
                                if id == 0 {
                                    id = {
                                        let mut state = state_ref.lock().unwrap();
                                        state.connect()
                                    };
                                }
                                let mut state = state_ref.lock().unwrap();
                                send(&mut websocket, PacEvent::start(state.request(id)))
                            }
                            EventType::Resolved(result) => {
                                let mut state = state_ref.lock().unwrap();
                                state.resolve(id, result);
                            }
                            _ => {}
                        }
                    }
                }

                // Errors will be primarily triggered by a ConnectionClose error so we will break the loop and drop the thread
                if let Err(_) = next {
                    if id != 0 {
                        state_ref.lock().unwrap().disconnect(id);
                    }
                    return;
                }
            }
        });
    }
}

fn send<T>(socket: &mut WebSocket<T>, msg: PacEvent)
where
    T: std::io::Read + std::io::Write,
{
    socket
        .write_message(serde_json::to_string(&msg).unwrap().into())
        .unwrap();
}
