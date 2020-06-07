use crate::{ChessBoard, ChessIndex, Piece};

pub struct Move<'a> {
    piece: Piece,
    from: ChessIndex,
    to: ChessIndex,
    board: &'a ChessBoard,
}

impl<'a> Move<'a> {
    pub fn new(piece: Piece, from: ChessIndex, to: ChessIndex, board: &'a ChessBoard) -> Self {
        Self {
            piece,
            from,
            to,
            board,
        }
    }

    pub fn from_index(&self) -> ChessIndex {
        self.from
    }

    pub fn to_index(&self) -> ChessIndex {
        self.to
    }
}
