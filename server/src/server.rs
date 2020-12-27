use std::collections::HashMap;

use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

use crate::ChessMessage;

pub struct Server {
    games: HashMap<String, Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub async fn run(self, port: u32) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .unwrap();

        loop {
            // Asynchronously wait for an inbound socket.
            let (mut socket, _) = listener.accept().await.unwrap();

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.

            task::spawn(async move {
                let mut buf = vec![0; 1024];
                loop {
                    let bytes = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");
                    if bytes == 0 {
                        break;
                    }
                }

                if let Ok(content) = String::from_utf8(buf) {
                    let mut trimmed = content.trim();
                    trimmed = &trimmed[..trimmed.len() - 1];
                    println!("'{}'", trimmed);
                    let message: ChessMessage = serde_json::from_str(trimmed).unwrap();
                    match message {
                        ChessMessage::Connect(conn) => {
                            println!("connecting to room: {}", conn.room);
                        }
                    }
                } else {
                    println!("invalid utf8");
                }
            });
        }
    }
}

struct Game {}
