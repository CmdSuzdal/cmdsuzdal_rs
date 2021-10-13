#[rustfmt::skip]
/// A vertical file (or column) inside an 8x8 board.
///
/// Traditionally, in square board games the vertical files are represented
/// from left to right using the letters from 'A' to 'H', so the "File A"
/// is the leftmost column, whereas the "File H" is the rightmost one.
#[derive(Debug, Clone, Copy)]
pub enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH,
}

// Files Masks --- These are the files indexes of the board:
//    _________________________
// r8|  0  1  2  3  4  5  6  7 |
// r7|  0  1  2  3  4  5  6  7 |
// r6|  0  1  2  3  4  5  6  7 |
// r5|  0  1  2  3  4  5  6  7 |
// r4|  0  1  2  3  4  5  6  7 |
// r3|  0  1  2  3  4  5  6  7 |
// r2|  0  1  2  3  4  5  6  7 |
// r1|  0  1  2  3  4  5  6  7 |
//    -------------------------
//     fa fb fc fd fe ff fg fh
pub const FILES_BBS: [u64; 8] = [
    0x0101010101010101,
    0x0101010101010101 << 1,
    0x0101010101010101 << 2,
    0x0101010101010101 << 3,
    0x0101010101010101 << 4,
    0x0101010101010101 << 5,
    0x0101010101010101 << 6,
    0x0101010101010101 << 7,
];

#[rustfmt::skip]
/// An horizontal rank (or row) inside an 8x8 board.
///
/// Traditionally, in suqare board games the horizontal files are represented
/// from bottom to top using the numbers from '1' to '8', so the "Rank 1"
/// is the bottom row, whereas the "Rank 8" is the top one.
#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8,
}

// Ranks Masks --- These are the rank indexes of the board:
//    _________________________
// r8|  7  7  7  7  7  7  7  7 |
// r7|  6  6  6  6  6  6  6  6 |
// r6|  5  5  5  5  5  5  5  5 |
// r5|  4  4  4  4  4  4  4  4 |
// r4|  3  3  3  3  3  3  3  3 |
// r3|  2  2  2  2  2  2  2  2 |
// r2|  1  1  1  1  1  1  1  1 |
// r1|  0  0  0  0  0  0  0  0 |
//     -------------------------
//     fa fb fc fd fe ff fg fh
pub const RANKS_BBS: [u64; 8] = [
    0x00000000000000FF,
    0x00000000000000FF << 8,
    0x00000000000000FF << 16,
    0x00000000000000FF << 24,
    0x00000000000000FF << 32,
    0x00000000000000FF << 40,
    0x00000000000000FF << 48,
    0x00000000000000FF << 56,
];

