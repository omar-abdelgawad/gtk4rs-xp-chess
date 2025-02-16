pub mod piece;
use crate::consts::{COLS, ROWS};
use piece::{piece_from_char, Empty, Piece, PieceColor};
const STARTING_BOARD_STR: [[char; COLS]; ROWS] = [
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
];
/// Represents a chess board with game state.
#[derive(Clone, PartialEq)]
pub struct Board {
    board: [Piece; ROWS * COLS],
    pub turn_player: PieceColor,
}
impl Board {
    pub fn new() -> Board {
        let board = STARTING_BOARD_STR
            .iter()
            .flatten()
            .map(|&c| piece_from_char(c))
            .collect::<Vec<Piece>>()
            .try_into()
            .unwrap();
        Board {
            board: board,
            turn_player: PieceColor::White,
        }
    }
    pub fn get_piece(&self, row: usize, col: usize) -> &Piece {
        &self.board[row * COLS + col]
    }
    pub fn set_piece(&mut self, row: usize, col: usize, piece: Piece) {
        self.board[row * COLS + col] = piece;
    }
    pub fn try_move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), ()> {
        let legal_moves = self.get_legal_moves(from);
        if legal_moves.contains(&to) {
            self.move_piece(from, to);
            // self.in_check = self.is_check(self.turn_player);
            return Ok(());
        }
        Err(())
    }
    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> () {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        self.board
            .swap(from_row * COLS + from_col, to_row * COLS + to_col);
        self.set_piece(from_row, from_col, Piece::Empty(Empty {}));
        self.turn_player = self.turn_player.opposite();
        self.board[to_row * COLS + to_col].become_moved();
    }
    /// returns a list of 0-indexed legal moves for a piece at a given position
    pub fn get_legal_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = self.get_valid_moves_to_consider(from);
        self.filter_legal_moves_check(from, &mut moves)
    }
    fn get_valid_moves_to_consider(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let (from_row, from_col) = from;
        let piece = self.get_piece(from_row, from_col);
        if piece.color() != Some(self.turn_player) {
            return vec![];
        }
        match piece {
            Piece::Empty(_) => vec![],
            Piece::Pawn(p) => p.moves_to_consider(from_row, from_col, self),
            Piece::Knight(k) => k.moves_to_consider(from_row, from_col, self),
            Piece::Bishop(b) => b.moves_to_consider(from_row, from_col, self),
            Piece::Rook(r) => r.moves_to_consider(from_row, from_col, self),
            Piece::Queen(q) => q.moves_to_consider(from_row, from_col, self),
            Piece::King(k) => {
                // can't move to a square with a friendly piece
                // can't castle if it has moved
                // can't castle if the rook has moved
                // can't castle if there are pieces between the king and rook
                // can't castle if the king is in check
                // can't castle if the king moves through check
                k.moves_to_consider(from_row, from_col, self)
            }
        }
    }
    fn filter_legal_moves_check(
        &self,
        from: (usize, usize),
        moves: &mut Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let res = moves.iter().filter(|&to| {
            let mut board = self.clone();
            let cur_player = board.turn_player;
            board.move_piece(from, *to);
            !board.king_still_in_check(cur_player)
        });
        println!("Filtered moves: {:?}", moves);
        res.cloned().collect()
    }
    fn king_still_in_check(&self, color: PieceColor) -> bool {
        let king_position = self.get_king_position(color);
        let attacking_color = color.opposite();
        for row in 0..ROWS {
            for col in 0..COLS {
                let piece = self.get_piece(row, col);
                if piece.color() == Some(attacking_color) {
                    let legal_moves = self.get_valid_moves_to_consider((row, col));
                    if legal_moves.contains(&king_position) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn get_king_position(&self, color: PieceColor) -> (usize, usize) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if let Piece::King(k) = self.get_piece(row, col) {
                    if k.color == color {
                        println!("King found at ({}, {})", row, col);
                        return (row, col);
                    }
                }
            }
        }
        panic!("King not found");
    }
}

// pub enum MoveType {
//     Capture,
//     Move,
//     Castle,
//     EnPassant,
//     Promotion,
// }
