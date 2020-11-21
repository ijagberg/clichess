use crate::{
    consts, file::FileIter, rank::RankIter, square::Square, ChessIndex, Color, Move, Piece,
    PieceType,
};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct ChessBoard {
    white_king: ChessIndex,
    black_king: ChessIndex,
    squares: [Square; 64],
}

impl ChessBoard {
    pub fn is_checked(&self, color: Color) -> bool {
        let king_color = self
            .get_king(color)
            .expect(&format!(
                "no {} king exists",
                color.to_string().to_lowercase()
            ))
            .color();
        let king_index = match color {
            Color::Black => self.black_king,
            Color::White => self.white_king,
        };

        if let Some(_knight_idx) = self.is_checked_by_knight(king_index, king_color) {
            return true;
        }

        if let Some(_pawn_idx) = self.is_checked_by_pawn(king_index, king_color) {
            return true;
        }

        if let Some(_bishop_idx) = self.is_checked_by_bishop(king_index, king_color) {
            return true;
        }

        if let Some(_rook_idx) = self.is_checked_by_rook(king_index, king_color) {
            return true;
        }

        if let Some(_queen_idx) = self.is_checked_by_queen(king_index, king_color) {
            return true;
        }

        false
    }

    fn is_checked_by_bishop(
        &self,
        king_index: ChessIndex,
        king_color: Color,
    ) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();

