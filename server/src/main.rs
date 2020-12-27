mod server;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    port: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectMessage {}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let opts = Opts::from_args();

    let runner = server::Server::new();
    runner.run(opts.port).await;
}
