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
    let mut lines = Vec::new();
    for rank in RankIter::new(Rank::First) {
        let mut pieces = Vec::new();
        for file in FileIter::new(File::A) {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            pieces.push(output);
        }

        lines.push(pieces.join(" "));
    }
    println!("{}", lines.join("\n"));
    println!("---");
}

fn print_blacks_perspective(board: &ChessBoard) {
    println!("---");
    let mut lines = Vec::new();
    for rank in RankIter::new(Rank::First) {
        let mut pieces = Vec::new();
        for file in FileIter::new(File::H).rev() {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            pieces.push(output);
        }

        lines.push(pieces.join(" "));
    }
    println!("{}", lines.join("\n"));
    println!("---");
}
