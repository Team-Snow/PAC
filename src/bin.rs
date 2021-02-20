mod both;

use crate::both::*;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

pub fn main() {
    println!("Starting PAC server!");

    // Start server on localhost
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Loop over incoming connections
    for stream in server.incoming() {
        // Spawn a thread for each connection
        // Todo: Connections should be moved to a async thread pool instead of using system threads
        spawn(move || {
            // Accept connection and start reading incoming messages
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let next = websocket.read_message();
                if let (Ok(ref msg)) = next {
                    if msg.is_text() {
                        // Deserialize message into a PacEvent
                        let event: PacEvent = serde_json::from_str(&msg.to_string()).unwrap();

                        websocket
                            .write_message(
                                serde_json::to_string(&PacEvent::start()).unwrap().into(),
                            )
                            .unwrap();
                    }
                }

                // Errors will be primarily triggered by a ConnectionClose error so we will break the loop and join the thread
                if let (Err(e)) = next {
                    break;
                }
            }
        });
    }
}
