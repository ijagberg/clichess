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
            let (mut socket, _) = listener.accept().await.unwrap();
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
                    let (first, last) = (content.find('{').unwrap(), content.rfind('}').unwrap());
                    let trimmed = &content[first..=last];
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
