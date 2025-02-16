use crate::board::piece::{Piece, PieceColor};
use gtk::Image;
pub fn get_image(piece: &Piece) -> Option<Image> {
    match piece {
        Piece::Pawn(p) => {
            if p.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_plt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_pdt60.png"))
            }
        }
        Piece::Knight(k) => {
            if k.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_nlt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_ndt60.png"))
            }
        }
        Piece::Bishop(b) => {
            if b.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_blt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_bdt60.png"))
            }
        }
        Piece::Rook(r) => {
            if r.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_rlt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_rdt60.png"))
            }
        }
        Piece::Queen(q) => {
            if q.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_qlt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_qdt60.png"))
            }
        }
        Piece::King(k) => {
            if k.color == PieceColor::White {
                Some(Image::from_file("./resources/Chess_klt60.png"))
            } else {
                Some(Image::from_file("./resources/Chess_kdt60.png"))
            }
        }
        Piece::Empty(_) => None,
    }
}
