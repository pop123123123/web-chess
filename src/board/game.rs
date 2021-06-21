use super::action::castling::{CastlingAction, CastlingSide};
use super::action::promotion::{Direction, PromotionAction};
use super::action::standard::StandardAction;
use super::action::{Action, ActionRequest, Move};
use super::cell::Cell;
use super::constants::INITIAL_BOARD;
use super::error::InvalidMove;
use super::piece::{BoardPiece, Color, Piece};
use crate::board::action::en_passant::EnPassantAction;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

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
        let mut en_passant_action: bool = false;

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
                        // en passant
                        let last_action = match self.history.last() {
                            Some(a) => a,
                            None => return Err(InvalidMove::OutOfRange),
                        };
                        let is_last_moved_piece_pawn =
                            self.get_piece_at(last_action.to()).unwrap().piece == Piece::Pawn;
                        let last_opponent_move_is_double =
                            (last_action.to().row() - last_action.from().row()).abs() == 2;
                        let is_same_column = to.col() == last_action.to().col();

                        if is_last_moved_piece_pawn
                            && last_opponent_move_is_double
                            && is_same_column
                        {
                            en_passant_action = true;
                        } else {
                            return Err(InvalidMove::OutOfRange);
                        }
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
        } else if en_passant_action {
            Action::EnPassant(EnPassantAction { from, to })
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
                // en passant: target cell is empty
                Action::EnPassant(action) if action.target_cell() == cell => None,
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
