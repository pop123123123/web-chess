use crate::board::action::Move;
use crate::board::cell::Cell;
use crate::board::piece::Color;
use serde::{Deserialize, Serialize};

/// Castling side (kingside or queenside)
#[derive(Debug, Serialize, Deserialize)]
pub enum CastlingSide {
    King,
    Queen,
}

/// Castling action
#[derive(Debug, Serialize, Deserialize)]
pub struct CastlingAction {
    pub side: CastlingSide,
    pub color: Color,
}

impl CastlingAction {
    pub fn tower_from(&self) -> Cell {
        let row = match self.color {
            Color::White => 0,
            Color::Black => 7,
        };
        let column = match self.side {
            CastlingSide::King => 7,
            CastlingSide::Queen => 0,
        };
        Cell::new(row, column)
    }

    pub fn tower_to(&self) -> Cell {
        let row = match self.color {
            Color::White => 0,
            Color::Black => 7,
        };
        let column = match self.side {
            CastlingSide::King => 5,
            CastlingSide::Queen => 3,
        };
        Cell::new(row, column)
    }
}

impl Move for CastlingAction {
    fn from(&self) -> Cell {
        let row = match self.color {
            Color::White => 0,
            Color::Black => 7,
        };
        Cell::new(row, 4)
    }

    fn to(&self) -> Cell {
        let row = match self.color {
            Color::White => 0,
            Color::Black => 7,
        };
        let column = match self.side {
            CastlingSide::King => 6,
            CastlingSide::Queen => 2,
        };
        Cell::new(row, column)
    }
}
