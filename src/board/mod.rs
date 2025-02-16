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
pub struct Board {
    pub board: [Piece; ROWS * COLS],
    turn_player: PieceColor,
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
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> () {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        self.board
            .swap(from_row * COLS + from_col, to_row * COLS + to_col);
        self.board[from_row * COLS + from_col] = Piece::Empty(Empty {});
        self.turn_player = self.turn_player.opposite();
        self.board[to_row * COLS + to_col].become_moved();
    }
    /// returns a list of 0-indexed legal moves for a piece at a given position
    pub fn get_legal_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let (from_row, from_col) = from;
        let piece = &(self.board[from_row * COLS + from_col]);
        if piece.color() != Some(self.turn_player) {
            return vec![];
        }
        match piece {
            Piece::Empty(e) => vec![],
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
        moves: Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        todo!()
    }
}

// pub enum MoveType {
//     Capture,
//     Move,
//     Castle,
//     EnPassant,
//     Promotion,
// }
