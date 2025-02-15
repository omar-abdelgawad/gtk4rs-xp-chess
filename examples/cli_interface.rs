use std::io;
use std::io::BufRead;

use xp_chess::board::piece::{Piece, PieceColor};
use xp_chess::board::Board;
use xp_chess::consts::COLS;
fn char_from_piece(piece: &Piece) -> char {
    let (ch, color) = match piece {
        Piece::Pawn(p) => ('P', p.color),
        Piece::Knight(p) => ('N', p.color),
        Piece::Bishop(p) => ('B', p.color),
        Piece::Rook(p) => ('R', p.color),
        Piece::Queen(p) => ('Q', p.color),
        Piece::King(p) => ('K', p.color),
        Piece::Empty(_) => ('*', PieceColor::White),
    };

    // Convert to lowercase if the piece is black
    if color == PieceColor::White {
        ch.to_ascii_lowercase()
    } else {
        ch
    }
}
fn print_board(board: &Board) {
    for (i, piece) in board.board.iter().enumerate() {
        let piece_char = char_from_piece(piece);
        print!("{} ", piece_char);
        if (i + 1) % COLS == 0 {
            println!();
        }
    }
}
fn get_row_col() -> (usize, usize) {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let row: usize = parts.next().unwrap().parse().unwrap();
    let col: usize = parts.next().unwrap().parse().unwrap();
    (row, col)
}
/// returns 0-indexed row and column for the piece to move and the destination
fn get_rows_cols() -> (usize, usize, usize, usize) {
    println!("Enter piece to move (row1 col1): ");
    let (row1, col1) = get_row_col();
    println!("Enter destination (row2 col2): ");
    let (row2, col2) = get_row_col();
    (row1 - 1, col1 - 1, row2 - 1, col2 - 1)
}
fn main() {
    let mut board = Board::new();
    for _ in 0..5 {
        print_board(&board);
        let (row1, col1, row2, col2) = get_rows_cols();
        let legal_moves = board.get_legal_moves((row1, col1));
        println!("Legal moves:{:?}", legal_moves);
        print_legal_moves(&legal_moves, &board);
        board.move_piece((row1, col1), (row2, col2));
    }
}
/// print the board like print_board but replace the parts attacked by the piece with 'X'
fn print_legal_moves(legal_moves: &Vec<(usize, usize)>, board: &Board) {
    for (i, piece) in board.board.iter().enumerate() {
        let piece_char = if legal_moves.contains(&(i / COLS, i % COLS)) {
            'X'
        } else {
            char_from_piece(piece)
        };
        print!("{} ", piece_char);
        if i % COLS == COLS - 1 {
            println!();
        }
    }
}
