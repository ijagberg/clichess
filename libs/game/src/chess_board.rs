use crate::{
    consts, file::FileIter, rank::RankIter, square::Square, CastleMove, ChessIndex, ChessMove,
    Color, File, Piece, PieceType, Rank,
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
    previous: Box<Option<ChessBoard>>,
}

impl ChessBoard {
    /// Check if a square is in check
    pub fn is_checked(&self, index: ChessIndex, color: Color) -> bool {
        if let Some(_knight_idx) = self.is_checked_by_knight(index, color) {
            return true;
        }

        if let Some(_pawn_idx) = self.is_checked_by_pawn(index, color) {
            return true;
        }

        if let Some(_bishop_idx) = self.is_checked_by_bishop(index, color) {
            return true;
        }

        if let Some(_rook_idx) = self.is_checked_by_rook(index, color) {
            return true;
        }

        if let Some(_queen_idx) = self.is_checked_by_queen(index, color) {
            return true;
        }

        false
    }

    pub fn is_king_checked(&self, color: Color) -> bool {
        match color {
            Color::Black => self.is_checked(self.black_king, color),
            Color::White => self.is_checked(self.white_king, color),
        }
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

    pub fn valid_moves_from(&self, from_index: ChessIndex) -> Vec<ChessMove> {
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
            clone
                .execute_move(&valid_move)
                .expect("invalid move attempted");
            if clone.is_king_checked(piece_color) {
                // can't actually make this move
            } else {
                actual_valid_moves.push(valid_move);
            }
            clone.undo_last_move();
        }

        actual_valid_moves
    }

    fn undo_last_move(&mut self) {
        if self.previous.is_some() {
            let p = self.previous.take();
            *self = p.unwrap();
        }
    }

    pub fn execute_move(&mut self, chess_move: &ChessMove) -> Result<(), MovePieceError> {
        let prev = self.clone();
        let result = match chess_move {
            ChessMove::Regular(regular_move) => {
                self.move_piece(regular_move.from(), regular_move.to())
            }
            ChessMove::Castle(_) => unimplemented!(),
            ChessMove::Promotion(_) => unimplemented!(),
        };

        self.previous = Box::new(Some(prev));

        result
    }

    fn execute_castle_move(&mut self, castle_move: CastleMove) {
        let mut king = self[castle_move.king_from()]
            .take_piece()
            .expect("must be a king at from index");
        let mut rook = self[castle_move.rook_from()]
            .take_piece()
            .expect("must be a rook at from index");

        king.add_index_to_history(castle_move.king_to());
        rook.add_index_to_history(castle_move.rook_to());

        self[castle_move.king_to()].set_piece(king);
        self[castle_move.rook_to()].set_piece(rook);
    }

    pub fn is_move_valid(&self, chess_move: &ChessMove) -> bool {
        match chess_move {
            ChessMove::Regular(regular) => {
                let valid_moves_from = self.valid_moves_from(regular.from());
                valid_moves_from.contains(&chess_move)
            }
            ChessMove::Castle(_) => unimplemented!(),
            ChessMove::Promotion(_) => unimplemented!(),
        }
    }

    fn can_castle(
        &self,
        king_index: ChessIndex,
        rook_index: ChessIndex,
    ) -> Result<CastleMove, CanCastleError> {
        let (king, rook) = match (self[king_index].piece(), self[rook_index].piece()) {
            (Some(king), Some(rook))
                if king.is_king() && rook.is_rook() && king.color() == rook.color() =>
            {
                (king, rook)
            }
            _ => {
                // either there is no piece at `king_index` or the piece is not a king
                return Err(CanCastleError::WrongPieces);
            }
        };

        let color = king.color();

        if king.has_made_move() {
            return Err(CanCastleError::PieceHasMadeMove(king_index));
        }
        if rook.has_made_move() {
            return Err(CanCastleError::PieceHasMadeMove(rook_index));
        }

        // check that squares between the king and rook are empty and not in check
        let indices_between = ChessIndex::indices_between(king_index, rook_index);
        debug_assert!(
            indices_between.len() == 4 || indices_between.len() == 5,
            format!("{:?}", (indices_between.len(), king_index, rook_index))
        );
        for index_in_between in indices_between {
            if index_in_between != king_index
                && index_in_between != rook_index
                && self[index_in_between].piece().is_some()
            {
                // square between the king and rook is not empty
                return Err(CanCastleError::PiecesBetween);
            }
            if self.is_checked(index_in_between, color) {
                return Err(CanCastleError::SquareInCheck(index_in_between));
            }
        }

        let (king_to, rook_to) = if king_index.file() < rook_index.file() {
            (
                ChessIndex::new((king_index.file() + 2).unwrap(), king_index.rank()),
                ChessIndex::new((king_index.file() + 1).unwrap(), rook_index.rank()),
            )
        } else {
            (
                ChessIndex::new((king_index.file() - 2).unwrap(), king_index.rank()),
                ChessIndex::new((king_index.file() - 1).unwrap(), rook_index.rank()),
            )
        };

        Ok(CastleMove::new(king_index, king_to, rook_index, rook_to))
    }

