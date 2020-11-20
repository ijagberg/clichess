use crate::ChessIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: ChessIndex,
    to: ChessIndex,
}

impl Move {
    pub fn new(from: ChessIndex, to: ChessIndex) -> Self {
        Self { from, to }
    }

    pub fn from_index(&self) -> ChessIndex {
        self.from
    }

    pub fn to_index(&self) -> ChessIndex {
        self.to
    }
}
