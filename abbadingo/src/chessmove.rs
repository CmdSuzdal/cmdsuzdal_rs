//! Definition of the [ChessMove] structure used to store in a compact way a chess move,
//! and related methods implementation.
//!

use crate::bbdefines::*;
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
///  - `bits[9..11]`: not used
///  - `bits[12..17]`: the start Cell (0...63, cannot be invalid)
///  - `bits[18..23]`: the destination Cell (0...63, cannot be invalid)
///  - `bits[24..30]`: the en passant Cell (64 = InvalidCell if no en-passant)
///  - `bits[31]`: invalid move flag (1 = invalid)
///
///  # Examples:
///
///    - Pawn e2 to e3: `0 1000000 010100 001100 000 110 110 101` =
///                     `0100 0000 0101 0000 1100 0001 1011 0101` = `0x4050C1B5`
///

const TAKEN_PIECE_OFFSET: u32 = 3;
const PROMOTED_PIECE_OFFSET: u32 = 6;
const START_CELL_OFFSET: u32 = 12;
const DESTINATION_CELL_OFFSET: u32 = 18;
const EN_PASSANT_CELL_OFFSET: u32 = 24;

const PIECE_MASK: u32 = 0x00000007;
const VALID_CELL_MASK: u32 = 0x0000003F;
//const VALID_AND_INVALID_CELL_MASK: u32 = 0x0000007F;

const INVALID_PIECE: u32 = 0x00000006;
const INVALID_CELL: u32 = 0x00000040;

#[derive(Default, Debug, PartialEq)]
pub struct ChessMove {
    pub m: u32,
}

impl ChessMove {
    /// Default constructor of the [ChessMove] structure
    ///
    /// # Arguments
    ///
    /// * `moved_piece`: The [ChessPiece] being moved
    /// * `start_cell`: The starting [Cell] of the moved piece in the board
    /// * `destination_cell`: The destination [Cell] of the moved piece in the board
    /// * `taken_piece`: The type of the [ChessPiece] taken if any (None otherwise)
    /// * `promoted_piece`: The type of the [ChessPiece] the pawn is promoted to if any (None otherwise)
    ///
    pub fn new(
        moved_piece: ChessPiece,
        start_cell: Cell,
        dest_cell: Cell,
        taken_piece: Option<ChessPiece>,
        promoted_piece: Option<ChessPiece>,
    ) -> ChessMove {
        let mut m = moved_piece as u32;
        if let Some(p) = taken_piece {
            m |= ((p as u32) & PIECE_MASK) << TAKEN_PIECE_OFFSET;
        } else {
            m |= INVALID_PIECE << TAKEN_PIECE_OFFSET;
        }
        if let Some(p) = promoted_piece {
            m |= ((p as u32) & PIECE_MASK) << PROMOTED_PIECE_OFFSET;
        } else {
            m |= INVALID_PIECE << PROMOTED_PIECE_OFFSET;
        }
        m |= ((start_cell as u32) & VALID_CELL_MASK) << START_CELL_OFFSET;
        m |= ((dest_cell as u32) & VALID_CELL_MASK) << DESTINATION_CELL_OFFSET;

        if moved_piece == ChessPiece::Pawn {
            if let Some(c) = ChessMove::compute_en_passant(start_cell, dest_cell) {
                m |= ((c as u32) & VALID_CELL_MASK) << EN_PASSANT_CELL_OFFSET;
            } else {
                m |= INVALID_CELL << EN_PASSANT_CELL_OFFSET;
            }
        } else {
            m |= INVALID_CELL << EN_PASSANT_CELL_OFFSET;
        }
        ChessMove { m }
    }

