#![allow(dead_code)]

use std::{
    convert::TryFrom,
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

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

#[derive(Copy, Clone, Default)]
pub struct ChessIndex {
    row: usize,
    col: usize,
}

impl ChessIndex {
    pub fn new(col: usize, row: usize) -> Self {
        if col >= 8 {
            panic!("invalid col")
        }
        if row >= 8 {
            panic!("invalid row");
        }
        Self { row, col }
    }

    fn linear_value(&self) -> usize {
        8 * self.row + self.col
    }

    pub fn rank(&self) -> Rank {
        Rank::try_from(self.row as u8).unwrap()
    }

    pub fn file(&self) -> File {
        File::try_from(self.col as u8).unwrap()
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

pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl TryFrom<char> for Rank {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let rank = match value {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(rank)
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let output = match value {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(output)
    }
}

pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl TryFrom<char> for File {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let file = match value {
            'a' | 'A' => File::A,
            'b' | 'B' => File::B,
            'c' | 'C' => File::C,
            'd' | 'D' => File::D,
            'e' | 'E' => File::E,
            'f' | 'F' => File::F,
            'g' | 'G' => File::G,
            'h' | 'H' => File::H,
            _ => return Err(()),
        };

        Ok(file)
    }
}

impl TryFrom<u8> for File {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let output = match value {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => return Err(()),
        };
        Ok(output)
    }
}

impl From<(usize, usize)> for ChessIndex {
    fn from((col, row): (usize, usize)) -> Self {
        Self::new(col, row)
    }
}

impl From<File> for u8 {
    fn from(file: File) -> Self {
        match file {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::First => 0,
            Rank::Second => 1,
            Rank::Third => 2,
            Rank::Fourth => 3,
            Rank::Fifth => 4,
            Rank::Sixth => 5,
            Rank::Seventh => 6,
            Rank::Eighth => 7,
        }
    }
}

impl From<(File, Rank)> for ChessIndex {
    fn from((file, rank): (File, Rank)) -> Self {
        Self::new(u8::from(file) as usize, u8::from(rank) as usize)
    }
}

pub struct Square {
    color: Color,
    piece: Option<Piece>,
}

impl Square {
    pub fn piece(&self) -> Option<&Piece> {
        match &self.piece {
            Some(p) => Some(p),
            None => None,
        }
    }

    pub fn empty(color: Color) -> Self {
        Self::new(color, None)
    }

    pub fn occupied(color: Color, piece: Piece) -> Self {
        Self::new(color, Some(piece))
    }

    pub fn new(color: Color, piece: Option<Piece>) -> Self {
        Self { color, piece }
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }
}

pub struct ChessBoard {
    squares: [Square; 64],
}

impl ChessBoard {
    pub fn iter(&self) -> ChessBoardIter {
        ChessBoardIter::new(&self)
    }

    pub fn move_piece<T>(&mut self, from: T, to: T) -> Result<Option<Piece>, MovePieceError>
    where
        T: Into<ChessIndex>,
    {
        let from: ChessIndex = from.into();
        let to: ChessIndex = to.into();

        // check if there is actually a piece at from
        let from_piece = match self[from].piece {
            Some(p) => p,
            None => return Err(MovePieceError::NoPieceToMove),
        };

        let to_square = &mut self[to];
        match to_square.piece {
            Some(other_piece) => {
                if from_piece.color != other_piece.color {
                    // replace the other piece
                    to_square.set_piece(from_piece);

                    self[from].piece = None;
                    Ok(Some(other_piece))
                } else {
                    Err(MovePieceError::OwnPieceAtTarget)
                }
            }
            None => {
                to_square.set_piece(from_piece);
                self[from].piece = None;

                Ok(None)
            }
        }
    }
}

#[derive(Debug)]
pub enum MovePieceError {
    NoPieceToMove,
    OwnPieceAtTarget,
}

impl Display for MovePieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            MovePieceError::NoPieceToMove => format!("no piece at specified from coordinate"),
            MovePieceError::OwnPieceAtTarget => format!("can't move to a square you occupy"),
        };

        write!(f, "{}", output)
    }
}

impl Error for MovePieceError {}

impl Index<ChessIndex> for ChessBoard {
    type Output = Square;
    fn index(&self, index: ChessIndex) -> &Self::Output {
        &self.squares[index.linear_value()]
    }
}

