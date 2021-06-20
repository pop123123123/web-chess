use crate::board::Cell;
use crate::board::Move;
use serde::{Deserialize, Serialize};

/// Standard action
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardAction {
    pub from: Cell,
    pub to: Cell,
}

impl Move for StandardAction {
    fn from(&self) -> Cell {
        self.from
    }

    fn to(&self) -> Cell {
        self.to
    }
}