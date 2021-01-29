use std::{collections::HashMap, io, str::FromStr};

use chess::{ai::Strategy, ChessIndex, ChessMove, Color, Game};
pub use local::{PlayLocal, PlayLocalMode, PlayLocalOpts};

mod local;

pub trait Player {
    fn get_move(&self, game: &Game) -> ChessMove;
}

pub struct LocalPlayer {}

impl LocalPlayer {
    pub fn new() -> Self {
        Self {}
    }

    fn choose_move(
        game: &Game,
        from: ChessIndex,
        valid_moves: HashMap<ChessIndex, ChessMove>,
    ) -> ChessMove {
        let player = game.current_player();
        let piece = game.board().piece_at(from).unwrap();

        loop {
            let to_index = Self::input_chess_index(&format!(
                "{} player, where do you want to move your {} on {}?",
                player, piece, from
            ));
            if let Some(chosen_move) = valid_moves.get(&to_index) {
                return *chosen_move;
            } else {
                println!("your {} on {} can't move to {}", piece, from, to_index);
            }
        }
    }

    fn choose_from_square(game: &Game) -> (ChessIndex, HashMap<ChessIndex, ChessMove>) {
        let player = game.current_player();
        let (from_index, valid_moves) = loop {
            let from_index = Self::input_chess_index(&format!(
                "{} player, what piece to you want to move?",
                player
            ));
            match game.board().piece_at(from_index) {
                Some(piece) if piece.color() == player => {
                    let valid_moves = game.valid_moves_from(from_index);
                    if valid_moves.is_empty() {
                        println!("your piece at {} has no valid moves", from_index);
                        println!("enter a new index: ");
                    } else {
                        println!("these are the valid moves from {}", from_index);
                        Self::print_highlighted(player, game, &valid_moves);
                    }
                    break (from_index, valid_moves);
                }
                _ => {
                    println!(
                        "the {} square does not contain a piece that you can move",
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
                    ChessMove::Regular(rm) => (rm.to_idx(), m),
                    ChessMove::Castle(cm) => (cm.king_to(), m),
                    ChessMove::Promotion(pm) => (pm.to_idx(), m),
                    ChessMove::EnPassant(epm) => (epm.to_idx(), m),
                })
                .collect(),
        )
    }

    fn input_chess_index(reason: &str) -> ChessIndex {
        let stdin = io::stdin();
        loop {
            println!("{}", reason);
            let input = {
                let mut buffer = String::new();
                stdin.read_line(&mut buffer).unwrap();
                buffer.trim().to_owned()
            };

            match ChessIndex::from_str(&input) {
                Ok(idx) => {
                    return idx;
                }
                Err(err) => {
                    println!("invalid format of index: '{}'", err);
                }
            }
        }
    }
    fn print_highlighted(player: Color, game: &Game, highlighted: &[ChessMove]) {
        match player {
            Color::Black => {
                println!(
                    "{}",
                    chess::fmt::blacks_perspective(
                        game.board(),
                        &highlighted
                            .iter()
                            .map(|vm| match vm {
                                ChessMove::Regular(rm) => rm.to_idx(),
                                ChessMove::Castle(cm) => cm.king_to(),
                                ChessMove::Promotion(pm) => pm.to_idx(),
                                ChessMove::EnPassant(em) => em.to_idx(),
                            })
                            .collect(),
                    )
                );
            }
            Color::White => {
                println!(
                    "{}",
                    chess::fmt::whites_perspective(
                        game.board(),
                        &highlighted
                            .iter()
                            .map(|vm| match vm {
                                ChessMove::Regular(rm) => rm.to_idx(),
                                ChessMove::Castle(cm) => cm.king_to(),
                                ChessMove::Promotion(pm) => pm.to_idx(),
                                ChessMove::EnPassant(em) => em.to_idx(),
                            })
                            .collect(),
                    )
                );
            }
        }
    }
}

impl Player for LocalPlayer {
    fn get_move(&self, game: &Game) -> ChessMove {
        let (from, moves) = Self::choose_from_square(game);

        let chosen_move = Self::choose_move(game, from, moves);

        chosen_move
    }
}

pub struct ComputerPlayer<T> {
    strategy: T,
}

impl<T> ComputerPlayer<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }
}

impl<T> Player for ComputerPlayer<T>
where
    T: Strategy,
{
    fn get_move(&self, game: &Game) -> ChessMove {
        let best_move = self.strategy.get_move(game);
        best_move
    }
}
