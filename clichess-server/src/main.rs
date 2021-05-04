use websocket::{sync::Server, OwnedMessage};

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }
        std::thread::spawn(|| {
            let client = request.use_protocol("rust-websocket").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Text(content) => {
                        println!("content: {}", content);
                        let response = format!("pong: {}", content);
                        sender.send_message(&OwnedMessage::Text(response)).unwrap();
                    }
                    unsupported => println!("unsupported: {:?}", unsupported),
                }
            }
        });
    }
}
