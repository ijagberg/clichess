use std::fmt::Display;

use crate::Color;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
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

    pub fn is_pawn(&self) -> bool {
        match self.piece_type() {
            PieceType::Pawn => true,
            _ => false,
        }
    }

    pub fn is_knight(&self) -> bool {
        match self.piece_type() {
            PieceType::Knight => true,
            _ => false,
        }
    }

    pub fn is_rook(&self) -> bool {
        match self.piece_type() {
            PieceType::Rook => true,
            _ => false,
        }
    }

    pub fn is_bishop(&self) -> bool {
        match self.piece_type() {
            PieceType::Bishop => true,
            _ => false,
        }
    }

    pub fn is_queen(&self) -> bool {
        match self.piece_type() {
            PieceType::Queen => true,
            _ => false,
        }
    }

    pub fn is_king(&self) -> bool {
        match self.piece_type() {
            PieceType::King => true,
            _ => false,
        }
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match (&self.color(), &self.piece_type()) {
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

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
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
