use chess::{ChessIndex, ChessMove, Color, Game};
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
            crate::print_whites_perspective(self.game().board());
            self.single_turn(Color::White);
            if self.game_over() {
                println!("White wins");
                return Ok(());
            }

            crate::print_blacks_perspective(&self.game().board());
            self.single_turn(Color::Black);
            if self.game_over() {
                println!("Black wins");
                return Ok(());
            }
        }
    }

    fn game_over(&self) -> bool {
        self.game.is_king_checked(Color::Black) || self.game.is_king_checked(Color::White)
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
        from: ChessIndex,
        valid_moves: HashMap<ChessIndex, ChessMove>,
    ) -> ChessMove {
        let piece = self.game().board().piece_at(from).unwrap();

        loop {
            let to_index = input_chess_index(&format!(
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

    fn choose_from_square(&self, player: Color) -> (ChessIndex, HashMap<ChessIndex, ChessMove>) {
        let (from_index, valid_moves) = loop {
            let from_index = input_chess_index(&format!(
                "{} player, what piece to you want to move?",
                player
            ));
            match self.game().board().piece_at(from_index) {
                Some(piece) if piece.color() == player => {
                    let valid_moves = self.game().valid_moves_from(from_index);
                    if valid_moves.is_empty() {
                        println!("your piece at {} has no valid moves", from_index);
                        println!("enter a new index: ");
                    } else {
                        println!("these are the valid moves from {}", from_index);
                        self.print_highlighted(player, &valid_moves);
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

    fn print_highlighted(&self, player: Color, highlighted: &[ChessMove]) {
        match player {
            Color::Black => {
                println!(
                    "{}",
                    chess::fmt::blacks_perspective(
                        &self.game().board(),
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
                        &self.game().board(),
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
