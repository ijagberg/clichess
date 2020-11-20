mod modes;

use std::convert::TryFrom;

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
    for rank in RankIter::new(Rank::First).rev() {
        for file in FileIter::new(File::A) {
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
    for rank in RankIter::new(Rank::First) {
        for file in FileIter::new(File::A).rev() {
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

fn chess_index(s: &str) -> ChessIndex {
    let chars: Vec<_> = s.chars().collect();

    ChessIndex::from((
        File::try_from(chars[0]).unwrap(),
        Rank::try_from(chars[1]).unwrap(),
    ))
}
