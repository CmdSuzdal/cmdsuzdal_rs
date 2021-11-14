//! Definition of the [ChessMove] structure used to store in a compact way a chess move,
//! and related methods implementation.
//!

use crate::chessdefines::*;

pub const EMPTY_CHESSMOVE: u32 = 0;
pub const INVALID_CHESSMOVE: u32 = 0x80_00_00_00;

/// Struct used to represent a Chess Move in a compact 32-bits wide representation.
///
/// The 32-bits wide bitmask uses the following bits:
///
///  - `bits[0..2]`: the moved piece 0 = King to 5 = Pawn, see [ChessPiece]
///  - `bits[3..5]`: in case of opposite army piece taken, the taken piece (can be 6 = InvalidPiece if no piece taken)
///  - `bits[6..8]`: in case of promotion, the piece chosen after promotion (can be 6 = InvalidPiece if no promotion)
///  - `bits[9]`: if 1, move is a check
///  - `bits[10]`: if 1, move is a checkmate
///  - `bits[11]`: not used
///  - `bits[12..17]`: the start Cell (0...63, cannot be invalid)
///  - `bits[18..23]`: the destination Cell (0...63, cannot be invalid)
///  - `bits[24..30]`: the en passant Cell (64 = InvalidCell if no en-passant)
///  - `bits[31]`: invalid move flag (1 = invalid)
///
///  # Examples:
///
///    - Pawn e2 to e3: `0 1000000 010100 001100 0001 1011 0101` = `0x4050C1B5`
///
#[derive(Default, Debug, PartialEq)]
pub struct ChessMove {
    pub m: u32,
}

impl From<u32> for ChessMove {
    fn from(m: u32) -> Self {
        ChessMove {m}
    }
}

// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_value_for_chess_move_is_empty_move() {
        let m: ChessMove = Default::default();
        assert_eq!(m.m, EMPTY_CHESSMOVE);
    }
}
