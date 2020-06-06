use game::*;

fn main() {
    let board = ChessBoard::default();

    println!("White's perspective:");
    print_whites_perspective(&board);
    println!("---");
    println!("Black's perspective:");
    print_blacks_perspective(&board);
}

fn print_whites_perspective(board: &ChessBoard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            print!("{}", output);
        }

        println!();
    }
}

fn print_blacks_perspective(board: &ChessBoard) {
    for rank in 0..8 {
        for file in 0..8 {
            let chess_index = ChessIndex::from((file, rank));
            let output = match board[chess_index].piece() {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };

            print!("{}", output);
        }

        println!();
    }
}
