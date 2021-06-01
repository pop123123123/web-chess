use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub enum Error {
    CellWrongArguments,
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Serialize, Deserialize, PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct Cell {
    #[packed_field(bits = "0..=2")]
    pub row: Integer<u8, packed_bits::Bits3>,
    #[packed_field(bits = "3..=6")]
    pub column: Integer<u8, packed_bits::Bits3>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Cell {
    pub fn new(row: u8, column: u8) -> Cell {
        debug_assert!(row < 8, "row {} is out of bounds", row);
        debug_assert!(column < 8, "column {} is out of bounds", column);
        Cell {
            row: row.into(),
            column: column.into(),
        }
    }
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct BoardPiece {
    pub color: Color,
    pub piece: Piece,
}

pub enum InvalidMove {
    WrongTurn,
    EmptySourceCell,
    OutOfBounds,
    OutOfRange,
    OutOfSight,
    ProvokeCheck,
    FriendlyFire,
}

const INITIAL_BOARD: [[Option<BoardPiece>; 8]; 8] = [
    [
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Rook,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Knight,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Bishop,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Queen,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::King,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Bishop,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Knight,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Rook,
        }),
    ],
    [
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Pawn,
        }),
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Pawn,
        }),
    ],
    [
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Rook,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Knight,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Bishop,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Queen,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::King,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Bishop,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Knight,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Rook,
        }),
    ],
];

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub from: Cell,
    pub to: Cell,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    history: Vec<Action>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            history: Vec::new(),
        }
    }
    pub fn get_turn(&self) -> Color {
        if self.history.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }
    pub fn is_move_valid(&self, planned_action: &Action) -> Result<(), InvalidMove> {
        let origin_cell = self
            .history
            .iter()
            .rev()
            .fold(planned_action.from, |cell, action| {
                if action.to == cell {
                    action.from
                } else {
                    cell
                }
            });

        let original_piece = &INITIAL_BOARD[u8::from(origin_cell.row) as usize]
            [u8::from(origin_cell.column) as usize];

        match original_piece {
            Some(p) => {
                if p.color == self.get_turn() {
                    Ok(())
                } else {
                    Err(InvalidMove::WrongTurn)
                }
            }
            None => Err(InvalidMove::EmptySourceCell),
        }
    }
    pub fn push_move(&mut self, planned_action: Action) -> Result<(), InvalidMove> {
        let res = self.is_move_valid(&planned_action);
        if res.is_ok() {
            self.history.push(planned_action);
        }
        res
    }
}
