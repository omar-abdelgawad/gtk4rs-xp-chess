use std::io;
use std::io::BufRead;

use xp_chess::board::Board;
use xp_chess::board::Piece;
fn print_board(board: &Board) {
    for row in board.board.iter() {
        for cell in row.iter() {
            let piece_char = Piece::char_from_piece(cell);
            print!("{} ", piece_char);
        }
        println!();
    }
}

fn main() {
    let board = Board::new();
    for _ in 0..5 {
        print_board(&board);
        println!("Enter move (row1 col1 row2 col2): ");
        let line = io::stdin().lock().lines().next().unwrap().unwrap();
        // parse 4 integers (row1, col1, row2, col2) from line
        let mut parts = line.split_whitespace();
        let row1: usize = parts.next().unwrap().parse().unwrap();
        let col1: usize = parts.next().unwrap().parse().unwrap();
        let row2: usize = parts.next().unwrap().parse().unwrap();
        let col2: usize = parts.next().unwrap().parse().unwrap();
        println!(
            "Moving piece from ({}, {}) to ({}, {})",
            row1, col1, row2, col2
        );
        // board.move_piece((row1, col1), (row2, col2)).unwrap();
    }
}
