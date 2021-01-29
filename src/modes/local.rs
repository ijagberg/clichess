use chess::{Color, Game};
use std::str::FromStr;
use structopt::StructOpt;

use super::Player;

#[derive(Debug, StructOpt)]
pub struct PlayLocalOpts {
    mode: PlayLocalMode,
}

impl PlayLocalOpts {
    pub fn mode(&self) -> PlayLocalMode {
        self.mode
    }
}

#[derive(Debug, StructOpt, Clone, Copy)]
pub enum PlayLocalMode {
    VsHuman,
    VsComputerAsBlack,
    VsComputerAsWhite,
}

impl FromStr for PlayLocalMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vs-human" => Ok(Self::VsHuman),
            "vs-computer-as-black" => Ok(Self::VsComputerAsBlack),
            "vs-computer-as-white" => Ok(Self::VsComputerAsWhite),
            err => Err(format!("invalid mode: '{}'", err)),
        }
    }
}

pub struct PlayLocal<A, B> {
    opts: PlayLocalOpts,
    white_player: A,
    black_player: B,

    game: Game,
}

impl<A, B> PlayLocal<A, B>
where
    A: Player,
    B: Player,
{
    pub fn new(opts: PlayLocalOpts, white_player: A, black_player: B) -> Self {
        Self {
            opts,
            white_player,
            black_player,
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
        false
    }

    fn single_turn(&mut self, player: Color) {
        let chosen_move = match player {
            Color::Black => self.black_player.get_move(self.game()),
            Color::White => self.white_player.get_move(self.game()),
        };

        self.game.make_move(chosen_move).unwrap();
    }
}
