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