    fn valid_king_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        // increasing file
        if let Some(file) = from_index.file() + 1 {
            let to_index = ChessIndex::new(file, from_index.rank());
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // decreasing file
        if let Some(file) = from_index.file() - 1 {
            let to_index = ChessIndex::new(file, from_index.rank());
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // increasing rank
        if let Some(rank) = from_index.rank() + 1 {
            let to_index = ChessIndex::new(from_index.file(), rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // decreasing rank
        if let Some(rank) = from_index.rank() - 1 {
            let to_index = ChessIndex::new(from_index.file(), rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // increasing file, increasing rank
        if let (Some(file), Some(rank)) = (from_index.file() + 1, from_index.rank() + 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // increasing file, decreasing rank
        if let (Some(file), Some(rank)) = (from_index.file() + 1, from_index.rank() - 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // decreasing file, increasing rank
        if let (Some(file), Some(rank)) = (from_index.file() - 1, from_index.rank() + 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        // decreasing file, decreasing rank
        if let (Some(file), Some(rank)) = (from_index.file() - 1, from_index.rank() - 1) {
            let to_index = ChessIndex::new(file, rank);
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                }
            }
        }

        moves
    }

    fn valid_rook_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        // increasing rank
        for rank in RankIter::start_at(from_index.rank()).skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // decreasing rank
        for rank in RankIter::start_at(from_index.rank()).rev().skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // increasing file
        for file in FileIter::start_at(from_index.file()).skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // decreasing file
        for file in FileIter::start_at(from_index.file()).rev().skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(p) if p.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        moves
    }

    fn valid_knight_moves_from(
        &self,
        from_index: ChessIndex,
        piece_color: Color,
    ) -> Vec<ChessMove> {
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
                match self[to_index].piece() {
                    Some(p) if p.color() == piece_color => {}
                    e => {
                        moves.push(ChessMove::regular(from_index, to_index, e));
                    }
                }
            }
        }
        moves
    }

    fn valid_bishop_moves_from(
        &self,
        from_index: ChessIndex,
        piece_color: Color,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        // increasing file, increasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()))
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // increasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
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
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // decreasing file, decreasing rank
        for index in FileIter::start_at(from_index.file())
            .rev()
            .zip(RankIter::start_at(from_index.rank()).rev())
            .map(|(file, rank)| ChessIndex::new(file, rank))
            .skip(1)
        {
            match self[index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        moves
    }

    fn valid_queen_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        // increasing rank
        let mut moves = Vec::new();
        for rank in RankIter::start_at(from_index.rank()).skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // decreasing rank
        for rank in RankIter::start_at(from_index.rank()).rev().skip(1) {
            let to_index = ChessIndex::from((from_index.file(), rank));
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // increasing file
        for file in FileIter::start_at(from_index.file()).skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // decreasing file
        for file in FileIter::start_at(from_index.file()).rev().skip(1) {
            let to_index = ChessIndex::from((file, from_index.rank()));
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
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
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        // increasing file, decreasing rank
        for (to_file, to_rank) in FileIter::start_at(from_index.file())
            .zip(RankIter::start_at(from_index.rank()).rev())
            .skip(1)
        {
            let to_index = ChessIndex::new(to_file, to_rank);
            match self[to_index].piece() {
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
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
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
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
                Some(target_piece) if target_piece.color() == piece_color => {
                    break;
                }
                e => {
                    moves.push(ChessMove::regular(from_index, to_index, e));
                    if e.is_some() {
                        break;
                    }
                }
            }
        }

        moves
    }

    fn moves_to_opponents_piece(
        &self,
        start: ChessIndex,
        file_step: i32,
        rank_step: i32,
        color: Color,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        for idx in (0..)
            .map(|n| {
                if let (Some(file), Some(rank)) = (
                    File::try_from(i32::from(&start.file()) + n * file_step).ok(),
                    Rank::try_from(i32::from(&start.rank()) + n * rank_step).ok(),
                ) {
                    let idx = ChessIndex::new(file, rank);
                    Some(idx)
                } else {
                    None
                }
            })
            .take_while(|idx| idx.is_some())
            .skip(1)
        {
            let idx = idx.expect("should always be some because we checked `idx.is_some()` above");
            match self[idx].piece() {
                Some(p) => {
                    if p.color() == color.opponent() {
                        moves.push(ChessMove::regular(start, idx, Some(p)));
                    }
                    break;
                }
                None => {
                    moves.push(ChessMove::regular(start, idx, None));
                }
            }
        }

        moves
    }

    /// Move a piece from `from` to `to`
    fn move_piece<T>(&mut self, from: T, to: T) -> Result<(), MovePieceError>
    where
        T: Into<ChessIndex>,
    {
        let from: ChessIndex = from.into();
        let to: ChessIndex = to.into();

        // check if there is actually a piece at from
        let from_piece = match self[from].piece() {
            Some(p) => p,
            None => return Err(MovePieceError::NoPieceToMove(from)),
        };

        if let Some(piece) = self[to].piece() {
            if piece.color() == from_piece.color() {
                return Err(MovePieceError::OwnPieceAtTarget);
            }
        }

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
        from_piece.add_index_to_history(to);
        self[to].set_piece(from_piece);
        Ok(())
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

#[derive(Debug, PartialEq)]
enum CanCastleError {
    WrongPieces,
    SquareInCheck(ChessIndex),
    PiecesBetween,
    PieceHasMadeMove(ChessIndex),
}

impl Display for CanCastleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            CanCastleError::SquareInCheck(idx) => {
                format!("can't castle because {} is in check", idx)
            }
            CanCastleError::WrongPieces => {
                format!("can't castle because at least one of the given squares was wrong")
            }
            CanCastleError::PiecesBetween => {
                format!("can't castle because there are pieces between the king and rook")
            }
            CanCastleError::PieceHasMadeMove(idx) => format!(
                "can't castle because the piece at {} has already moved",
                idx
            ),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum MovePieceError {
    NoPieceToMove(ChessIndex),
    OwnPieceAtTarget,
}

impl Display for MovePieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            MovePieceError::NoPieceToMove(index) => format!("no piece at {}", index),
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

        let mut s = Self {
            squares,
            white_king: consts::E1,
            black_king: consts::E8,
            previous: Box::new(None),
        };

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

#[cfg(test)]
mod tests {
    use consts::*;

    use super::*;

    #[test]
    fn test_rook_moves() {
        let mut board = ChessBoard::default();
        board.move_piece(A2, E4).unwrap();

        let targets: Vec<ChessIndex> = board
            .valid_rook_moves_from(E4, Color::White)
            .iter()
            .map(|m| match m {
                ChessMove::Regular(reg) => reg.to(),
                _ => unreachable!(),
            })
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
            .map(|m| match m {
                ChessMove::Regular(reg) => reg.to(),
                _ => unreachable!(),
            })
            .collect();

        assert_eq!(vec![C3, A3], targets);

        let targets: Vec<ChessIndex> = board
            .valid_knight_moves_from(E4, Color::White)
            .iter()
            .map(|m| match m {
                ChessMove::Regular(reg) => reg.to(),
                _ => unreachable!(),
            })
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
            .map(|m| match m {
                ChessMove::Regular(reg) => reg.to(),
                _ => unreachable!(),
            })
            .collect();

        assert_eq!(vec![G5, H6, G3, E5, D6, C7, E3,], targets);
    }

    #[test]
    fn test_queen_moves() {
        let mut board = ChessBoard::default();
        board.move_piece(D1, D4).unwrap();

        let actual = board.valid_moves_from(D4);
        let expected = vec![
            ChessMove::regular(D4, D5, board[D5].piece()),
            ChessMove::regular(D4, D6, board[D6].piece()),
            ChessMove::regular(D4, D7, board[D7].piece()),
            ChessMove::regular(D4, D3, board[D3].piece()),
            ChessMove::regular(D4, E4, board[E4].piece()),
            ChessMove::regular(D4, F4, board[F4].piece()),
            ChessMove::regular(D4, G4, board[G4].piece()),
            ChessMove::regular(D4, H4, board[H4].piece()),
            ChessMove::regular(D4, C4, board[C4].piece()),
            ChessMove::regular(D4, B4, board[B4].piece()),
            ChessMove::regular(D4, A4, board[A4].piece()),
            ChessMove::regular(D4, E5, board[E5].piece()),
            ChessMove::regular(D4, F6, board[F6].piece()),
            ChessMove::regular(D4, G7, board[G7].piece()),
            ChessMove::regular(D4, E3, board[E3].piece()),
            ChessMove::regular(D4, C5, board[C5].piece()),
            ChessMove::regular(D4, B6, board[B6].piece()),
            ChessMove::regular(D4, A7, board[A7].piece()),
            ChessMove::regular(D4, C3, board[C3].piece()),
        ];
        for (actual, expected) in actual.into_iter().zip(expected.into_iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_king_moves() {
        let mut board = ChessBoard::default();
        println!("{}\n", board);

        assert_eq!(board.valid_king_moves_from(E1, Color::White), vec![]);

        board.move_piece(E1, E4).unwrap();
        println!("{}\n", board);
        assert_eq!(
            board.valid_king_moves_from(E4, Color::White),
            vec![
                ChessMove::regular(E4, F4, board[F4].piece()),
                ChessMove::regular(E4, D4, board[D4].piece()),
                ChessMove::regular(E4, E5, board[E5].piece()),
                ChessMove::regular(E4, E3, board[E3].piece()),
                ChessMove::regular(E4, F5, board[F5].piece()),
                ChessMove::regular(E4, F3, board[F3].piece()),
                ChessMove::regular(E4, D5, board[D5].piece()),
                ChessMove::regular(E4, D3, board[D3].piece()),
            ]
        );

        board.move_piece(F7, F6).unwrap();
        println!("{}\n", board);
        assert_eq!(
            board.valid_moves_from(E4),
            vec![
                ChessMove::regular(E4, F4, board[F4].piece()),
                ChessMove::regular(E4, D4, board[D4].piece()),
                ChessMove::regular(E4, E3, board[E3].piece()),
                ChessMove::regular(E4, F5, board[F5].piece()),
                ChessMove::regular(E4, F3, board[F3].piece()),
                ChessMove::regular(E4, D5, board[D5].piece()),
                ChessMove::regular(E4, D3, board[D3].piece()),
            ]
        );
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
                ChessMove::regular(A3, A4, board[A4].piece()),
                ChessMove::regular(A3, A5, board[A5].piece()),
                ChessMove::regular(A3, A6, board[A6].piece()),
                ChessMove::regular(A3, A7, board[A7].piece()),
                ChessMove::regular(A3, B3, board[B3].piece()),
                ChessMove::regular(A3, C3, board[C3].piece()),
                ChessMove::regular(A3, D3, board[D3].piece()),
                ChessMove::regular(A3, E3, board[E3].piece()),
                ChessMove::regular(A3, F3, board[F3].piece()),
                ChessMove::regular(A3, G3, board[G3].piece()),
                ChessMove::regular(A3, H3, board[H3].piece()),
            ]
        );

        board.move_piece(E8, A6).unwrap();
        board.move_piece(A8, A5).unwrap();
        assert_eq!(
            board.valid_moves_from(A5),
            vec![
                ChessMove::regular(A5, A4, board[A4].piece()),
                ChessMove::regular(A5, A3, board[A3].piece())
            ]
        );
    }

    #[test]
    fn test_is_move_valid() {
        // let board = ChessBoard::default();

        // assert!(board.is_move_valid(Move::new(E2, E4)));
        // assert!(board.is_move_valid(Move::new(A1, A3)));
    }

    #[test]
    fn test_move_history() {
        let mut board = ChessBoard::default();

        assert_eq!(board[E2].piece().unwrap().history(), &vec![E2]);

        board.move_piece(E2, E3).unwrap();
        assert_eq!(board[E3].piece().unwrap().history(), &vec![E2, E3]);

        board.move_piece(E3, E4).unwrap();
        assert_eq!(board[E4].piece().unwrap().history(), &vec![E2, E3, E4]);
    }

    #[test]
    fn test_can_castle() {
        let mut board = ChessBoard::default();

        assert_eq!(board.can_castle(E1, H1), Err(CanCastleError::PiecesBetween)); // can't castle because there are pieces between
        assert_eq!(board.can_castle(E1, A1), Err(CanCastleError::PiecesBetween)); // can't castle because there are pieces between
        assert_eq!(board.can_castle(E1, F1), Err(CanCastleError::WrongPieces)); // can't castle because F1 is a bishop
        assert_eq!(board.can_castle(D1, H1), Err(CanCastleError::WrongPieces)); // can't castle because D1 is a queen

        // move bishop and knight and pawns out of the way
        board[F1].take_piece();
        board[G1].take_piece();
        board[F2].take_piece();
        board[G2].take_piece();

        // move black rook to check square between white king and white rook
        board.move_piece(H8, G6).unwrap();

        assert_eq!(
            board.can_castle(E1, H1),
            Err(CanCastleError::SquareInCheck(G1))
        ); // can't castle because G1 is in check

        // move black rook away
        board[G6].take_piece();

        assert_eq!(
            board.can_castle(E1, H1),
            Ok(CastleMove::new(E1, G1, H1, F1))
        ); // can castle now

        // move white rook to G1
        board.move_piece(H1, G1).unwrap();

        assert_eq!(
            board.can_castle(E1, G1),
            Err(CanCastleError::PieceHasMadeMove(G1))
        ); // rook has moved now
    }

    #[test]
    fn test_execute_castle_move() {
        let mut board = ChessBoard::default();

        board[F1].clear();
        board[G1].clear();

        let castle_move = board.can_castle(E1, H1).unwrap();

        board.execute_castle_move(castle_move);

        assert_eq!(board[E1].piece(), None);
        assert_eq!(board[H1].piece(), None);

        let rook = board[F1].piece().unwrap();
        assert_eq!(rook.history(), &vec![H1, F1]);

        let king = board[G1].piece().unwrap();
        assert!(king.is_king());
        assert_eq!(king.history(), &vec![E1, G1]);
    }

    #[test]
    fn test_moves_to_opponents_piece() {
        use Color::*;
        let mut board = ChessBoard::default();

        board.move_piece(D1, E5).unwrap();

        // increasing rank
        assert_eq!(
            board.moves_to_opponents_piece(E5, 0, 1, White),
            vec![
                ChessMove::regular(E5, E6, board[E6].piece()),
                ChessMove::regular(E5, E7, board[E7].piece()),
            ]
        );

        // decreasing rank
        assert_eq!(
            board.moves_to_opponents_piece(E5, 0, -1, White),
            vec![
                ChessMove::regular(E5, E4, board[E4].piece()),
                ChessMove::regular(E5, E3, board[E3].piece()),
            ]
        );

        // increasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, 1, 0, White),
            vec![
                ChessMove::regular(E5, F5, board[F5].piece()),
                ChessMove::regular(E5, G5, board[G5].piece()),
                ChessMove::regular(E5, H5, board[H5].piece()),
            ]
        );

        // decreasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, -1, 0, White),
            vec![
                ChessMove::regular(E5, D5, board[D5].piece()),
                ChessMove::regular(E5, C5, board[C5].piece()),
                ChessMove::regular(E5, B5, board[B5].piece()),
                ChessMove::regular(E5, A5, board[A5].piece()),
            ]
        );

        // diagonal
        // increasing rank, increasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, 1, 1, White),
            vec![
                ChessMove::regular(E5, F6, board[F6].piece()),
                ChessMove::regular(E5, G7, board[G7].piece()),
            ]
        );

        // diagonal
        // increasing rank, decreasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, -1, 1, White),
            vec![
                ChessMove::regular(E5, D6, board[D6].piece()),
                ChessMove::regular(E5, C7, board[C7].piece()),
            ]
        );

        // diagonal
        // decreasing rank, increasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, 1, -1, White),
            vec![
                ChessMove::regular(E5, F4, board[F4].piece()),
                ChessMove::regular(E5, G3, board[G3].piece()),
            ]
        );

        // diagonal
        // decreasing rank, decreasing file
        assert_eq!(
            board.moves_to_opponents_piece(E5, -1, -1, White),
            vec![
                ChessMove::regular(E5, D4, board[D4].piece()),
                ChessMove::regular(E5, C3, board[C3].piece()),
            ]
        );
    }
}
