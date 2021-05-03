use modes::PlayLocalOpts;
use structopt::StructOpt;

mod fmt;
mod modes;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    PlayLocal(PlayLocalOpts),
}

fn main() {
    let opts: Opts = Opts::from_args();

    match opts.command {
        Command::PlayLocal(opts) => {
            let mut game = modes::PlayLocal::new(opts);
            game.play().unwrap();
        }
    }
}
