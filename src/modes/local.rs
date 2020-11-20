use game::{ChessBoard, ChessIndex, Color};
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PlayLocalOpts {}

pub struct PlayLocal {
    #[allow(unused)]
    opts: PlayLocalOpts,
    board: ChessBoard,
}

impl PlayLocal {
    pub fn new(opts: PlayLocalOpts) -> Self {
        Self {
            opts,
            board: ChessBoard::default(),
        }
    }

    pub fn play(&mut self) -> Result<(), ()> {
        loop {
            crate::print_whites_perspective(&self.board);
            self.single_turn(Color::White);
            if self.game_over() {
                println!("White wins");
                return Ok(());
            }

            crate::print_blacks_perspective(&self.board);
            self.single_turn(Color::Black);
            if self.game_over() {
                println!("Black wins");
                return Ok(());
            }
        }
    }

    fn game_over(&self) -> bool {
        false // todo
    }

    fn single_turn(&mut self, player: Color) {
        let stdin = io::stdin();
        loop {
            println!("{} player: enter your move (from -> to): ", player);
            let input = {
                let mut buffer_string = String::new();
                stdin.read_line(&mut buffer_string).unwrap();
                buffer_string.trim().to_owned()
            };

            let (from, to) = match parse_move(&input) {
                Ok(ok) => ok,
                Err(_) => {
                    continue;
                }
            };

            if self.board[from]
                .piece()
                .map(|p| p.color() != player)
                .unwrap_or(true)
            {
                println!("you don't have a piece on that square");
                continue;
            }

            match self.board.move_piece(from, to) {
                Ok(Some(taken_piece)) => {
                    println!(
                        "{} took piece: {} ({} {})",
                        player.to_string().to_lowercase(),
                        taken_piece,
                        taken_piece.color(),
                        taken_piece.piece_type()
                    );
                    return;
                }
                Ok(None) => {
                    return;
                }
                Err(e) => {
                    format!("error making move: '{}'", e);
                    continue;
                }
            }
        }
    }
}

fn parse_move(input: &str) -> Result<(ChessIndex, ChessIndex), ()> {
    let parts: Vec<_> = input.split_whitespace().collect();

    if parts.len() != 2 {
        println!("input should consist of two parts");
        return Err(());
    }

    let from_idx = parts[0].parse::<ChessIndex>().map_err(|e| {
        println!("error parsing move: '{}'", e);
        ()
    })?;

    let to_idx = parts[1].parse::<ChessIndex>().map_err(|e| {
        println!("error parsing move: '{}'", e);
        ()
    })?;

    Ok((from_idx, to_idx))
}
