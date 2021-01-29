mod modes;

use std::collections::HashSet;

use chess::ChessBoard;
use modes::{ComputerPlayer, LocalPlayer, PlayLocalMode, PlayLocalOpts};
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
        Command::PlayLocal(opts) => match opts.mode() {
            PlayLocalMode::VsHuman => {
                let mut game = modes::PlayLocal::new(opts, LocalPlayer::new(), LocalPlayer::new());
                game.play().unwrap();
            }
            PlayLocalMode::VsComputerAsBlack => {
                let mut game = modes::PlayLocal::new(
                    opts,
                    ComputerPlayer::new(chess::ai::Material {}),
                    LocalPlayer::new(),
                );
                game.play().unwrap();
            }
            PlayLocalMode::VsComputerAsWhite => {
                let mut game = modes::PlayLocal::new(
                    opts,
                    LocalPlayer::new(),
                    ComputerPlayer::new(chess::ai::Material {}),
                );
                game.play().unwrap();
            }
        },
    }
}

fn print_whites_perspective(board: &ChessBoard) {
    println!("---");
    println!("{}", chess::fmt::whites_perspective(board, &HashSet::new()));
    println!("---");
}

fn print_blacks_perspective(board: &ChessBoard) {
    println!("---");
    println!("{}", chess::fmt::blacks_perspective(board, &HashSet::new()));
    println!("---");
}