#[rustfmt::skip]
/// Enumeration that lists all the cells inside an 8x8
/// square board.
///
/// The cells starts from A1 on bottom left (bit index 0) to H8 on top right (bit index 63).
///
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
pub enum Cell {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// Given a File and a Cell, returns the corresponding Cell
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(to_cell(File::FileC, Rank::Rank2), Cell::C2);
/// ```
///
pub fn to_cell(f: File, r: Rank) -> Cell {
    // It is not necessary to check for validity because f and r are safe
    // and the result of the from_u32 is a valid Cell for sure
    num::FromPrimitive::from_u32((r as u32) * 8 + (f as u32)).unwrap()
}

// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn try_all_to_cell_conversions() {
        assert_eq!(to_cell(File::FileA, Rank::Rank1), Cell::A1);
        assert_eq!(to_cell(File::FileA, Rank::Rank2), Cell::A2);
        assert_eq!(to_cell(File::FileA, Rank::Rank3), Cell::A3);
        assert_eq!(to_cell(File::FileA, Rank::Rank4), Cell::A4);
        assert_eq!(to_cell(File::FileA, Rank::Rank5), Cell::A5);
        assert_eq!(to_cell(File::FileA, Rank::Rank6), Cell::A6);
        assert_eq!(to_cell(File::FileA, Rank::Rank7), Cell::A7);
        assert_eq!(to_cell(File::FileA, Rank::Rank8), Cell::A8);
        assert_eq!(to_cell(File::FileB, Rank::Rank1), Cell::B1);
        assert_eq!(to_cell(File::FileB, Rank::Rank2), Cell::B2);
        assert_eq!(to_cell(File::FileB, Rank::Rank3), Cell::B3);
        assert_eq!(to_cell(File::FileB, Rank::Rank4), Cell::B4);
        assert_eq!(to_cell(File::FileB, Rank::Rank5), Cell::B5);
        assert_eq!(to_cell(File::FileB, Rank::Rank6), Cell::B6);
        assert_eq!(to_cell(File::FileB, Rank::Rank7), Cell::B7);
        assert_eq!(to_cell(File::FileB, Rank::Rank8), Cell::B8);
        assert_eq!(to_cell(File::FileC, Rank::Rank1), Cell::C1);
        assert_eq!(to_cell(File::FileC, Rank::Rank2), Cell::C2);
        assert_eq!(to_cell(File::FileC, Rank::Rank3), Cell::C3);
        assert_eq!(to_cell(File::FileC, Rank::Rank4), Cell::C4);
        assert_eq!(to_cell(File::FileC, Rank::Rank5), Cell::C5);
        assert_eq!(to_cell(File::FileC, Rank::Rank6), Cell::C6);
        assert_eq!(to_cell(File::FileC, Rank::Rank7), Cell::C7);
        assert_eq!(to_cell(File::FileC, Rank::Rank8), Cell::C8);
        assert_eq!(to_cell(File::FileD, Rank::Rank1), Cell::D1);
        assert_eq!(to_cell(File::FileD, Rank::Rank2), Cell::D2);
        assert_eq!(to_cell(File::FileD, Rank::Rank3), Cell::D3);
        assert_eq!(to_cell(File::FileD, Rank::Rank4), Cell::D4);
        assert_eq!(to_cell(File::FileD, Rank::Rank5), Cell::D5);
        assert_eq!(to_cell(File::FileD, Rank::Rank6), Cell::D6);
        assert_eq!(to_cell(File::FileD, Rank::Rank7), Cell::D7);
        assert_eq!(to_cell(File::FileD, Rank::Rank8), Cell::D8);
        assert_eq!(to_cell(File::FileE, Rank::Rank1), Cell::E1);
        assert_eq!(to_cell(File::FileE, Rank::Rank2), Cell::E2);
        assert_eq!(to_cell(File::FileE, Rank::Rank3), Cell::E3);
        assert_eq!(to_cell(File::FileE, Rank::Rank4), Cell::E4);
        assert_eq!(to_cell(File::FileE, Rank::Rank5), Cell::E5);
        assert_eq!(to_cell(File::FileE, Rank::Rank6), Cell::E6);
        assert_eq!(to_cell(File::FileE, Rank::Rank7), Cell::E7);
        assert_eq!(to_cell(File::FileE, Rank::Rank8), Cell::E8);
        assert_eq!(to_cell(File::FileF, Rank::Rank1), Cell::F1);
        assert_eq!(to_cell(File::FileF, Rank::Rank2), Cell::F2);
        assert_eq!(to_cell(File::FileF, Rank::Rank3), Cell::F3);
        assert_eq!(to_cell(File::FileF, Rank::Rank4), Cell::F4);
        assert_eq!(to_cell(File::FileF, Rank::Rank5), Cell::F5);
        assert_eq!(to_cell(File::FileF, Rank::Rank6), Cell::F6);
        assert_eq!(to_cell(File::FileF, Rank::Rank7), Cell::F7);
        assert_eq!(to_cell(File::FileF, Rank::Rank8), Cell::F8);
        assert_eq!(to_cell(File::FileG, Rank::Rank1), Cell::G1);
        assert_eq!(to_cell(File::FileG, Rank::Rank2), Cell::G2);
        assert_eq!(to_cell(File::FileG, Rank::Rank3), Cell::G3);
        assert_eq!(to_cell(File::FileG, Rank::Rank4), Cell::G4);
        assert_eq!(to_cell(File::FileG, Rank::Rank5), Cell::G5);
        assert_eq!(to_cell(File::FileG, Rank::Rank6), Cell::G6);
        assert_eq!(to_cell(File::FileG, Rank::Rank7), Cell::G7);
        assert_eq!(to_cell(File::FileG, Rank::Rank8), Cell::G8);
        assert_eq!(to_cell(File::FileH, Rank::Rank1), Cell::H1);
        assert_eq!(to_cell(File::FileH, Rank::Rank2), Cell::H2);
        assert_eq!(to_cell(File::FileH, Rank::Rank3), Cell::H3);
        assert_eq!(to_cell(File::FileH, Rank::Rank4), Cell::H4);
        assert_eq!(to_cell(File::FileH, Rank::Rank5), Cell::H5);
        assert_eq!(to_cell(File::FileH, Rank::Rank6), Cell::H6);
        assert_eq!(to_cell(File::FileH, Rank::Rank7), Cell::H7);
        assert_eq!(to_cell(File::FileH, Rank::Rank8), Cell::H8);
    }
}