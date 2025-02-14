const STARTING_BOARD_STR: [[char; 8]; 8] = [
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['*', '*', '*', '*', '*', '*', '*', '*'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
];

pub struct Board {
    pub board: [[Piece; 8]; 8],
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: STARTING_BOARD_STR.map(|row| row.map(|cell| Piece::piece_from_char(cell))),
        }
    }
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), ()> {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        let piece = self.board[from_row][from_col];
        if piece == Piece::Empty {
            return Err(());
        }
        self.board[to_row][to_col] = piece;
        self.board[from_row][from_col] = Piece::Empty;
        Ok(())
    }
}
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
// maybe instead we should have seperate structs but make Piece a trait?
pub enum Piece {
    King(PieceColor),
    Queen(PieceColor),
    Rook(PieceColor),
    Bishop(PieceColor),
    Knight(PieceColor),
    Pawn(PieceColor),
    Empty,
}
impl Piece {
    fn piece_from_char(c: char) -> Piece {
        match c {
            'K' => Piece::King(PieceColor::Black),
            'Q' => Piece::Queen(PieceColor::Black),
            'R' => Piece::Rook(PieceColor::Black),
            'B' => Piece::Bishop(PieceColor::Black),
            'N' => Piece::Knight(PieceColor::Black),
            'P' => Piece::Pawn(PieceColor::Black),
            'k' => Piece::King(PieceColor::White),
            'q' => Piece::Queen(PieceColor::White),
            'r' => Piece::Rook(PieceColor::White),
            'b' => Piece::Bishop(PieceColor::White),
            'n' => Piece::Knight(PieceColor::White),
            'p' => Piece::Pawn(PieceColor::White),
            '*' => Piece::Empty,
            _ => panic!("Invalid piece character: {}", c),
        }
    }
    pub fn char_from_piece(piece: &Piece) -> char {
        match piece {
            Piece::King(PieceColor::Black) => 'K',
            Piece::Queen(PieceColor::Black) => 'Q',
            Piece::Rook(PieceColor::Black) => 'R',
            Piece::Bishop(PieceColor::Black) => 'B',
            Piece::Knight(PieceColor::Black) => 'N',
            Piece::Pawn(PieceColor::Black) => 'P',
            Piece::King(PieceColor::White) => 'k',
            Piece::Queen(PieceColor::White) => 'q',
            Piece::Rook(PieceColor::White) => 'r',
            Piece::Bishop(PieceColor::White) => 'b',
            Piece::Knight(PieceColor::White) => 'n',
            Piece::Pawn(PieceColor::White) => 'p',
            Piece::Empty => '*',
        }
    }
}
