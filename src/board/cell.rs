use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};

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
