#![allow(dead_code)]

mod chess_board;
mod chess_move;
mod file;
mod rank;
mod square;

pub use chess_board::ChessBoard;
pub use chess_move::Move;
pub use file::{File, FileIter};
pub use rank::{Rank, RankIter};
use std::{convert::TryFrom, error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn pawn(color: Color) -> Self {
        Self::new(PieceType::Pawn, color)
    }

    pub fn knight(color: Color) -> Self {
        Self::new(PieceType::Knight, color)
    }

    pub fn bishop(color: Color) -> Self {
        Self::new(PieceType::Bishop, color)
    }

    pub fn rook(color: Color) -> Self {
        Self::new(PieceType::Rook, color)
    }

    pub fn queen(color: Color) -> Self {
        Self::new(PieceType::Queen, color)
    }

    pub fn king(color: Color) -> Self {
        Self::new(PieceType::King, color)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        };

        write!(f, "{}", output)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match (&self.color, &self.piece_type) {
            (Color::Black, PieceType::Pawn) => "♟︎",
            (Color::Black, PieceType::Knight) => "♞",
            (Color::Black, PieceType::Bishop) => "♝",
            (Color::Black, PieceType::Rook) => "♜",
            (Color::Black, PieceType::Queen) => "♛",
            (Color::Black, PieceType::King) => "♚",
            (Color::White, PieceType::Pawn) => "♙",
            (Color::White, PieceType::Knight) => "♘",
            (Color::White, PieceType::Bishop) => "♗",
            (Color::White, PieceType::Rook) => "♖",
            (Color::White, PieceType::Queen) => "♕",
            (Color::White, PieceType::King) => "♔",
        };
        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChessIndex {
    file: File,
    rank: Rank,
}

impl ChessIndex {
    pub fn new(file: File, rank: Rank) -> Self {
        Self { rank, file }
    }

    fn linear_value(&self) -> usize {
        (8 * (i32::from(&self.rank) - 1) + (i32::from(&self.file) - 1)) as usize
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
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
        write!(f, "{}{}", self.file, self.rank)
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