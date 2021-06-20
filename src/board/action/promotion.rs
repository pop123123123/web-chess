use crate::board::piece::PromotePiece;
use crate::board::Cell;
use crate::board::Color;
use crate::board::Move;
use crate::board::Piece;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Straight,
    Left,
    Right,
}

impl From<Direction> for i8 {
    fn from(direction: Direction) -> i8 {
        match direction {
            Direction::Straight => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl From<i8> for Direction {
    fn from(value: i8) -> Direction {
        match value {
            0 => Direction::Straight,
            i8::MIN..=-1_i8 => Direction::Left,
            1_i8..=i8::MAX => Direction::Right,
        }
    }
}

/// Promotion action
#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionAction {
    pub start_column: u8,
    pub color: Color,
    pub direction: Direction,
    pub promote_piece: PromotePiece,
}

impl PromotionAction {
    pub fn new(
        start_column: u8,
        color: Color,
        direction: Direction,
        promote_piece: Piece,
    ) -> PromotionAction {
        let end_column: i8 = start_column as i8 + i8::from(direction);
        debug_assert!(
            (0..=7).contains(&end_column),
            "move leads out of bounds (column {})",
            end_column
        );
        PromotionAction {
            start_column,
            color,
            direction,
            promote_piece: PromotePiece::from(promote_piece),
        }
    }
}

impl Move for PromotionAction {
    fn from(&self) -> Cell {
        Cell::new(
            match self.color {
                Color::Black => 1,
                Color::White => 6,
            },
            self.start_column,
        )
    }

    fn to(&self) -> Cell {
        Cell::new(
            match self.color {
                Color::Black => 0,
                Color::White => 7,
            },
            self.start_column + self.direction as u8,
        )
    }
}
