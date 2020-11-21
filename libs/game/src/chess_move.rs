use crate::{ChessBoard, ChessIndex, Piece};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChessMove<'a> {
    Regular(RegularMove<'a>),
    Castle(CastleMove),
    Promotion(PromotionMove),
}

impl<'a> ChessMove<'a> {
    pub fn regular(from: ChessIndex, to: ChessIndex, piece: Option<&'a Piece>) -> ChessMove {
        ChessMove::Regular(RegularMove::new(from, to, piece))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RegularMove<'a>(ChessIndex, ChessIndex, Option<&'a Piece>);

impl<'a> RegularMove<'a> {
    pub fn new(from: ChessIndex, to: ChessIndex, piece: Option<&'a Piece>) -> Self {
        Self(from, to, piece)
    }

    pub fn from(&self) -> ChessIndex {
        self.0
    }

    pub fn to(&self) -> ChessIndex {
        self.1
    }

    pub fn piece(&self) -> Option<&Piece> {
        self.2
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CastleMove(ChessIndex, ChessIndex, ChessIndex, ChessIndex);

impl CastleMove {
    pub fn new(
        king_from: ChessIndex,
        king_to: ChessIndex,
        rook_from: ChessIndex,
        rook_to: ChessIndex,
    ) -> CastleMove {
        CastleMove(king_from, king_to, rook_from, rook_to)
    }

    pub fn king_from(&self) -> ChessIndex {
        self.0
    }

    pub fn king_to(&self) -> ChessIndex {
        self.1
    }

    pub fn rook_from(&self) -> ChessIndex {
        self.2
    }

    pub fn rook_to(&self) -> ChessIndex {
        self.3
    }

    pub fn validate(&self, board: &ChessBoard) -> Result<(), ()> {
        if board[self.king_from()]
            .piece()
            .map(|p| !p.is_king())
            .unwrap_or(true)
        {
            // from is not a king
            return Err(());
        }

        if board[self.rook_from()]
            .piece()
            .map(|p| !p.is_rook())
            .unwrap_or(true)
        {
            // to is not a rook
            return Err(());
        }

        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PromotionMove();