        // increasing file, increasing rank
        for idx in FileIter::start_at(king_index.file())
            .zip(RankIter::start_at(king_index.rank()))
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_bishop() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // increasing file, decreasing rank
        for idx in FileIter::start_at(king_index.file())
            .zip(RankIter::start_at(king_index.rank()).rev())
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_bishop() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file, increasing rank
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .zip(RankIter::start_at(king_index.rank()))
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_bishop() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file, decreasing rank
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .zip(RankIter::start_at(king_index.rank()).rev())
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_bishop() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        None
    }

    fn is_checked_by_pawn(&self, king_index: ChessIndex, king_color: Color) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();
        match king_color {
            Color::White => {
                // black pawn on king's rank + 1
                let offsets: Vec<(i32, i32)> = vec![(-1, 1), (1, 1)];

                for (file_offset, rank_offset) in offsets {
                    if let Ok(to_index) = ChessIndex::try_from((
                        u8::from(&king_index.file()) as i32 + file_offset,
                        u8::from(&king_index.rank()) as i32 + rank_offset,
                    )) {
                        if self[to_index]
                            .piece()
                            .map(|p| p.is_pawn() && p.color() == opponent_color)
                            .unwrap_or(false)
                        {
                            return Some(to_index);
                        }
                    }
                }
            }
            Color::Black => {
                // white pawn on king's rank - 1
                let offsets = vec![(-1, -1), (1, -1)];

                for (file_offset, rank_offset) in offsets {
                    if let Ok(to_index) = ChessIndex::try_from((
                        u8::from(&king_index.file()) as i32 + file_offset,
                        u8::from(&king_index.rank()) as i32 + rank_offset,
                    )) {
                        if self[to_index]
                            .piece()
                            .map(|p| p.is_pawn() && p.color() == opponent_color)
                            .unwrap_or(false)
                        {
                            return Some(to_index);
                        }
                    }
                }
            }
        }

        None
    }

    fn is_checked_by_knight(
        &self,
        king_index: ChessIndex,
        king_color: Color,
    ) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();
        // check if there is an opponent knight a knight's move away
        let offsets = vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for (file_offset, rank_offset) in offsets {
            if let Ok(to_index) = ChessIndex::try_from((
                u8::from(&king_index.file()) as i32 + file_offset,
                u8::from(&king_index.rank()) as i32 + rank_offset,
            )) {
                if self[to_index]
                    .piece()
                    .map(|p| p.is_knight() && p.color() == opponent_color)
                    .unwrap_or(false)
                {
                    return Some(to_index);
                }
            }
        }

        None
    }

    fn is_checked_by_rook(&self, king_index: ChessIndex, king_color: Color) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();

        // increasing file
        for idx in FileIter::start_at(king_index.file())
            .map(|file| ChessIndex::new(file, king_index.rank()))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_rook() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .map(|file| ChessIndex::new(file, king_index.rank()))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_rook() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // increasing rank
        for idx in RankIter::start_at(king_index.rank())
            .map(|rank| ChessIndex::new(king_index.file(), rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_rook() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing rank
        for idx in RankIter::start_at(king_index.rank())
            .rev()
            .map(|rank| ChessIndex::new(king_index.file(), rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_rook() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        None
    }

    fn is_checked_by_queen(&self, king_index: ChessIndex, king_color: Color) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();

        // increasing file
        for idx in FileIter::start_at(king_index.file())
            .map(|file| ChessIndex::new(file, king_index.rank()))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .map(|file| ChessIndex::new(file, king_index.rank()))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // increasing rank
        for idx in RankIter::start_at(king_index.rank())
            .map(|rank| ChessIndex::new(king_index.file(), rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing rank
        for idx in RankIter::start_at(king_index.rank())
            .rev()
            .map(|rank| ChessIndex::new(king_index.file(), rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // increasing file, increasing rank
        for idx in FileIter::start_at(king_index.file())
            .zip(RankIter::start_at(king_index.rank()))
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // increasing file, decreasing rank
        for idx in FileIter::start_at(king_index.file())
            .zip(RankIter::start_at(king_index.rank()).rev())
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file, increasing rank
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .zip(RankIter::start_at(king_index.rank()))
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        // decreasing file, decreasing rank
        for idx in FileIter::start_at(king_index.file())
            .rev()
            .zip(RankIter::start_at(king_index.rank()).rev())
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            if let Some(p) = self[idx].piece() {
                if p.is_queen() && p.color() == opponent_color {
                    return Some(idx);
                } else {
                    break;
                }
            }
        }

        None
    }

    fn get_king(&self, color: Color) -> Option<&Piece> {
        match color {
            Color::Black => self[self.black_king].piece(),
            Color::White => self[self.white_king].piece(),
        }
    }

    pub fn valid_moves_from(&self, from_index: ChessIndex) -> Vec<Move> {
        let mut clone: ChessBoard = self.clone();

        let piece = match clone[from_index].piece() {
            Some(p) => p,
            None => return Vec::new(),
        };
        let piece_color = piece.color();

        let valid_moves = match piece.piece_type() {
            PieceType::Pawn => unimplemented!(),
            PieceType::Knight => self.valid_knight_moves_from(from_index, piece_color),
            PieceType::Bishop => self.valid_bishop_moves_from(from_index, piece_color),
            PieceType::Rook => self.valid_rook_moves_from(from_index, piece_color),
            PieceType::Queen => self.valid_queen_moves_from(from_index, piece_color),
            PieceType::King => self.valid_king_moves_from(from_index, piece_color),
        };

        let mut actual_valid_moves = Vec::new();
        for valid_move in valid_moves {
            let piece_at_target = clone
                .execute_move(valid_move)
                .expect(&format!("invalid move attempted: '{:?}'", valid_move));
            if clone.is_checked(piece_color) {
                // can't actually make this move
            } else {
                actual_valid_moves.push(valid_move);
            }

            // undo the move
            // first take the piece we moved back
            let piece = clone[valid_move.to_index()].take_piece().unwrap(); // can call unwrap here because we successfully executed the move earlier

            // put the moved piece back to the original square
            clone[from_index].set_piece(piece);

            // if we took some piece by executing the move, put it back
            if let Some(taken_piece) = piece_at_target {
                clone[valid_move.to_index()].set_piece(taken_piece);
            }
        }

        actual_valid_moves
    }

    pub fn execute_move(&mut self, chess_move: Move) -> Result<Option<Piece>, MovePieceError> {
        self.move_piece(chess_move.from_index(), chess_move.to_index())
    }

    pub fn is_move_valid(&self, chess_move: Move) -> bool {
        let valid_moves_from = self.valid_moves_from(chess_move.from_index());
        valid_moves_from.contains(&chess_move)
    }

    fn valid_king_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // increasing file
        if let Some(file) = from_index.file() + 1 {
            let to_index = ChessIndex::new(file, from_index.rank());
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing file
        if let Some(file) = from_index.file() - 1 {
            let to_index = ChessIndex::new(file, from_index.rank());
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing rank
        if let Some(rank) = from_index.rank() + 1 {
            let to_index = ChessIndex::new(from_index.file(), rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing rank
        if let Some(rank) = from_index.rank() - 1 {
            let to_index = ChessIndex::new(from_index.file(), rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing file, increasing rank
        if let (Some(file), Some(rank)) = (from_index.file() + 1, from_index.rank() + 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing file, decreasing rank
        if let (Some(file), Some(rank)) = (from_index.file() + 1, from_index.rank() - 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing file, increasing rank
        if let (Some(file), Some(rank)) = (from_index.file() - 1, from_index.rank() + 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing file, decreasing rank
        if let (Some(file), Some(rank)) = (from_index.file() - 1, from_index.rank() - 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        moves
    }

    fn valid_rook_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        // increasing rank
        for rank in RankIter::start_at(from_index.rank()).skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing rank
        for rank in RankIter::start_at(from_index.rank()).rev().skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing file
        for file in FileIter::start_at(from_index.file()).skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing file
        for file in FileIter::start_at(from_index.file()).rev().skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        moves
    }

    fn valid_knight_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        let offsets = vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for (file_offset, rank_offset) in offsets {
            if let Ok(to_index) = ChessIndex::try_from((
                u8::from(&from_index.file()) as i32 + file_offset,
                u8::from(&from_index.rank()) as i32 + rank_offset,
            )) {
                if self[to_index]
                    .piece()
                    .map(|p| p.color() == piece_color)
                    .unwrap_or(false)
                {
                    continue;
                }
                moves.push(Move::new(from_index, to_index));
            }
        }
        moves
    }

    fn valid_bishop_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        // increasing file, increasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()))
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // increasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // decreasing file, increasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .rev()
            .zip(RankIter::start_at(from_index.rank()))
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // decreasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .rev()
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        moves
    }

    fn valid_queen_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<Move> {
        // increasing rank
        let mut moves = Vec::new();
        for rank in RankIter::start_at(from_index.rank()).skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing rank
        for rank in RankIter::start_at(from_index.rank()).rev().skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing file
        for file in FileIter::start_at(from_index.file()).skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // decreasing file
        for file in FileIter::start_at(from_index.file()).rev().skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index))
                    }
                    break;
                }
                _ => {
                    moves.push(Move::new(from_index, to_index));
                }
            }
        }

        // increasing file, increasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()))
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // increasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // decreasing file, increasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .rev()
            .zip(RankIter::start_at(from_index.rank()))
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        // decreasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .rev()
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) => {
                    if target_piece.color() != piece_color {
                        moves.push(Move::new(from_index, to_index));
                    }
                    break;
                }
                _ => moves.push(Move::new(from_index, to_index)),
            }
        }

        moves
    }

    /// Move a piece from `from` to `to`
    ///
    /// # Returns
    /// A result containing the piece that was taken (if any), or an error describing why the move could not be made
    pub fn move_piece<T>(&mut self, from: T, to: T) -> Result<Option<Piece>, MovePieceError>
    where
        T: Into<ChessIndex>,
    {
        let from: ChessIndex = from.into();
        let to: ChessIndex = to.into();

        // check if there is actually a piece at from
        let from_piece = match self[from].piece() {
            Some(p) => p,
            None => return Err(MovePieceError::NoPieceToMove),
        };

        match self[to].piece() {
            Some(other_piece) if other_piece.color() != from_piece.color() => {
                // there is an opponent piece at the target square
                // replace the other piece
                let mut from_piece = self[from].take_piece().unwrap(); // can call unwrap here because we matched on `piece()` above
                if from_piece.is_king() {
                    match from_piece.color() {
                        Color::Black => {
                            self.black_king = to;
                        }
                        Color::White => {
                            self.white_king = to;
                        }
                    }
                }
                from_piece.add_move_to_history(Move::new(from, to));
                let old_piece = self[to].set_piece(from_piece).unwrap(); // --||--;

                Ok(Some(old_piece))
            }
            Some(_other_piece) => {
                // there is a piece of the same color at the target square
                Err(MovePieceError::OwnPieceAtTarget)
            }
            None => {
                // there is no piece at the target square
                let mut from_piece = self[from].take_piece().unwrap();
                if from_piece.is_king() {
                    match from_piece.color() {
                        Color::Black => {
                            self.black_king = to;
                        }
                        Color::White => {
                            self.white_king = to;
                        }
                    }
                }
                from_piece.add_move_to_history(Move::new(from, to));
                self[to].set_piece(from_piece);
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

        Self {
            squares: board,
            white_king: consts::E1,
            black_king: consts::E8,
        }
    }
}

#[cfg(test)]
mod tests {
    use consts::*;

    use super::*;

    #[test]
    fn test_should_capture() {
        let mut board = ChessBoard::default();

        // move e2 pawn to e6 to prepare test
        board.move_piece(E2, E6).unwrap();

        let black_d7_pawn = board.move_piece(E6, D7).unwrap();

        assert_eq!(Some(Piece::pawn(Color::Black)), black_d7_pawn);
    }

    #[test]
    fn test_rook_moves() {
        let mut board = ChessBoard::default();
        board.move_piece(A2, E4).unwrap();

        let targets: Vec<ChessIndex> = board
            .valid_rook_moves_from(E4, Color::White)
            .iter()
            .map(|m| m.to_index())
            .collect();

        assert_eq!(vec![E5, E6, E7, E3, F4, G4, H4, D4, C4, B4, A4,], targets)
    }

    #[test]
    fn test_knight_moves() {
        let mut board = ChessBoard::default();

        board.move_piece(G1, E4).unwrap();

        let targets: Vec<ChessIndex> = board
            .valid_knight_moves_from(B1, Color::White)
            .iter()
            .map(|m| m.to_index())
            .collect();

        assert_eq!(vec![C3, A3], targets);

        let targets: Vec<ChessIndex> = board
            .valid_knight_moves_from(E4, Color::White)
            .iter()
            .map(|m| m.to_index())
            .collect();

        assert_eq!(vec![G5, G3, C5, C3, F6, D6,], targets);
    }

    #[test]
    fn test_bishop_moves() {
        let mut board = ChessBoard::default();
        board.move_piece(F1, F4).unwrap();

        let targets: Vec<ChessIndex> = board
            .valid_bishop_moves_from(F4, Color::White)
            .iter()
            .map(|m| m.to_index())
            .collect();

        assert_eq!(vec![G5, H6, G3, E5, D6, C7, E3,], targets);
    }

    #[test]
    fn test_queen_moves() {
        let mut board = ChessBoard::default();
        board.move_piece(D1, D4).unwrap();

        assert_eq!(
            board.valid_moves_from(D4),
            vec![
                Move::new(D4, D5),
                Move::new(D4, D6),
                Move::new(D4, D7),
                Move::new(D4, D3),
                Move::new(D4, E4),
                Move::new(D4, F4),
                Move::new(D4, G4),
                Move::new(D4, H4),
                Move::new(D4, C4),
                Move::new(D4, B4),
                Move::new(D4, A4),
                Move::new(D4, E5),
                Move::new(D4, F6),
                Move::new(D4, G7),
                Move::new(D4, E3),
                Move::new(D4, C5),
                Move::new(D4, B6),
                Move::new(D4, A7),
                Move::new(D4, C3),
            ]
        );
    }

    #[test]
    fn test_king_moves() {
        let mut board = ChessBoard::default();

        assert_eq!(board.valid_king_moves_from(E1, Color::White), vec![]);

        board.move_piece(E1, E4).unwrap();
        assert_eq!(
            board.valid_king_moves_from(E4, Color::White),
            vec![
                Move::new(E4, F4),
                Move::new(E4, D4),
                Move::new(E4, E5),
                Move::new(E4, E3),
                Move::new(E4, F5),
                Move::new(E4, F3),
                Move::new(E4, D5),
                Move::new(E4, D3),
            ]
        );

        board.move_piece(F7, F6).unwrap();
        assert_eq!(
            board.valid_moves_from(E4),
            vec![
                Move::new(E4, F4),
                Move::new(E4, D4),
                Move::new(E4, E3),
                Move::new(E4, F5),
                Move::new(E4, F3),
                Move::new(E4, D5),
                Move::new(E4, D3),
            ]
        )
    }

    #[test]
    fn test_is_checked_by_pawn() {
        let mut board = ChessBoard::default();

        board.move_piece(E1, E4).unwrap();
        assert_eq!(board.is_checked_by_pawn(E4, Color::White), None);

        board.move_piece(D7, D5).unwrap();
        assert_eq!(board.is_checked_by_pawn(E4, Color::White), Some(D5));
    }

    #[test]
    fn test_is_checked_by_knight() {
        let mut board = ChessBoard::default();

        board.move_piece(E1, E4).unwrap();
        assert_eq!(board.is_checked_by_knight(E4, Color::White), None);

        board.move_piece(G8, F6).unwrap();
        assert_eq!(board.is_checked_by_knight(E4, Color::White), Some(F6));
    }

    #[test]
    fn test_is_checked_by_bishop() {
        let mut board = ChessBoard::default();

        board.move_piece(E1, E4).unwrap();
        assert_eq!(board.is_checked_by_bishop(E4, Color::White), None);

        board.move_piece(C8, G6).unwrap();
        assert_eq!(board.is_checked_by_bishop(E4, Color::White), Some(G6));

        board.move_piece(F2, F5).unwrap();
        assert_eq!(board.is_checked_by_bishop(E4, Color::White), None);
    }

    #[test]
    fn test_is_checked_by_rook() {
        let mut board = ChessBoard::default();

        board.move_piece(E1, E4).unwrap();
        assert_eq!(board.is_checked_by_rook(E4, Color::White), None);

        board.move_piece(H8, H4).unwrap();
        assert_eq!(board.is_checked_by_rook(E4, Color::White), Some(H4));

        board.move_piece(G2, G4).unwrap();
        assert_eq!(board.is_checked_by_rook(E4, Color::White), None);
    }

    #[test]
    fn test_is_checked_by_queen() {
        let mut board = ChessBoard::default();

        board.move_piece(E1, E4).unwrap();
        assert_eq!(board.is_checked_by_queen(E4, Color::White), None);

        board.move_piece(D8, C6).unwrap();
        assert_eq!(board.is_checked_by_queen(E4, Color::White), Some(C6));

        board.move_piece(C6, B4).unwrap();
        assert_eq!(board.is_checked_by_queen(E4, Color::White), Some(B4));

        board.move_piece(C1, D4).unwrap();
        assert_eq!(board.is_checked_by_queen(E4, Color::White), None);
    }

    #[test]
    fn test_valid_moves_from() {
        let mut board = ChessBoard::default();

        assert_eq!(board.valid_moves_from(A1), vec![]);

        board.move_piece(A1, A3).unwrap();
        assert_eq!(
            board.valid_moves_from(A3),
            vec![
                Move::new(A3, A4),
                Move::new(A3, A5),
                Move::new(A3, A6),
                Move::new(A3, A7),
                Move::new(A3, B3),
                Move::new(A3, C3),
                Move::new(A3, D3),
                Move::new(A3, E3),
                Move::new(A3, F3),
                Move::new(A3, G3),
                Move::new(A3, H3),
            ]
        );

        board.move_piece(E8, A6).unwrap();
        board.move_piece(A8, A5).unwrap();
        assert_eq!(
            board.valid_moves_from(A5),
            vec![Move::new(A5, A4), Move::new(A5, A3)]
        );
    }

    #[test]
    fn test_is_move_valid() {
        let board = ChessBoard::default();

        assert!(board.is_move_valid(Move::new(E2, E4)));
        assert!(board.is_move_valid(Move::new(A1, A3)));
    }

    #[test]
    fn test_move_history() {
        let mut board = ChessBoard::default();

        assert_eq!(board[E2].piece().unwrap().history(), &vec![]);

        board.move_piece(E2, E3).unwrap();
        assert_eq!(
            board[E3].piece().unwrap().history(),
            &vec![Move::new(E2, E3)]
        );

        board.move_piece(E3, E4).unwrap();
        assert_eq!(
            board[E4].piece().unwrap().history(),
            &vec![Move::new(E2, E3), Move::new(E3, E4)]
        );
    }
}
