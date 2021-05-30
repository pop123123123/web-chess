use std::vec::Vec;

pub enum Error {
    CellWrongArguments,
}

pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub row: u8,
    pub column: u8,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Cell {
    pub fn new(row: u8, column: u8) -> Result<Cell, Error> {
        if row > 7 || column > 7 {
            Err(Error::CellWrongArguments)
        } else {
            Ok(Cell { row, column })
        }
    }
}

pub enum Piece {
    Poon,
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
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::White,
            piece: Piece::Poon,
        }),
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
        }),
        Some(BoardPiece {
            color: Color::Black,
            piece: Piece::Poon,
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

pub struct Action {
    pub from: Cell,
    pub to: Cell,
}

pub struct Game {
    pub history: Vec<Action>,
}

impl Game {
    pub fn is_move_valid(planned_action: Action, history: &[Action]) -> Result<(), InvalidMove> {
        let origin_cell = history
            .iter()
            .rev()
            .fold(planned_action.from, |cell, action| {
                if action.to == cell {
                    action.from
                } else {
                    cell
                }
            });

        let original_piece = &INITIAL_BOARD[origin_cell.row as usize][origin_cell.column as usize];

        match original_piece {
            Some(_) => Ok(()),
            None => Err(InvalidMove::EmptySourceCell),
        }
    }
}
