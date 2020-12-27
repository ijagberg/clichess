mod server;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(long)]
    port: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ChessMessage {
    Connect(ConnectMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectMessage {
    room: String,
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let opts = Opts::from_args();

    let runner = server::Server::new();
    runner.run(opts.port).await;
}