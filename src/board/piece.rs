use serde::{Deserialize, Serialize};

/// Piece color
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Piece type
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Board piece defined by a color and a piece type
#[derive(Copy, Clone)]
pub struct BoardPiece {
    pub color: Color,
    pub piece: Piece,
}

/// Piece type for pawn promotion
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PromotePiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl From<Piece> for PromotePiece {
    fn from(piece: Piece) -> PromotePiece {
        debug_assert!(
            piece != Piece::Pawn,
            "it is not possible to promote pawn to pawn"
        );
        debug_assert!(
            piece != Piece::King,
            "it is not possible to promote pawn to king"
        );

        match piece {
            Piece::Knight => PromotePiece::Knight,
            Piece::Bishop => PromotePiece::Bishop,
            Piece::Rook => PromotePiece::Rook,
            Piece::Queen => PromotePiece::Queen,
            _ => PromotePiece::Queen,
        }
    }
}

impl From<PromotePiece> for Piece {
    fn from(piece: PromotePiece) -> Piece {
        match piece {
            PromotePiece::Knight => Piece::Knight,
            PromotePiece::Bishop => Piece::Bishop,
            PromotePiece::Rook => Piece::Rook,
            PromotePiece::Queen => Piece::Queen,
        }
    }
}
