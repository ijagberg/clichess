use crate::{ChessBoard, ChessIndex, Piece, PieceType};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChessMove<'a> {
    Regular(RegularMove<'a>),
    Castle(CastleMove),
    Promotion(PromotionMove<'a>),
}

impl<'a> ChessMove<'a> {
    pub fn regular(from: ChessIndex, to: ChessIndex, piece: Option<&'a Piece>) -> ChessMove {
        ChessMove::Regular(RegularMove::new(from, to, piece))
    }

    pub fn promotions(
        from: ChessIndex,
        to: ChessIndex,
        pawn: &'a Piece,
        taken_piece: Option<&'a Piece>,
    ) -> Vec<ChessMove<'a>> {
        vec![
            PieceType::Knight,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::Bishop,
        ]
        .into_iter()
        .map(|pt| ChessMove::promotion(from, to, pawn, Piece::new(pt, pawn.color()), taken_piece))
        .collect()
    }

    pub fn promotion(
        from: ChessIndex,
        to: ChessIndex,
        pawn: &'a Piece,
        promotion_piece: Piece,
        taken_piece: Option<&'a Piece>,
    ) -> ChessMove<'a> {
        ChessMove::Promotion(PromotionMove::new(
            from,
            to,
            pawn,
            promotion_piece,
            taken_piece,
        ))
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
pub struct PromotionMove<'a>(ChessIndex, ChessIndex, &'a Piece, Piece, Option<&'a Piece>);

impl<'a> PromotionMove<'a> {
    pub fn new(
        from: ChessIndex,
        to: ChessIndex,
        pawn: &'a Piece,
        promotion_piece: Piece,
        taken_piece: Option<&'a Piece>,
    ) -> Self {
        Self(from, to, pawn, promotion_piece, taken_piece)
    }

    pub fn from(&self) -> ChessIndex {
        self.0
    }

    pub fn to(&self) -> ChessIndex {
        self.1
    }

    pub fn pawn(&self) -> &Piece {
        &self.2
    }

    pub fn promotion_piece(&self) -> &Piece {
        &self.3
    }

    pub fn taken_piece(&self) -> Option<&Piece> {
        self.4
    }
}
