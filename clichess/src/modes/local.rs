use crate::fmt::*;
use chess::prelude::*;
use std::{collections::HashMap, io, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PlayLocalOpts {}

pub struct PlayLocal {
    #[allow(unused)]
    opts: PlayLocalOpts,
    game: Game,
}

impl PlayLocal {
    pub fn new(opts: PlayLocalOpts) -> Self {
        Self {
            opts,
            game: Game::new(),
        }
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn play(&mut self) -> Result<(), ()> {
        loop {
            print_whites_perspective(self.game().board());
            self.single_turn(Color::White);
            if self.game_over() {
                println!("White wins");
                return Ok(());
            }

            print_blacks_perspective(&self.game().board());
            self.single_turn(Color::Black);
            if self.game_over() {
                println!("Black wins");
                return Ok(());
            }
        }
    }

    fn game_over(&self) -> bool {
        self.game.is_over()
    }

    fn single_turn(&mut self, player: Color) {
        loop {
            let (from, moves) = self.choose_from_square(player);

            let chosen_move = self.choose_move(player, from, moves);

            self.game.make_move(chosen_move).unwrap();
            break;
        }
    }

    fn choose_move(
        &self,
        player: Color,
        from: Position,
        valid_moves: HashMap<Position, ChessMove>,
    ) -> ChessMove {
        let piece = self.game().board().get_piece(from).unwrap();

        loop {
            let to_index = input_chess_index(&format!(
                "{} player, where do you want to move your {} on {:?}?",
                player, piece, from
            ));
            if let Some(chosen_move) = valid_moves.get(&to_index) {
                return *chosen_move;
            } else {
                println!("your {} on {:?} can't move to {:?}", piece, from, to_index);
            }
        }
    }

    fn choose_from_square(&self, player: Color) -> (Position, HashMap<Position, ChessMove>) {
        let (from_index, valid_moves) = loop {
            let from_index = input_chess_index(&format!(
                "{} player, what piece to you want to move?",
                player
            ));
            match self.game().board().get_piece(from_index) {
                Some(piece) if piece.color() == player => {
                    let valid_moves = self.game().get_moves_from(from_index);
                    if valid_moves.is_empty() {
                        println!("your piece at {:?} has no valid moves", from_index);
                        println!("enter a new index: ");
                    } else {
                        println!("these are the valid moves from {:?}", from_index);
                        self.print_highlighted(player, &valid_moves);
                    }
                    break (from_index, valid_moves);
                }
                _ => {
                    println!(
                        "the {:?} square does not contain a piece that you can move",
                        from_index
                    );
                    println!("enter a new index: ");
                }
            }
        };

        (
            from_index,
            valid_moves
                .into_iter()
                .map(|m| match m {
                    ChessMove::Regular { to, .. } => (to, m),
                    ChessMove::Castle { king_to, .. } => (king_to, m),
                    ChessMove::Promotion { to, .. } => (to, m),
                    ChessMove::EnPassant { to, .. } => (to, m),
                })
                .collect(),
        )
    }

    fn print_highlighted(&self, player: Color, highlighted: &[ChessMove]) {
        match player {
            Color::Black => {
                println!(
                    "{}",
                    blacks_perspective(
                        &self.game().board(),
                        &highlighted
                            .iter()
                            .map(|vm| *match vm {
                                ChessMove::Regular { to, .. } => to,
                                ChessMove::Castle { king_to, .. } => king_to,
                                ChessMove::Promotion { to, .. } => to,
                                ChessMove::EnPassant { to, .. } => to,
                            })
                            .collect(),
                    )
                );
            }
            Color::White => {
                println!(
                    "{}",
                    whites_perspective(
                        &self.game().board(),
                        &highlighted
                            .iter()
                            .map(|vm| *match vm {
                                ChessMove::Regular { to, .. } => to,
                                ChessMove::Castle { king_to, .. } => king_to,
                                ChessMove::Promotion { to, .. } => to,
                                ChessMove::EnPassant { to, .. } => to,
                            })
                            .collect(),
                    )
                );
            }
        }
    }
}

fn input_chess_index(reason: &str) -> Position {
    let stdin = io::stdin();
    loop {
        println!("{}", reason);
        let input = {
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).unwrap();
            buffer.trim().to_owned()
        };

        match Position::from_str(&input) {
            Ok(idx) => {
                return idx;
            }
            Err(_) => {
                println!("invalid format of index");
            }
        }
    }
}
