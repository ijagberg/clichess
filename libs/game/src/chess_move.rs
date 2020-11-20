use crate::{ChessIndex, Piece};

pub struct Move {
    piece: Piece,
    from: ChessIndex,
    to: ChessIndex,
}

impl Move {
    pub fn new(piece: Piece, from: ChessIndex, to: ChessIndex) -> Self {
        Self { piece, from, to }
    }

    pub fn from_index(&self) -> ChessIndex {
        self.from
    }

    pub fn to_index(&self) -> ChessIndex {
        self.to
    }
}
