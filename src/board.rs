use std::result::Result;
use std::vec::Vec;

pub enum Error {
    CellWrongArguments,
}

pub enum Color {
    White,
    Black,
}

pub struct Cell {
    pub row: u8,
    pub column: u8,
}

impl Cell {
    pub fn new(row: u8, column: u8) -> std::result::Result<Cell, Error> {
        if row > 7 || column > 7 {
            Result::Err(Error::CellWrongArguments)
        } else {
            Result::Ok(Cell { row, column })
        }
    }
}

trait Piece {
    fn possible_moves(&self) -> Vec<Cell>;
}

struct Poon {
    color: Color,
}

impl Piece for Poon {
    fn possible_moves(&self) -> Vec<Cell> {
        if let Color::White = self.color {
            println!("White poon")
        } else {
            println!("Black poon")
        }
        Vec::new()
    }
}

struct Knight();

impl Piece for Knight {
    fn possible_moves(&self) -> Vec<Cell> {
        Vec::new()
    }
}

struct Bishop();

impl Piece for Bishop {
    fn possible_moves(&self) -> Vec<Cell> {
        Vec::new()
    }
}

struct Rook();

impl Piece for Rook {
    fn possible_moves(&self) -> Vec<Cell> {
        Vec::new()
    }
}

struct Queen();

impl Piece for Queen {
    fn possible_moves(&self) -> Vec<Cell> {
        Vec::new()
    }
}

struct King();

impl Piece for King {
    fn possible_moves(&self) -> Vec<Cell> {
        Vec::new()
    }
}

pub struct Action {
    pub from: Cell,
    pub to: Cell,
}

pub struct Game {
    pub history: Vec<Action>,
}
