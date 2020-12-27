use std::collections::HashMap;

use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

pub struct Server {
    games: HashMap<String, Game>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub async fn run(self) {
        let listener = TcpListener::bind("0.0.0.0").await.unwrap();

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
                    println!("content: '{}'", content);
                } else {
                    println!("invalid utf8");
                }
            });
        }
    }
}

struct Game {}
