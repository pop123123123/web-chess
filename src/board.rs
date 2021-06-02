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

/// Board piece defined by a color and a piece type
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

/// Board in its initial state
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

/// Action defined by source and destination cells
#[derive(Serialize, Deserialize)]
pub struct Action {
    pub from: Cell,
    pub to: Cell,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    /// History of actions
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

    /// Returns current turn color
    pub fn get_turn(&self) -> Color {
        if self.history.len() % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Returns whether a move is valid
    pub fn is_move_valid(&self, planned_action: &Action) -> Result<(), InvalidMove> {
        // rewind game to find original cell
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

        // get piece corresponding to original cell
        let original_piece_result = &INITIAL_BOARD[u8::from(origin_cell.row) as usize]
            [u8::from(origin_cell.column) as usize];

        let original_piece: &BoardPiece;

        match original_piece_result {
            Some(p) => {
                // reject action if incorrect color
                if p.color != self.get_turn() {
                    return Err(InvalidMove::WrongTurn);
                } else {
                    original_piece = p;
                }
            }
            None => {
                // reject action if moving from empty cell
                return Err(InvalidMove::EmptySourceCell);
            }
        }

        // calculate distance from origin
        let row_distance = ((u8::from(planned_action.to.row) as i8)
            - (u8::from(planned_action.from.row) as i8))
            .abs();
        let column_distance = ((u8::from(planned_action.to.column) as i8)
            - (u8::from(planned_action.from.column) as i8))
            .abs();

        // check whether the piece makes a legal move
        match &original_piece.piece {
            Piece::Knight => {
                if !((row_distance == 2 && column_distance == 1)
                    || (row_distance == 1 && column_distance == 2))
                {
                    return Err(InvalidMove::OutOfRange);
                }
            }
            Piece::Bishop => {
                // diagonal moves only
                if row_distance != column_distance {
                    return Err(InvalidMove::OutOfRange);
                }
                // TODO: list cells to check for emptiness
            }
            Piece::Rook => {
                // horizontal or vertical moves only
                if row_distance != 0 && column_distance != 0 {
                    return Err(InvalidMove::OutOfRange);
                }
                // TODO: list cells to check for emptiness
            }
            Piece::King => {
                // move to closest squares only
                if row_distance > 1 || column_distance > 1 {
                    return Err(InvalidMove::OutOfRange);
                }
                // TODO: list cells to check for emptiness
            }
            Piece::Queen => {
                // diagonal, horizontal or vertical moves only
                if row_distance != column_distance && row_distance != 0 && column_distance != 0 {
                    return Err(InvalidMove::OutOfRange);
                }
                // TODO: list cells to check for emptiness
            }
            Piece::Pawn => {
                let from_row = u8::from(planned_action.from.row) as i8;
                let to_row = u8::from(planned_action.to.row) as i8;
                let direction = match original_piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };
                let on_initial_pos = (original_piece.color == Color::White && from_row == 1)
                    || (original_piece.color == Color::Black && from_row == 6);

                if on_initial_pos && column_distance == 0 && to_row == from_row + 2 * direction {
                    // TODO: list cells to check for emptiness
                } else if to_row != from_row + direction || column_distance > 1 {
                    return Err(InvalidMove::OutOfRange);
                }
            }
        }

        // TODO: check cells in way of movement for emptiness

        // accept move
        Ok(())
    }

    /// Process action and add it to action history if it is valid
    pub fn push_move(&mut self, planned_action: Action) -> Result<(), InvalidMove> {
        let res = self.is_move_valid(&planned_action);
        if res.is_ok() {
            self.history.push(planned_action);
        }
        res
    }
}
