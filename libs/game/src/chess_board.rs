use crate::{file::FileIter, rank::RankIter, square::Square, ChessIndex, Color, File, Piece, Rank};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct ChessBoard {
    squares: [Square; 64],
}

impl ChessBoard {
    pub fn piece_at(&self, idx: ChessIndex) -> Option<&Piece> {
        self[idx].piece()
    }

    pub fn set_piece(&mut self, idx: ChessIndex, piece: Piece) -> Option<Piece> {
        if let Some(taken_piece) = self[idx].take_piece() {
            self[idx].set_piece(piece);
            Some(taken_piece)
        } else {
            self[idx].set_piece(piece);
            None
        }
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        for rank in RankIter::start_at(Rank::Eighth).rev() {
            let mut pieces = Vec::new();
            for file in FileIter::start_at(File::A) {
                let chess_index = ChessIndex::from((file, rank));
                let output = match self[chess_index].piece() {
                    Some(p) => format!("{}", p),
                    None => " ".to_string(),
                };

                pieces.push(output);
            }

            lines.push(pieces.join(" "));
        }
        write!(f, "{}", lines.join("\n"))
    }
}

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
        let squares = [
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

        let mut s = Self { squares };

        for file in FileIter::start_at(File::A) {
            for rank in RankIter::start_at(Rank::First) {
                let idx = ChessIndex::new(file, rank);
                if let Some(p) = s[idx].piece_mut() {
                    p.add_index_to_history(idx);
                }
            }
        }

        s
    }
}
