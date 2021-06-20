use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub enum Error {
    CellWrongArguments,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PackedStruct)]
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
    /// Create new cell described by row and column
    pub fn new(row: u8, column: u8) -> Cell {
        debug_assert!(row < 8, "row {} is out of bounds", row);
        debug_assert!(column < 8, "column {} is out of bounds", column);
        Cell {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Get cell row
    pub fn row(self) -> i8 {
        u8::from(self.row) as i8
    }

    /// Get cell column
    pub fn col(self) -> i8 {
        u8::from(self.column) as i8
    }
}

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

pub enum InvalidMove {
    WrongTurn,
    EmptySourceCell,
    OutOfBounds,
    OutOfRange,
    OutOfSight,
    ProvokeCheck,
    FriendlyFire,
    InvalidPromotionPiece,
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
pub struct ActionRequest {
    pub from: Cell,
    pub to: Cell,
    pub piece: Option<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Standard(StandardAction),
    EnPassant(EnPassantAction),
    Promotion(PromotionAction),
    Castling(CastlingAction),
}

trait Move {
    fn from(&self) -> Cell;
    fn to(&self) -> Cell;
}

impl Move for StandardAction {
    fn from(&self) -> Cell {
        self.from
    }

    fn to(&self) -> Cell {
        self.to
    }
}

/// Action defined by source and destination cells
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardAction {
    pub from: Cell,
    pub to: Cell,
}

type EnPassantAction = StandardAction;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionAction {
    pub start_column: u8,
    pub color: Color,
    pub direction: Direction,
    pub promote_piece: PromotePiece,
}

impl PromotionAction {
    fn new(
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

#[derive(Debug, Serialize, Deserialize)]
pub enum CastlingSide {
    King,
    Queen,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastlingAction {
    pub side: CastlingSide,
    pub color: Color,
}

impl CastlingAction {
    fn tower_from(&self) -> Cell {
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

    fn tower_to(&self) -> Cell {
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

impl Action {
    fn from(&self) -> Cell {
        match self {
            Action::Promotion(promotion) => {
                let row = match promotion.color {
                    Color::Black => 1,
                    Color::White => 6,
                };
                Cell::new(row, promotion.start_column)
            }
            Action::Castling(castling) => {
                let row = match castling.color {
                    Color::White => 0,
                    Color::Black => 7,
                };
                Cell::new(row, 4)
            }
            Action::Standard(action) => action.from,
            Action::EnPassant(action) => action.from,
        }
    }

    fn to(&self) -> Cell {
        match self {
            Action::Promotion(promotion) => {
                let row = match promotion.color {
                    Color::Black => 0,
                    Color::White => 7,
                };

                let column: i8 = promotion.start_column as i8 + i8::from(promotion.direction);
                Cell::new(row, column as u8)
            }
            Action::Castling(castling) => {
                let row = match castling.color {
                    Color::White => 0,
                    Color::Black => 7,
                };
                let column = match castling.side {
                    CastlingSide::King => 6,
                    CastlingSide::Queen => 2,
                };
                Cell::new(row, column)
            }
            Action::Standard(action) => action.to,
            Action::EnPassant(action) => action.to,
        }
    }
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

impl From<Vec<Action>> for Game {
    fn from(history: Vec<Action>) -> Self {
        Game { history }
    }
}

impl From<Vec<ActionRequest>> for Game {
    fn from(history: Vec<ActionRequest>) -> Self {
        let mut game = Game::new();
        for action in history {
            game.push_move(action);
        }
        game
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

    /// Resets the game
    pub fn reset(&mut self) {
        self.history.clear()
    }

    /// Returns whether a move is valid
    pub fn is_move_valid(&self, planned_action: &ActionRequest) -> Result<Action, InvalidMove> {
        let from = planned_action.from;
        let to = planned_action.to;
        let mut castling_action: Option<CastlingAction> = None;

        // get piece corresponding to original cell
        let original_piece = match self.get_piece_at(from) {
            Some(p) => p,
            None => {
                // reject action if moving air
                return Err(InvalidMove::EmptySourceCell);
            }
        };

        // reject action if incorrect color
        if original_piece.color != self.get_turn() {
            return Err(InvalidMove::WrongTurn);
        }

        // reject if friendly fire
        if self
            .get_piece_at(to)
            .as_ref()
            .map_or(false, |p| p.color == original_piece.color)
        {
            return Err(InvalidMove::FriendlyFire);
        }

        // calculate distance from origin
        let row_distance = (to.row() - from.row()).abs();
        let col_distance = (to.col() - from.col()).abs();

        // movement direction
        let direction_row = (to.row() - from.row()).signum();
        let direction_col = (to.col() - from.col()).signum();

        // check whether the piece makes a legal move
        let empty_cells = match &original_piece.piece {
            Piece::Knight => {
                if !((row_distance == 2 && col_distance == 1)
                    || (row_distance == 1 && col_distance == 2))
                {
                    return Err(InvalidMove::OutOfRange);
                }
                Vec::new()
            }
            Piece::Bishop => {
                // diagonal moves only
                if row_distance != col_distance {
                    return Err(InvalidMove::OutOfRange);
                }
                (1..row_distance)
                    .map(|i| {
                        Cell::new(
                            (from.row() + i * direction_row) as u8,
                            (from.col() + i * direction_col) as u8,
                        )
                    })
                    .collect()
            }
            Piece::Rook => {
                // horizontal or vertical moves only
                if row_distance != 0 && col_distance != 0 {
                    return Err(InvalidMove::OutOfRange);
                }
                (1..row_distance.max(col_distance))
                    .map(|i| {
                        Cell::new(
                            (from.row() + i * direction_row) as u8,
                            (from.col() + i * direction_col) as u8,
                        )
                    })
                    .collect()
            }
            Piece::King => {
                let tmp_castling_action = CastlingAction {
                    side: match direction_col {
                        -1 => CastlingSide::Queen,
                        _ => CastlingSide::King,
                    },
                    color: original_piece.color,
                };
                if from == tmp_castling_action.from() && to == tmp_castling_action.to() {
                    // castling action
                    // TODO: check that king and rook haven't moved
                    // TODO: check that king is not in check
                    let tower_from_col = tmp_castling_action.tower_from().col();
                    castling_action = Some(tmp_castling_action);
                    (1..(from.col() - tower_from_col).abs())
                        .map(|i| {
                            Cell::new(from.row() as u8, (from.col() + i * direction_col) as u8)
                        })
                        .collect()
                } else {
                    // move to closest squares only
                    if row_distance > 1 || col_distance > 1 {
                        return Err(InvalidMove::OutOfRange);
                    }
                    // no squares to check for emptiness
                    Vec::new()
                }
            }
            Piece::Queen => {
                // diagonal, horizontal or vertical moves only
                if row_distance != col_distance && row_distance != 0 && col_distance != 0 {
                    return Err(InvalidMove::OutOfRange);
                }
                (1..row_distance.max(col_distance))
                    .map(|i| {
                        Cell::new(
                            (from.row() + i * direction_row) as u8,
                            (from.col() + i * direction_col) as u8,
                        )
                    })
                    .collect()
            }
            Piece::Pawn => {
                let direction = match original_piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                // move two cells away, straight, from initial row
                if (-5 * direction + 7) / 2 == from.row()
                    && col_distance == 0
                    && to.row() == from.row() + 2 * direction
                {
                    (1..3)
                        .map(|i| {
                            Cell::new((from.row() + i * direction_row) as u8, (from.col()) as u8)
                        })
                        .collect()
                // move one cell away
                } else if to.row() == from.row() + direction && col_distance <= 1 {
                    let dest_piece = self.get_piece_at(to);
                    if col_distance == 0 && dest_piece.is_some()
                        || col_distance == 1 && dest_piece.is_none()
                    {
                        return Err(InvalidMove::OutOfRange);
                    }
                    Vec::new()
                } else {
                    return Err(InvalidMove::OutOfRange);
                }
            }
        };

        // check cells in way of movement for emptiness
        if empty_cells
            .iter()
            .any(|&cell| self.get_piece_at(cell).is_some())
        {
            return Err(InvalidMove::OutOfSight);
        }

        // final action
        let action = if original_piece.piece == Piece::Pawn
            && (match original_piece.color {
                Color::White => 7,
                Color::Black => 0,
            }) == to.row()
        {
            // promotion action
            let promote_piece = match planned_action.piece {
                Some(piece) => match piece {
                    'n' => Piece::Knight,
                    'r' => Piece::Rook,
                    'b' => Piece::Bishop,
                    'q' => Piece::Queen,
                    _ => return Err(InvalidMove::InvalidPromotionPiece),
                },
                None => Piece::Queen,
            };
            Action::Promotion(PromotionAction::new(
                from.col() as u8,
                original_piece.color,
                Direction::from(direction_col),
                promote_piece,
            ))
        } else if castling_action.is_some() {
            Action::Castling(castling_action.unwrap())
        } else {
            // standard action
            Action::Standard(StandardAction { from, to })
        };

        Ok(action)
    }

    /// Returns piece at the given location
    pub fn get_piece_at(&self, cell: Cell) -> Option<BoardPiece> {
        // rewind game to find original cell
        let mut board_piece: Option<BoardPiece> = None;
        let original_cell = self.history.iter().rev().try_fold(cell, |cell, action| {
            match action {
                // piece has moved away, cell is empty
                Action::Castling(action) if action.tower_from() == cell => None,
                action if action.from() == cell => None,
                // promotion: board piece is the one set by the promotion
                Action::Promotion(action) if action.to() == cell => {
                    board_piece = Some(BoardPiece {
                        color: action.color,
                        piece: Piece::from(action.promote_piece),
                    });
                    None
                }
                // piece enters the cell, capturing piece origin
                Action::Castling(action) if action.tower_to() == cell => Some(action.tower_from()),
                action if action.to() == cell => Some(action.from()),
                // no change
                _ => Some(cell),
            }
        });

        if board_piece.is_some() {
            return board_piece;
        }

        // get original piece
        original_cell.and_then(|c| INITIAL_BOARD[c.row() as usize][c.col() as usize])
    }

    /// Process action and add it to action history if it is valid
    pub fn push_move(&mut self, planned_action: ActionRequest) -> Option<InvalidMove> {
        let res = self.is_move_valid(&planned_action);
        if res.is_ok() {
            self.history.push(res.ok().unwrap());
            None
        } else {
            res.err()
        }
    }

    /// Remove the last action if any
    pub fn undo_move(&mut self) -> Option<Action> {
        self.history.pop()
    }
}
