pub mod castling;
pub mod en_passant;
pub mod promotion;
pub mod standard;

use crate::board::cell::Cell;
use castling::CastlingAction;
use en_passant::EnPassantAction;
use promotion::PromotionAction;
use serde::{Deserialize, Serialize};
use standard::StandardAction;

/// A piece move is defined by source and destination cells
pub trait Move {
    fn from(&self) -> Cell;
    fn to(&self) -> Cell;
}

/// Generic action
#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Standard(StandardAction),
    EnPassant(EnPassantAction),
    Promotion(PromotionAction),
    Castling(CastlingAction),
}

impl Action {
    pub fn from(&self) -> Cell {
        match self {
            Action::Promotion(action) => action.from(),
            Action::Castling(action) => action.from(),
            Action::Standard(action) => action.from(),
            Action::EnPassant(action) => action.from(),
        }
    }

    pub fn to(&self) -> Cell {
        match self {
            Action::Promotion(action) => action.to(),
            Action::Castling(action) => action.to(),
            Action::Standard(action) => action.to(),
            Action::EnPassant(action) => action.to(),
        }
    }
}

/// Action data sent by the client
#[derive(Serialize, Deserialize)]
pub struct ActionRequest {
    pub from: Cell,
    pub to: Cell,
    pub piece: Option<char>,
}
