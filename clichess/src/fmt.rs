use chess::prelude::*;
use std::{collections::HashSet, convert::TryFrom};

pub fn print_whites_perspective(board: &Board) {
    println!("---");
    println!("{}", whites_perspective(board, &HashSet::new()));
    println!("---");
}

pub fn print_blacks_perspective(board: &Board) {
    println!("---");
    println!("{}", blacks_perspective(board, &HashSet::new()));
    println!("---");
}

#[must_use]
pub fn whites_perspective(board: &Board, highlighted_squares: &HashSet<Position>) -> String {
    get_perspective(board, Color::White, highlighted_squares)
}

#[must_use]
pub fn blacks_perspective(board: &Board, highlighted_squares: &HashSet<Position>) -> String {
    get_perspective(board, Color::Black, highlighted_squares)
}

fn get_perspective(board: &Board, color: Color, highlighted_squares: &HashSet<Position>) -> String {
    let mut lines = Vec::new();

    for rank in color_rank(color) {
        let mut pieces = Vec::new();
        for file in color_file(color) {
            let index = Position::new(file, rank);
            let highlight = match highlighted_squares.contains(&index) {
                true => "X",
                false => " ",
            };
            let piece = match board.get_piece(index) {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };
            let output = format!("{}{} ", highlight, piece);

            pieces.push(output);
        }

        let mut line = format!("{}│", rank);
        line.push_str(&pieces.join("│"));
        line.push_str("│\n");

        lines.push(line);
    }

    let mut output = match color {
        Color::Black => String::from("   h   g   f   e   d   c   b   a  \n"),
        Color::White => String::from("   a   b   c   d   e   f   g   h  \n"),
    };
    output.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
    output.push_str(&lines.join(" ├───┼───┼───┼───┼───┼───┼───┼───┤\n"));
    output.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");
    output
}

fn color_rank(color: Color) -> Box<dyn Iterator<Item = Rank>> {
    match color {
        Color::Black => Box::new((1..=8_u32).map(|f| Rank::try_from(f).unwrap())),
        Color::White => Box::new((1..=8_u32).rev().map(|f| Rank::try_from(f).unwrap())),
    }
}

fn color_file(color: Color) -> Box<dyn Iterator<Item = File>> {
    match color {
        Color::Black => Box::new((1..=8_u32).rev().map(|f| File::try_from(f).unwrap())),
        Color::White => Box::new((1..=8_u32).map(|f| File::try_from(f).unwrap())),
    }
}
