use crate::consts::{COLS, ROWS};

use super::Board;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PieceColor {
    White,
    Black,
}
impl PieceColor {
    pub fn opposite(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Pawn {
    pub color: PieceColor,
    pub has_moved: bool,
    pub can_en_passant_col: Option<usize>,
}
impl Pawn {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let direction: i32 = match self.color {
            PieceColor::White => -1,
            PieceColor::Black => 1,
        };
        // forward 1 and 2 move logic
        if !self.has_moved {
            moves.push(((row as i32 + direction * 2) as usize, col));
        }
        let new_row = (row as i32 + direction) as usize;
        for i in [0, 1, -1] {
            let new_col = (col as i32 + i) as usize;
            if new_col >= COLS {
                continue;
            }
            if i == 0 && !board.get_piece(new_row, new_col).is_empty() {
                continue;
            } else if i == 1 || i == -1 {
                let dest_piece = board.get_piece(new_row, new_col);
                if dest_piece.color() != Some(self.color.opposite()) {
                    continue;
                }
            }
            moves.push((new_row, new_col));
        }
        if let Some(col) = self.can_en_passant_col {
            moves.push(((row as i32 + direction) as usize, col));
        }
        println!("pawn moves:{:?}", moves);
        moves
            .iter()
            .filter(|(r, c)| !(r < &0 || r >= &ROWS || c < &0 || c >= &COLS))
            .filter(|(r, c)| {
                let piece = board.get_piece(*r, *c);
                piece.color() != Some(self.color)
            })
            .map(|(r, c)| (*r, *c))
            .collect()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Knight {
    pub color: PieceColor,
}
impl Knight {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let (row, col) = (row as i32, col as i32);
        let mut moves = vec![];
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
        ];
        for (dr, dc) in DIRECTIONS {
            let res_row = row + dr;
            let res_col = col + dc;
            if !(res_row < 0 || res_row >= ROWS as i32 || res_col < 0 || res_col >= COLS as i32) {
                moves.push((res_row as usize, res_col as usize));
            }
        }
        moves.retain(|&(r, c)| {
            let piece = board.get_piece(r, c);
            piece.color() != Some(self.color)
        });
        moves
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Bishop {
    pub color: PieceColor,
}
impl Bishop {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (dr, dc) in DIRECTIONS.iter() {
            let mut res_row = row as i32 + dr;
            let mut res_col = col as i32 + dc;
            while !(res_row < 0 || res_row >= ROWS as i32 || res_col < 0 || res_col >= COLS as i32)
            {
                let dest_piece = board.get_piece(res_row as usize, res_col as usize);
                if dest_piece.color() == Some(self.color) {
                    break;
                }
                moves.push((res_row as usize, res_col as usize));
                if !dest_piece.is_empty() {
                    break;
                }
                res_row += dr;
                res_col += dc;
            }
        }
        moves
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Rook {
    pub color: PieceColor,
    pub has_moved: bool,
}
impl Rook {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DIRECTIONS.iter() {
            let mut res_row = row as i32 + dr;
            let mut res_col = col as i32 + dc;
            while !(res_row < 0 || res_row >= ROWS as i32 || res_col < 0 || res_col >= COLS as i32)
            {
                let dest_piece = board.get_piece(res_row as usize, res_col as usize);
                if dest_piece.color() == Some(self.color) {
                    break;
                }
                moves.push((res_row as usize, res_col as usize));
                if !dest_piece.is_empty() {
                    break;
                }
                res_row += dr;
                res_col += dc;
            }
        }
        moves
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Queen {
    pub color: PieceColor,
}
impl Queen {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for (dr, dc) in DIRECTIONS.iter() {
            let mut res_row = row as i32 + dr;
            let mut res_col = col as i32 + dc;
            while !(res_row < 0 || res_row >= ROWS as i32 || res_col < 0 || res_col >= COLS as i32)
            {
                let dest_piece = board.get_piece(res_row as usize, res_col as usize);
                if dest_piece.color() == Some(self.color) {
                    break;
                }
                moves.push((res_row as usize, res_col as usize));
                if !dest_piece.is_empty() {
                    break;
                }
                res_row += dr;
                res_col += dc;
            }
        }
        moves
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct King {
    pub color: PieceColor,
    pub has_moved: bool,
}
impl King {
    pub fn moves_to_consider(&self, row: usize, col: usize, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (dr, dc) in DIRECTIONS.iter() {
            let res_row = row as i32 + dr;
            let res_col = col as i32 + dc;
            if !(res_row < 0 || res_row >= ROWS as i32 || res_col < 0 || res_col >= COLS as i32) {
                let dest_piece = board.get_piece(res_row as usize, res_col as usize);
                if dest_piece.color() == Some(self.color) {
                    continue;
                }
                moves.push((res_row as usize, res_col as usize));
            }
        }
        // castling is a little bit difficult
        moves
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Empty {}

#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
    Empty(Empty),
}
impl Piece {
    pub fn color(&self) -> Option<PieceColor> {
        match self {
            Piece::Empty(_) => None,
            Piece::Pawn(p) => Some(p.color),
            Piece::Knight(k) => Some(k.color),
            Piece::Bishop(b) => Some(b.color),
            Piece::Rook(r) => Some(r.color),
            Piece::Queen(q) => Some(q.color),
            Piece::King(k) => Some(k.color),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Piece::Empty(_) => true,
            _ => false,
        }
    }
    pub fn become_moved(&mut self) {
        match self {
            Piece::Pawn(p) => p.has_moved = true,
            Piece::Rook(r) => r.has_moved = true,
            Piece::King(k) => k.has_moved = true,
            _ => (),
        }
    }
}
// maybe instead we should have seperate structs but make Piece a trait?
pub fn piece_from_char(c: char) -> Piece {
    let color = if c.is_lowercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };
    match c.to_ascii_lowercase() {
        'p' => Piece::Pawn(Pawn {
            color,
            has_moved: false,
            can_en_passant_col: None,
        }),
        'n' => Piece::Knight(Knight { color }),
        'b' => Piece::Bishop(Bishop { color }),
        'r' => Piece::Rook(Rook {
            color,
            has_moved: false,
        }),
        'q' => Piece::Queen(Queen { color }),
        'k' => Piece::King(King {
            color,
            has_moved: false,
        }),
        '*' => Piece::Empty(Empty {}),
        _ => panic!("Invalid piece char: {}", c),
    }
}
