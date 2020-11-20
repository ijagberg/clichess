mod modes;

use game::*;
use modes::PlayLocalOpts;
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
    for rank in RankIter::start_at(Rank::First) {
        let mut pieces = Vec::new();
        for file in FileIter::start_at(File::A) {
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
    for rank in RankIter::start_at(Rank::First) {
        let mut pieces = Vec::new();
        for file in FileIter::start_at(File::H).rev() {
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