    /// Evalues if a move causes en-passant cell (assuming the moving piece is a Pawn)
    ///
    fn compute_en_passant(from: Cell, to: Cell) -> Option<Cell> {
        let r = rank(from);

        // If from cell is in 2nd rank and to = from + 16
        // we have an en passant in from + 8
        if r == Rank::Rank2 && to == num::FromPrimitive::from_i32(from as i32 + 16).unwrap() {
            return n(from);
        }
        // If from cell is in 7th rank and to = from - 16
        // we have an en passant in from - 8
        if r == Rank::Rank7 && to == num::FromPrimitive::from_i32(from as i32 - 16).unwrap() {
            return s(from);
        }
        None
    }
}
//impl From<u32> for ChessMove {
//    fn from(m: u32) -> Self {
//        ChessMove {m}
//    }
//}

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

    #[test]
    fn panw_from_e2_to_e3_move() {
        //    - Pawn (5 = 101) e2 (12 = 001100) to e3 (20 010100)
        //      0 1000000 010100 001100 000 110 110 101
        //      0100 0000 0101 0000 1100 0001 1011 0101 = 0x4050C1B5
        assert_eq!(
            ChessMove::new(ChessPiece::Pawn, Cell::E2, Cell::E3, None, None),
            ChessMove { m: 0x4050C1B5 }
        );
    }

    #[test]
    fn pawn_from_e4_to_f5_with_pawn_taken_move() {
        //    - Pawn (5 = 101) from e4 (28 = 011100) to f5 (37 = 100101) with Pawn (5 = 101) taken
        //      0 1000000 100101 011100 000 110 101 101
        //      0100 0000 1001 0101 1100 0001 1010 1101 = 0x4095 C1AD
        assert_eq!(
            ChessMove::new(
                ChessPiece::Pawn,
                Cell::E4,
                Cell::F5,
                Some(ChessPiece::Pawn),
                None
            ),
            ChessMove { m: 0x4095C1AD }
        );
    }

    #[test]
    fn knight_from_d2_to_b3_with_pawn_taken_move() {
        //    - Knight (3 = 011) from d2 (11 = 001011) to b3 (17 = 010001) with Pawn (5 = 101) taken
        //      0 1000000 010001 001011 000 110 101 011
        //      0100 0000 0100 0100 1011 0001 1010 1011 = 0x4044B1AB
        assert_eq!(
            ChessMove::new(
                ChessPiece::Knight,
                Cell::D2,
                Cell::B3,
                Some(ChessPiece::Pawn),
                None
            ),
            ChessMove { m: 0x4044B1AB }
        );
    }

    #[test]
    fn bishop_a1_to_h8_with_queen_taken_move() {
        //    - Bishop (2 = 010) from a1 (0 = 000000) to h8 (63 = 111111) with Queen (1 = 001) taken
        //      0 1000000 111111 000000 000 110 001 010
        //      0100 0000 1111 1100 0000 0001 1000 1010 = 0x40FC018A
        assert_eq!(
            ChessMove::new(
                ChessPiece::Bishop,
                Cell::A1,
                Cell::H8,
                Some(ChessPiece::Queen),
                None
            ),
            ChessMove { m: 0x40FC018A }
        );
    }

    #[test]
    fn rook_from_g7_to_a7_with_bishop_taken_move() {
        //    - Rook (4 = 100) from g7 (54 = 110110) to a7 (48 = 110000) with Bishop (2 = 010) taken
        //      0 1000000 110000 110110 000 110 010 100
        //      0100 0000 1100 0011 0110 0001 1001 0100 = 0x40C36194
        assert_eq!(
            ChessMove::new(
                ChessPiece::Rook,
                Cell::G7,
                Cell::A7,
                Some(ChessPiece::Bishop),
                None
            ),
            ChessMove { m: 0x40C36194 }
        );
    }

    #[test]
    fn queen_from_e1_to_e8_with_queen_taken_move() {
        //    - Queen (1 = 001) from e1 (4 = 000100) to e8 (60 = 111100) with Queen (1 = 001) taken
        //      0 1000000 111100 000100 000 110 001 001
        //      0100 0000 1111 0000 0100 0001 1000 1001 = 0x40F04189
        assert_eq!(
            ChessMove::new(
                ChessPiece::Queen,
                Cell::E1,
                Cell::E8,
                Some(ChessPiece::Queen),
                None
            ),
            ChessMove { m: 0x40F04189 }
        );
    }

    #[test]
    fn king_from_d3_to_e4_with_pawn_taken_move() {
        //    - King (0 = 000) from d3 (19 = 010011) to e4 (28 = 011100) with Pawn (5 = 101) taken
        //      0 1000000 011100 010011 000 110 101 000
        //      0100 0000 0111 0001 0011 0001 1010 1000 = 0x407131A8
        assert_eq!(
            ChessMove::new(
                ChessPiece::King,
                Cell::D3,
                Cell::E4,
                Some(ChessPiece::Pawn),
                None
            ),
            ChessMove { m: 0x407131A8 }
        );
    }

    // Promotions testing
    #[test]
    fn pawn_from_b7_to_b8_with_promotion_to_queen() {
        // Pawn (5 = 101) from b7 (49 = 110001) to b8 (57 = 111001) with no piece taken and promotion to Queen (1 = 001)
        //      0 1000000 111001 110001 000 001 110 101
        //      0100 0000 1110 0111 0001 0000 0111 0101 = 0x40E71075
        assert_eq!(
            ChessMove::new(
                ChessPiece::Pawn,
                Cell::B7,
                Cell::B8,
                None,
                Some(ChessPiece::Queen)
            ),
            ChessMove { m: 0x40E71075 }
        );
    }

    #[test]
    fn pawn_from_g2_to_h1_with_rook_taken_and_promotion_to_knight() {
        // Pawn (5 = 101) from g2 (14 = 001110) to h1 (7 = 000111) with Rook (4 = 0100) taken and promotion to Knigh (3 = 011)
        //      0 1000000 000111 001110 000 011 100 101
        //      0100 0000 0001 1100 1110 0000 1110 0101 = 0x401CE0E5
        assert_eq!(
            ChessMove::new(
                ChessPiece::Pawn,
                Cell::G2,
                Cell::H1,
                Some(ChessPiece::Rook),
                Some(ChessPiece::Knight)
            ),
            ChessMove { m: 0x401CE0E5 }
        );
    }

    // En-passant testing
    #[test]
    fn pawn_from_c2_to_c4_with_implicit_en_passant_in_c3() {
        // Pawn (5 = 101) from c2 (10 = 001010) to c4 (26 = 011010) with en passant in c3 (18 = 010010)
        //      0 0010010 011010 001010 000 110 110 101
        //      0001 0010 0110 1000 1010 0001 1011 0101 = 0x1268A1B5
        assert_eq!(
            ChessMove::new(ChessPiece::Pawn, Cell::C2, Cell::C4, None, None),
            ChessMove { m: 0x1268A1B5 }
        );
    }

    #[test]
    fn pawn_from_d7_to_d5_with_implicit_en_passant_in_d6() {
        // Pawn (5 = 101) from d7 (51 = 110011) to d5 (35 = 100011) with en passant in d6 (43 = 101011)
        //      0 0101011 100011 110011 000 110 110 101
        //      0010 1011 1000 1111 0011 0001 1011 0101 = 0x2B8F31B5
        assert_eq!(
            ChessMove::new(ChessPiece::Pawn, Cell::D7, Cell::D5, None, None),
            ChessMove { m: 0x2B8F31B5 }
        );
    }

    #[test]
    fn queen_from_e2_to_e4_does_not_generate_en_passant_in_e3() {
        // Queen (1 = 001) from e2 (12 = 001100) to e4 (28 = 011100) does not generate en passant
        //      0 1000000 011100 001100 000 110 110 001
        //      0100 0000 0111 0000 1100 0001 1011 0001 = 0x4070C1B1
        assert_eq!(
            ChessMove::new(ChessPiece::Queen, Cell::E2, Cell::E4, None, None),
            ChessMove { m: 0x4070C1B1 }
        );
    }

    #[test]
    fn rook_from_d7_to_d5_does_not_generate_en_passant_in_d6() {
        // Rook (4 = 100) from d7 (51 = 110011) to d5 (35 = 100011) does not generate en passant
        //      0 1000000 100011 110011 000 110 110 100
        //      0100 0000 1000 1111 0011 0001 1011 0100 = 0x408F31B4
        assert_eq!(
            ChessMove::new(ChessPiece::Rook, Cell::D7, Cell::D5, None, None),
            ChessMove { m: 0x408F31B4 }
        );
    }
}
