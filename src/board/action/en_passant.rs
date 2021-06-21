use super::standard::StandardAction;
use super::Move;
use crate::board::cell::Cell;

/// En passant action
pub type EnPassantAction = StandardAction;

impl EnPassantAction {
    pub fn target_cell(&self) -> Cell {
        Cell::new(self.from().row() as u8, self.to().col() as u8)
    }
}
