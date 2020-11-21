#![allow(dead_code)]

mod chess_board;
mod chess_move;
mod consts;
mod file;
mod rank;
mod square;
mod piece;

pub use chess_board::ChessBoard;
pub use chess_move::Move;
pub use file::{File, FileIter};
pub use rank::{Rank, RankIter};
pub use piece::*;
use std::{convert::TryFrom, error::Error, fmt::Display, str::FromStr};


#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Color::Black => "Black",
            Color::White => "White",
        };

        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct ChessIndex(File, Rank);

impl ChessIndex {
    pub fn new(file: File, rank: Rank) -> Self {
        Self(file, rank)
    }

    fn linear_value(&self) -> usize {
        (8 * (u8::from(&self.rank()) - 1) + (u8::from(&self.file()) - 1)) as usize
    }

    pub fn rank(&self) -> Rank {
        self.1
    }

    pub fn file(&self) -> File {
        self.0
    }
}

impl From<(File, Rank)> for ChessIndex {
    fn from((file, rank): (File, Rank)) -> Self {
        ChessIndex::new(file, rank)
    }
}

impl TryFrom<(i32, i32)> for ChessIndex {
    type Error = ();
    fn try_from((file, rank): (i32, i32)) -> Result<Self, Self::Error> {
        let file = u8::try_from(file).map_err(|_| ())?;
        let rank = u8::try_from(rank).map_err(|_| ())?;
        match (File::try_from(file), Rank::try_from(rank)) {
            (Ok(f), Ok(r)) => Ok(ChessIndex::new(f, r)),
            _ => Err(()),
        }
    }
}

impl FromStr for ChessIndex {
    type Err = ParseChessIndexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseChessIndexError::LengthNot2);
        }

        let file_char = s.as_bytes()[0] as char;
        let file =
            File::try_from(file_char).map_err(|_| ParseChessIndexError::InvalidFile(file_char))?;

        let rank_char = s.as_bytes()[1] as char;
        let rank =
            Rank::try_from(rank_char).map_err(|_| ParseChessIndexError::InvalidRank(rank_char))?;

        Ok(ChessIndex::from((file, rank)))
    }
}

impl Display for ChessIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[derive(Debug)]
pub enum ParseChessIndexError {
    LengthNot2,
    InvalidFile(char),
    InvalidRank(char),
}

impl Display for ParseChessIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseChessIndexError::LengthNot2 => format!("format should be 'xy', x: file, y: rank"),
            ParseChessIndexError::InvalidFile(file) => format!("invalid file: '{}'", file),
            ParseChessIndexError::InvalidRank(rank) => format!("invalid rank: '{}'", rank),
        };

        write!(f, "{}", output)
    }
}

impl Error for ParseChessIndexError {}
