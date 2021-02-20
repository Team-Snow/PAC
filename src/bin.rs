mod both;

use crate::both::*;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

pub fn main() {
    println!("Starting PAC server!");

    // Start server on localhost
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                // Todo: Unexpected connection close event
                let msg = websocket.read_message().unwrap();

                if msg.is_text() {
                    let event: PacEvent = serde_json::from_str(&msg.to_string()).unwrap();
                    println!("{:?}", event);
                    websocket
                        .write_message(serde_json::to_string(&PacEvent::start()).unwrap().into())
                        .unwrap();
                }
            }
        });
    }
}