impl IndexMut<ChessIndex> for ChessBoard {
    fn index_mut(&mut self, index: ChessIndex) -> &mut Self::Output {
        &mut self.squares[index.linear_value()]
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        let board = [
            // rank 1
            Square::occupied(Color::Black, Piece::rook(Color::White)), // a1
            Square::occupied(Color::White, Piece::knight(Color::White)), // b1
            Square::occupied(Color::Black, Piece::bishop(Color::White)), // c1
            Square::occupied(Color::White, Piece::queen(Color::White)), // d1
            Square::occupied(Color::Black, Piece::king(Color::White)), // e1
            Square::occupied(Color::White, Piece::bishop(Color::White)), // f1
            Square::occupied(Color::Black, Piece::knight(Color::White)), // g1
            Square::occupied(Color::White, Piece::rook(Color::White)), // h1
            // rank 2
            Square::occupied(Color::White, Piece::pawn(Color::White)), // a2
            Square::occupied(Color::Black, Piece::pawn(Color::White)), // b2
            Square::occupied(Color::White, Piece::pawn(Color::White)), // c2
            Square::occupied(Color::Black, Piece::pawn(Color::White)), // d2
            Square::occupied(Color::White, Piece::pawn(Color::White)), // e2
            Square::occupied(Color::Black, Piece::pawn(Color::White)), // f2
            Square::occupied(Color::White, Piece::pawn(Color::White)), // g2
            Square::occupied(Color::Black, Piece::pawn(Color::White)), // h2
            // rank 3
            Square::empty(Color::Black), // a3
            Square::empty(Color::White), // b3
            Square::empty(Color::Black), // c3
            Square::empty(Color::White), // d3
            Square::empty(Color::Black), // e3
            Square::empty(Color::White), // f3
            Square::empty(Color::Black), // g3
            Square::empty(Color::White), // h3
            // rank 4
            Square::empty(Color::White), // a4
            Square::empty(Color::Black), // b4
            Square::empty(Color::White), // c4
            Square::empty(Color::Black), // d4
            Square::empty(Color::White), // e4
            Square::empty(Color::Black), // f4
            Square::empty(Color::White), // g4
            Square::empty(Color::Black), // h4
            // rank 5
            Square::empty(Color::Black), // a5
            Square::empty(Color::White), // b5
            Square::empty(Color::Black), // c5
            Square::empty(Color::White), // d5
            Square::empty(Color::Black), // e5
            Square::empty(Color::White), // f5
            Square::empty(Color::Black), // g5
            Square::empty(Color::White), // h5
            // rank 6
            Square::empty(Color::White), // a6
            Square::empty(Color::Black), // b6
            Square::empty(Color::White), // c6
            Square::empty(Color::Black), // d6
            Square::empty(Color::White), // e6
            Square::empty(Color::Black), // f6
            Square::empty(Color::White), // g6
            Square::empty(Color::Black), // h6
            // rank 7
            Square::occupied(Color::Black, Piece::pawn(Color::Black)), // a7
            Square::occupied(Color::White, Piece::pawn(Color::Black)), // b7
            Square::occupied(Color::Black, Piece::pawn(Color::Black)), // c7
            Square::occupied(Color::White, Piece::pawn(Color::Black)), // d7
            Square::occupied(Color::Black, Piece::pawn(Color::Black)), // e7
            Square::occupied(Color::White, Piece::pawn(Color::Black)), // f7
            Square::occupied(Color::Black, Piece::pawn(Color::Black)), // g7
            Square::occupied(Color::White, Piece::pawn(Color::Black)), // h7
            // rank 8
            Square::occupied(Color::White, Piece::rook(Color::Black)), // a8
            Square::occupied(Color::Black, Piece::knight(Color::Black)), // b8
            Square::occupied(Color::White, Piece::bishop(Color::Black)), // c8
            Square::occupied(Color::Black, Piece::queen(Color::Black)), // d8
            Square::occupied(Color::White, Piece::king(Color::Black)), // e8
            Square::occupied(Color::Black, Piece::bishop(Color::Black)), // f8
            Square::occupied(Color::White, Piece::knight(Color::Black)), // g8
            Square::occupied(Color::Black, Piece::rook(Color::Black)), // h8
        ];
        Self { squares: board }
    }
}

pub struct ChessBoardIter<'a> {
    index: ChessIndex,
    board: &'a ChessBoard,
}

impl<'a> ChessBoardIter<'a> {
    fn new(board: &'a ChessBoard) -> Self {
        let index = ChessIndex::default();
        Self { index, board }
    }
}

impl<'a> Iterator for ChessBoardIter<'a> {
    type Item = &'a Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index.row == 7 && self.index.col == 7 {
            // reached end of board
            None
        } else {
            let current = self.index; // save current index to return later
            self.index.col += 1; // move 1 step to the right

            if self.index.col == 8 {
                // moved too far, reset column
                self.index.col = 0;
                self.index.row += 1;
            }

            Some(&self.board[current])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_capture() {
        let mut board = ChessBoard::default();

        // move e2 pawn to e6 to prepare test
        board
            .move_piece((File::E, Rank::Second), (File::E, Rank::Sixth))
            .unwrap();

        let black_d7_pawn = board
            .move_piece((File::E, Rank::Sixth), (File::D, Rank::Seventh))
            .unwrap();

        assert_eq!(Some(Piece::pawn(Color::Black)), black_d7_pawn);
    }
}
