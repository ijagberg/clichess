mod modes;

use game::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    PlayLocal(PlayLocalOpts),
}

#[derive(Debug, StructOpt)]
pub struct PlayLocalOpts {}

fn main() {
    let opts: Opts = Opts::from_args();

    match opts.command {
        Command::PlayLocal(opts) => {
            let mut game = modes::PlayLocal::new(opts);
            game.play().unwrap();
        }
    }
}

fn print_whites_perspective(board: &ChessBoard) {
    println!("---");
    for rank in (0..8).rev() {
        for file in 0..8 {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            print!("{}", output);
        }

        println!();
    }
    println!("---");
}

fn print_blacks_perspective(board: &ChessBoard) {
    println!("---");
    for rank in 0..8 {
        for file in 0..8 {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            print!("{}", output);
        }

        println!();
    }
    println!("---");
}
