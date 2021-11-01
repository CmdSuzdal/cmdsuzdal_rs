//! Base definitions of items used in square 8x8 bitboard representation
//!
//! In this module there are definition for Cells, Files, Ranks and other
//! concepts used in definition and manipulation of [BitBoard](crate::bitboard::BitBoard).)s.

use std::convert::TryFrom;
use std::fmt;

use crate::error::AbbaDingoError;

// ********************************************************************************
// ********************************************************************************
// ENUMs, STRUCTs, DEFINEs
// ********************************************************************************
// ********************************************************************************

pub type BitBoardState = u64;

pub const EMPTY_STATE: BitBoardState = 0;

#[rustfmt::skip]
/// A vertical file (or column) inside an 8x8 board.
///
/// Traditionally, in square board games the vertical files are represented
/// from left to right using the letters from 'A' to 'H', so the "File A"
/// is the leftmost column, whereas the "File H" is the rightmost one.
#[derive(Debug, Clone, Copy, FromPrimitive, PartialOrd, PartialEq, Eq)]
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
pub const FILES_BBS: [BitBoardState; 8] = [
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
/// Traditionally, in square board games the horizontal files are represented
/// from bottom to top using the numbers from '1' to '8', so the "Rank 1"
/// is the bottom row, whereas the "Rank 8" is the top one.
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
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
pub const RANKS_BBS: [BitBoardState; 8] = [
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

/// The number of [Cell]s in a 8x8 [BitBoard]
pub const NUM_CELLS: usize = 64;

/// A Diagonal inside an 8x8 board.
///
/// Traditionally, in square board games the first diagonal (#0) is the
/// upper left one, composed only by the cell A8, diagonal #1 is one
/// immediately to the right composed by the cells A7 and B8 and so on
/// until the last diagonal #14 in the lower right corner, composed only by
/// the H1 cell. The 'main' diagonal is the #7, starting from the lower left
/// cell A1 to the upper right corner H8.
///
///```text
///    _________________________
/// r8|  0  1  2  3  4  5  6  7 |
/// r7|  1  2  3  4  5  6  7  8 |
/// r6|  2  3  4  5  6  7  8  9 |
/// r5|  3  4  5  6  7  8  9 10 |
/// r4|  4  5  6  7  8  9 10 11 |
/// r3|  5  6  7  8  9 10 11 12 |
/// r2|  6  7  8  9 10 11 12 13 |
/// r1|  7  8  9 10 11 12 13 14 |
///    -------------------------
///     fa fb fc fd fe ff fg fh
//```
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
pub enum Diagonal {
    Diag0,
    Diag1,
    Diag2,
    Diag3,
    Diag4,
    Diag5,
    Diag6,
    Diag7,
    Diag8,
    Diag9,
    Diag10,
    Diag11,
    Diag12,
    Diag13,
    Diag14,
}

// Diagonal masks
pub const DIAGS_BBS: [BitBoardState; 15] = [
    0x0100000000000000_u64, //  0
    0x0201000000000000_u64, //  1
    0x0402010000000000_u64, //  2
    0x0804020100000000_u64, //  3
    0x1008040201000000_u64, //  4
    0x2010080402010000_u64, //  5
    0x4020100804020100_u64, //  6
    0x8040201008040201_u64, //  7
    0x0080402010080402_u64, //  8
    0x0000804020100804_u64, //  9
    0x0000008040201008_u64, // 10
    0x0000000080402010_u64, // 11
    0x0000000000804020_u64, // 12
    0x0000000000008040_u64, // 13
    0x0000000000000080_u64, // 14
];

/// As Anti Diagonal inside an 8x8 board.
///
/// Traditionally, in square board games the first anti diagonal (#0) is the
/// lower left one, composed only by the cell A1, anti diagonal #1 is one
/// immediately to the right composed by the cells B1 and A2 and so on
/// until the last anti diagonal #14 in the upper right corner, composed only
/// by the H8 cell. The 'main' anti diagonal is the #7, starting from the
/// lower right cell H1 to the upper left cell A8.
///
///```text
///    _________________________
/// r8|  7  8  9 10 11 12 13 14 |
/// r7|  6  7  8  9 10 11 12 13 |
/// r6|  5  6  7  8  9 10 11 12 |
/// r5|  4  5  6  7  8  9 10 11 |
/// r4|  3  4  5  6  7  8  9 10 |
/// r3|  2  3  4  5  6  7  8  9 |
/// r2|  1  2  3  4  5  6  7  8 |
/// r1|  0  1  2  3  4  5  6  7 |
///    -------------------------
///     fa fb fc fd fe ff fg fh
//```
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
pub enum AntiDiagonal {
    AntiDiag0,
    AntiDiag1,
    AntiDiag2,
    AntiDiag3,
    AntiDiag4,
    AntiDiag5,
    AntiDiag6,
    AntiDiag7,
    AntiDiag8,
    AntiDiag9,
    AntiDiag10,
    AntiDiag11,
    AntiDiag12,
    AntiDiag13,
    AntiDiag14,
}

// Diagonal masks
pub const ANTIDIAGS_BBS: [BitBoardState; 15] = [
    0x0000000000000001_u64, //  0
    0x0000000000000102_u64, //  1
    0x0000000000010204_u64, //  2
    0x0000000001020408_u64, //  3
    0x0000000102040810_u64, //  4
    0x0000010204081020_u64, //  5
    0x0001020408102040_u64, //  6
    0x0102040810204080_u64, //  7
    0x0204081020408000_u64, //  8
    0x0408102040800000_u64, //  9
    0x0810204080000000_u64, // 10
    0x1020408000000000_u64, // 11
    0x2040800000000000_u64, // 12
    0x4080000000000000_u64, // 13
    0x8000000000000000_u64, // 14
];

// ********************************************************************************
// ********************************************************************************
// METHODS
// ********************************************************************************
// ********************************************************************************

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

/// Given a Cell, returns its File
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(file(Cell::C5), File::FileC);
/// assert_eq!(file(Cell::A7), File::FileA);
/// assert_eq!(file(Cell::H1), File::FileH);
/// ```
///
pub fn file(c: Cell) -> File {
    // It is not necessary to check for validity because c is safe
    // and the result of the from_u32 is a valid File for sure
    num::FromPrimitive::from_u32(c as u32 % 8).unwrap()
}

/// Given a Cell, returns its Rank
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(rank(Cell::B2), Rank::Rank2);
/// assert_eq!(rank(Cell::F5), Rank::Rank5);
/// assert_eq!(rank(Cell::G8), Rank::Rank8);
/// ```
///
pub fn rank(c: Cell) -> Rank {
    // It is not necessary to check for validity because c is safe
    // and the result of the from_u32 is a valid Rank for sure
    num::FromPrimitive::from_u32(c as u32 >> 3).unwrap()
}

/// Given a Cell, returns its (File, Rank) pair
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(coords(Cell::C7), (File::FileC, Rank::Rank7));
/// assert_eq!(coords(Cell::E1), (File::FileE, Rank::Rank1));
/// assert_eq!(coords(Cell::G4), (File::FileG, Rank::Rank4));
/// ```
///
pub fn coords(c: Cell) -> (File, Rank) {
    (file(c), rank(c))
}

/// Given a Cell, returns its Diagonal
///
///```text
///    _________________________
/// r8|  0  1  2  3  4  5  6  7 |
/// r7|  1  2  3  4  5  6  7  8 |
/// r6|  2  3  4  5  6  7  8  9 |
/// r5|  3  4  5  6  7  8  9 10 |
/// r4|  4  5  6  7  8  9 10 11 |
/// r3|  5  6  7  8  9 10 11 12 |
/// r2|  6  7  8  9 10 11 12 13 |
/// r1|  7  8  9 10 11 12 13 14 |
///    -------------------------
///     fa fb fc fd fe ff fg fh
///
///```
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(diagonal(Cell::B3), Diagonal::Diag6);
/// assert_eq!(diagonal(Cell::E4), Diagonal::Diag8);
/// assert_eq!(diagonal(Cell::G2), Diagonal::Diag12);
/// ```
///
pub fn diagonal(c: Cell) -> Diagonal {
    // It is not necessary to check for validity because c is safe
    // and the result of the from_i32 is a valid Diagonal for sure
    num::FromPrimitive::from_i32(file(c) as i32 - rank(c) as i32 + 7).unwrap()
}

/// Given a Cell, returns its AntiDiagonal
///
///```text
///    _________________________
/// r8|  7  8  9 10 11 12 13 14 |
/// r7|  6  7  8  9 10 11 12 13 |
/// r6|  5  6  7  8  9 10 11 12 |
/// r5|  4  5  6  7  8  9 10 11 |
/// r4|  3  4  5  6  7  8  9 10 |
/// r3|  2  3  4  5  6  7  8  9 |
/// r2|  1  2  3  4  5  6  7  8 |
/// r1|  0  1  2  3  4  5  6  7 |
///    -------------------------
///     fa fb fc fd fe ff fg fh
///```
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(anti_diagonal(Cell::A3), AntiDiagonal::AntiDiag2);
/// assert_eq!(anti_diagonal(Cell::C6), AntiDiagonal::AntiDiag7);
/// assert_eq!(anti_diagonal(Cell::H8), AntiDiagonal::AntiDiag14);
/// ```
///
pub fn anti_diagonal(c: Cell) -> AntiDiagonal {
    // It is not necessary to check for validity because c is safe
    // and the result of the from_i32 is a valid Diagonal for sure
    num::FromPrimitive::from_i32(file(c) as i32 + rank(c) as i32).unwrap()
}

/// Given a Cell, returns its (Diagonal, AntiDiagonal) pair
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(diags(Cell::C6), (Diagonal::Diag4, AntiDiagonal::AntiDiag7));
/// assert_eq!(diags(Cell::E2), (Diagonal::Diag10, AntiDiagonal::AntiDiag5));
/// assert_eq!(diags(Cell::G7), (Diagonal::Diag7, AntiDiagonal::AntiDiag12));
/// ```
///
pub fn diags(c: Cell) -> (Diagonal, AntiDiagonal) {
    (diagonal(c), anti_diagonal(c))
}

/// Given a cell, returns its west file.
///
/// The west file is not defined for cells in A file, so this function
/// returns an Option<File>, returning a valid File if the Cell is on
/// File B to H, or None if the Cell is in the A file.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// # use abbadingo::error::AbbaDingoError;
/// assert_eq!(west(Cell::C5), Some(File::FileB));
/// assert_eq!(west(Cell::A7), None);
/// ```
pub fn west(c: Cell) -> Option<File> {
    num::FromPrimitive::from_i32(file(c) as i32 - 1)
}

/// Given a cell, returns its east file.
///
/// The east file is not defined for cells in H file, so this function
/// returns an Option<File>, returning a valid File if the Cell is on
/// File A to G, or None if the Cell is in the H file.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(east(Cell::A1), Some(File::FileB));
/// assert_eq!(east(Cell::E3), Some(File::FileF));
/// assert_eq!(east(Cell::H2), None);
/// ```
pub fn east(c: Cell) -> Option<File> {
    num::FromPrimitive::from_i32(file(c) as i32 + 1)
}

/// Given a cell, returns its south rank.
///
/// The south rank is not defined for cells in rank 1, so this
/// function returns an Option<Rank>, returning a valid Rank if
/// the Cell is on Rank 2 to 8, or None if the Cell is in the Rank 1.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(south(Cell::A4), Some(Rank::Rank3));
/// assert_eq!(south(Cell::E8), Some(Rank::Rank7));
/// assert_eq!(south(Cell::G1), None);
/// ```
pub fn south(c: Cell) -> Option<Rank> {
    num::FromPrimitive::from_i32(rank(c) as i32 - 1)
}

/// Given a cell, returns its north rank.
///
/// The north rank is not defined for cells in rank 8, so this
/// function returns an Option<Rank>, returning a valid Rank if
/// the Cell is on Rank 1 to 7, or None if the Cell is in the Rank 8.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(north(Cell::C1), Some(Rank::Rank2));
/// assert_eq!(north(Cell::B4), Some(Rank::Rank5));
/// assert_eq!(north(Cell::H8), None);
/// ```
pub fn north(c: Cell) -> Option<Rank> {
    num::FromPrimitive::from_i32(rank(c) as i32 + 1)
}

/// Given a cell, returns the cell at its "west" position.
///
/// The west position is not defined for cells in the A file, so
/// this function returns an Option<Cell>, returning None if the
/// cell is in a position without a valid west neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(w(Cell::C4), Some(Cell::B4));
/// assert_eq!(w(Cell::G7), Some(Cell::F7));
/// assert_eq!(w(Cell::A3), None);
/// ```
pub fn w(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileA => None,
        _ => num::FromPrimitive::from_i32(c as i32 - 1),
    }
}

/// Given a cell, returns the cell at its "north-west" position.
///
/// The north-west position is not defined for cells in the A file,
/// or in the Rank 8, so this function return an Option<Cell>,
/// returning None if the cell is in a position without a valid
/// north-west neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(nw(Cell::B1), Some(Cell::A2));
/// assert_eq!(nw(Cell::D5), Some(Cell::C6));
/// assert_eq!(nw(Cell::H7), Some(Cell::G8));
/// assert_eq!(nw(Cell::A3), None);
/// assert_eq!(nw(Cell::C8), None);
/// ```
pub fn nw(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileA => None,
        _ => num::FromPrimitive::from_i32(c as i32 + 7),
    }
}

/// Given a cell, returns the cell at its "north" position.
///
/// The north position is not defined for cells in the Rank 8,
/// so this function return an Option<Cell>, returning None if
/// the cell is in a position without a valid north neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(n(Cell::A1), Some(Cell::A2));
/// assert_eq!(n(Cell::C2), Some(Cell::C3));
/// assert_eq!(n(Cell::G7), Some(Cell::G8));
/// assert_eq!(n(Cell::F8), None);
/// ```
pub fn n(c: Cell) -> Option<Cell> {
    match rank(c) {
        Rank::Rank8 => None,
        _ => num::FromPrimitive::from_i32(c as i32 + 8),
    }
}

/// Given a cell, returns the cell at its "north-east" position.
///
/// The north-east position is not defined for cells in the H file,
/// or in the Rank 8, so this function return an Option<Cell>,
/// returning None if the cell is in a position without a valid
/// north-east neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(ne(Cell::A7), Some(Cell::B8));
/// assert_eq!(ne(Cell::E3), Some(Cell::F4));
/// assert_eq!(ne(Cell::G1), Some(Cell::H2));
/// assert_eq!(ne(Cell::D8), None);
/// assert_eq!(ne(Cell::H2), None);
/// ```
pub fn ne(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileH => None,
        _ => num::FromPrimitive::from_i32(c as i32 + 9),
    }
}

/// Given a cell, returns the cell at its "east" position.
///
/// The east position is not defined for cells in the H file, so
/// this function returns an Option<Cell>, returning None if the
/// cell is in a position without a valid east neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(e(Cell::A8), Some(Cell::B8));
/// assert_eq!(e(Cell::C6), Some(Cell::D6));
/// assert_eq!(e(Cell::H3), None);
/// ```
pub fn e(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileH => None,
        _ => num::FromPrimitive::from_i32(c as i32 + 1),
    }
}

/// Given a cell, returns the cell at its "south-east" position.
///
/// The south-east position is not defined for cells in the H file,
/// or in the Rank 1, so this function return an Option<Cell>,
/// returning None if the cell is in a position without a valid
/// south-east neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(se(Cell::E2), Some(Cell::F1));
/// assert_eq!(se(Cell::G5), Some(Cell::H4));
/// assert_eq!(se(Cell::B8), Some(Cell::C7));
/// assert_eq!(se(Cell::H5), None);
/// assert_eq!(se(Cell::G1), None);
/// ```
pub fn se(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileH => None,
        _ => num::FromPrimitive::from_i32(c as i32 - 7),
    }
}

/// Given a cell, returns the cell at its "south" position.
///
/// The south position is not defined for cells in the Rank 1,
/// so this function return an Option<Cell>, returning None if
/// the cell is in a position without a valid south neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(s(Cell::B2), Some(Cell::B1));
/// assert_eq!(s(Cell::F3), Some(Cell::F2));
/// assert_eq!(s(Cell::H7), Some(Cell::H6));
/// assert_eq!(s(Cell::F1), None);
/// ```
pub fn s(c: Cell) -> Option<Cell> {
    match rank(c) {
        Rank::Rank1 => None,
        _ => num::FromPrimitive::from_i32(c as i32 - 8),
    }
}

/// Given a cell, returns the cell at its "south-west" position.
///
/// The south-west position is not defined for cells in the A file,
/// or in the Rank 1, so this function return an Option<Cell>,
/// returning None if the cell is in a position without a valid
/// north-west neighbour cell.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(sw(Cell::B6), Some(Cell::A5));
/// assert_eq!(sw(Cell::E2), Some(Cell::D1));
/// assert_eq!(sw(Cell::G4), Some(Cell::F3));
/// assert_eq!(sw(Cell::A8), None);
/// assert_eq!(sw(Cell::H1), None);
/// ```
pub fn sw(c: Cell) -> Option<Cell> {
    match file(c) {
        File::FileA => None,
        _ => num::FromPrimitive::from_i32(c as i32 - 9),
    }
}

/// Computes the position of the [Cell] reached starting from `c`
/// and performing `step_north` steps towards north and `step_east` steps
/// towards east.
///
/// If `step_north` is negative the steps are done towards south,
/// if `step_east` is negative, the steps are done towards west.
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(calc_cell_after_steps(Cell::A1, 7, 0), Some(Cell::A8));
/// assert_eq!(calc_cell_after_steps(Cell::H6, 0, -7), Some(Cell::A6));
/// assert_eq!(calc_cell_after_steps(Cell::F2, 0, 0), Some(Cell::F2));
/// assert_eq!(calc_cell_after_steps(Cell::D4, 2, -1), Some(Cell::C6));
/// assert_eq!(calc_cell_after_steps(Cell::A4, -2, -1), None);
/// assert_eq!(calc_cell_after_steps(Cell::G7, 1, 2), None);
///
/// ```
pub fn calc_cell_after_steps(c: Cell, step_north: i32, step_east: i32) -> Option<Cell> {
    match (
        num::FromPrimitive::from_i32(rank(c) as i32 + step_north),
        num::FromPrimitive::from_i32(file(c) as i32 + step_east),
    ) {
        (Some(valid_rank), Some(valid_file)) => Some(to_cell(valid_file, valid_rank)),
        (_, _) => None,
    }
}

/// Computes the bitboard state with a single cell active
pub fn single_cell(c: Cell) -> BitBoardState {
    1_u64 << c as u64
}

/// Computes the bitboard state with the neighbour cells of a given cell active
pub fn neighbour(c: Cell) -> BitBoardState {
    let mut file_mask: BitBoardState = 0;
    let mut rank_mask: BitBoardState = 0;
    file_mask |= FILES_BBS[file(c) as usize];
    file_mask |= match west(c) {
        Some(w) => FILES_BBS[w as usize],
        _ => 0,
    };
    file_mask |= match east(c) {
        Some(e) => FILES_BBS[e as usize],
        _ => 0,
    };
    rank_mask |= RANKS_BBS[rank(c) as usize];
    rank_mask |= match north(c) {
        Some(n) => RANKS_BBS[n as usize],
        _ => 0,
    };
    rank_mask |= match south(c) {
        Some(s) => RANKS_BBS[s as usize],
        _ => 0,
    };
    file_mask & rank_mask ^ single_cell(c)
}

/// Computes the bitboard state with the cells of the file of a given cell active
pub fn file_mask(c: Cell) -> BitBoardState {
    FILES_BBS[file(c) as usize]
}

/// Computes the bitboard state with the cells of the rank a given cell active
pub fn rank_mask(c: Cell) -> BitBoardState {
    RANKS_BBS[rank(c) as usize]
}

/// Computes the bitboard state with the cells of the file and rank a given cell active
pub fn file_rank_mask(c: Cell) -> BitBoardState {
    FILES_BBS[file(c) as usize] | RANKS_BBS[rank(c) as usize]
}

/// Computes the bitboard state with the cells of the diagonal a given cell active
pub fn diag_mask(c: Cell) -> BitBoardState {
    DIAGS_BBS[diagonal(c) as usize]
}

/// Computes the bitboard state with the cells of the antidiagonal of a given cell active
pub fn antidiag_mask(c: Cell) -> BitBoardState {
    ANTIDIAGS_BBS[anti_diagonal(c) as usize]
}

/// Computes the bitboard state with the cells of both diagonals of a given cell active
pub fn diagonals_mask(c: Cell) -> BitBoardState {
    DIAGS_BBS[diagonal(c) as usize] | ANTIDIAGS_BBS[anti_diagonal(c) as usize]
}

/// Computes the bitboard state with the cells of file, rank and
/// diagonals of a given cell active (the "Queen" moves in the chess game)
pub fn queen_mask(c: Cell) -> BitBoardState {
    FILES_BBS[file(c) as usize]
        | RANKS_BBS[rank(c) as usize]
        | DIAGS_BBS[diagonal(c) as usize]
        | ANTIDIAGS_BBS[anti_diagonal(c) as usize]
}

// ----------------------------------------------------------------------------
// Functions and Traits implementation for File enum

/// Converts a File into its String representation ("a", ..., "h").
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(Into::<String>::into(File::FileA), "a");
/// ```
#[allow(clippy::from_over_into)]
impl Into<String> for File {
    fn into(self) -> String {
        format!("{}", (self as u8 + 97) as char)
    }
}

/// Tentatively convert a &str to a File.
///
/// For valid values ("a", "b", ..., "h") this function returns the
/// corresponding Ok(File), otherwise Err(AbbaDingoError::IllegalConversionToFile)
/// is returned.
///
/// # Example
/// ```
/// # use std::convert::TryFrom;
/// # use abbadingo::bbdefines::*;
/// # use abbadingo::error::AbbaDingoError;
/// assert_eq!(File::try_from("a"), Ok(File::FileA));
/// assert_eq!(File::try_from("x!"), Err(AbbaDingoError::IllegalConversionToFile));
/// ```
///
impl TryFrom<&str> for File {
    type Error = AbbaDingoError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" => Ok(File::FileA),
            "b" => Ok(File::FileB),
            "c" => Ok(File::FileC),
            "d" => Ok(File::FileD),
            "e" => Ok(File::FileE),
            "f" => Ok(File::FileF),
            "g" => Ok(File::FileG),
            "h" => Ok(File::FileH),
            _ => Err(AbbaDingoError::IllegalConversionToFile),
        }
    }
}

/// Display trait for the File enum.
///
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

// ----------------------------------------------------------------------------
// Functions and Traits implementation for Rank enum

/// Converts a Rank into its String representation ("1", ..., "8").
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(Into::<String>::into(Rank::Rank2), "2");
/// ```
#[allow(clippy::from_over_into)]
impl Into<String> for Rank {
    fn into(self) -> String {
        format!("{}", (self as u8 + 49) as char)
    }
}

/// Tentatively convert a &str to a Rank.
///
/// For valid values ("1", "2", ..., "8") this function returns the
/// corresponding Ok(Rank), otherwise Err(AbbaDingoError::IllegalConversionToRank)
/// is returned.
///
/// # Example
/// ```
/// # use std::convert::TryFrom;
/// # use abbadingo::bbdefines::*;
/// # use abbadingo::error::AbbaDingoError;
/// assert_eq!(Rank::try_from("3"), Ok(Rank::Rank3));
/// assert_eq!(Rank::try_from("0"), Err(AbbaDingoError::IllegalConversionToRank));
/// ```
///
impl TryFrom<&str> for Rank {
    type Error = AbbaDingoError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1" => Ok(Rank::Rank1),
            "2" => Ok(Rank::Rank2),
            "3" => Ok(Rank::Rank3),
            "4" => Ok(Rank::Rank4),
            "5" => Ok(Rank::Rank5),
            "6" => Ok(Rank::Rank6),
            "7" => Ok(Rank::Rank7),
            "8" => Ok(Rank::Rank8),
            _ => Err(AbbaDingoError::IllegalConversionToRank),
        }
    }
}

/// Display trait for the Rank enum.
///
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}

// ----------------------------------------------------------------------------
// Functions and Traits implementation for Cell enum

/// Converts a Cell into its String representation ("a1", ..., "h8").
///
/// # Example
/// ```
/// # use abbadingo::bbdefines::*;
/// assert_eq!(Into::<String>::into(Cell::E4), "e4");
/// ```
#[allow(clippy::from_over_into)]
impl Into<String> for Cell {
    fn into(self) -> String {
        format!("{}{}", file(self), rank(self))
    }
}

/// Tentatively convert a &str to a Cell.
///
/// For valid values ("a1", "a2", ..., "h8") this function returns the
/// corresponding Ok(Cell), otherwise Err(AbbaDingoError::IllegalConversionToCell)
/// is returned.
///
/// # Example
/// ```
/// # use std::convert::TryFrom;
/// # use abbadingo::bbdefines::*;
/// # use abbadingo::error::AbbaDingoError;
/// assert_eq!(Cell::try_from("g3"), Ok(Cell::G3));
/// assert_eq!(Cell::try_from("x0"), Err(AbbaDingoError::IllegalConversionToCell));
/// ```
///
impl TryFrom<&str> for Cell {
    type Error = AbbaDingoError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() != 2 {
            return Err(AbbaDingoError::IllegalConversionToCell);
        }
        let f = File::try_from(&value[0..1]);
        let r = Rank::try_from(&value[1..2]);
        match f {
            Ok(f) => match r {
                Ok(r) => Ok(to_cell(f, r)),
                _ => Err(AbbaDingoError::IllegalConversionToCell),
            },
            _ => Err(AbbaDingoError::IllegalConversionToCell),
        }
    }
}

/// Display trait for the Cell enum.
///
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
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

    // Tests for calc_cell_after_steps methods
    #[test]
    fn calc_cell_after_steps_test() {
        assert_eq!(calc_cell_after_steps(Cell::A1, 5, 0), Some(Cell::A6));
        assert_eq!(calc_cell_after_steps(Cell::A1, 0, 5), Some(Cell::F1));
        assert_eq!(calc_cell_after_steps(Cell::D4, 2, 1), Some(Cell::E6));
        assert_eq!(calc_cell_after_steps(Cell::D4, 1, 2), Some(Cell::F5));
        assert_eq!(calc_cell_after_steps(Cell::D4, -1, 2), Some(Cell::F3));
        assert_eq!(calc_cell_after_steps(Cell::D4, -2, 1), Some(Cell::E2));
        assert_eq!(calc_cell_after_steps(Cell::D4, -2, -1), Some(Cell::C2));
        assert_eq!(calc_cell_after_steps(Cell::D4, -1, -2), Some(Cell::B3));
        assert_eq!(calc_cell_after_steps(Cell::D4, 1, -2), Some(Cell::B5));
        assert_eq!(calc_cell_after_steps(Cell::D4, 2, -1), Some(Cell::C6));
        assert_eq!(calc_cell_after_steps(Cell::C2, 6, -2), Some(Cell::A8));
        assert_eq!(calc_cell_after_steps(Cell::B7, 1, 3), Some(Cell::E8));
        assert_eq!(calc_cell_after_steps(Cell::F5, 3, 2), Some(Cell::H8));
        assert_eq!(calc_cell_after_steps(Cell::D2, -1, -3), Some(Cell::A1));
        assert_eq!(calc_cell_after_steps(Cell::C2, 1, -3), None);
        assert_eq!(calc_cell_after_steps(Cell::B7, 2, -1), None);
        assert_eq!(calc_cell_after_steps(Cell::F5, 1, 3), None);
        assert_eq!(calc_cell_after_steps(Cell::D2, -2, -1), None);
    }

    // Test for the single_cell() method
    #[test]
    fn single_cell_mask_test() {
        assert_eq!(single_cell(Cell::A1), 1_u64);
        assert_eq!(single_cell(Cell::H8), 0x80_00_00_00_00_00_00_00_u64);
        assert_eq!(single_cell(Cell::E5), 0x00_00_00_10_00_00_00_00_u64);
    }

    // Test for the neighbour() method
    #[test]
    fn neighbour_mask_test() {
        assert_eq!(neighbour(Cell::D4), 0x00_00_00_1C_14_1C_00_00_u64);
        assert_eq!(neighbour(Cell::G1), 0x00_00_00_00_00_00_E0_A0_u64);
    }

    // Test for the file_mask() method
    #[test]
    fn file_mask_test() {
        assert_eq!(file_mask(Cell::A2), 0x01_01_01_01_01_01_01_01_u64);
        assert_eq!(file_mask(Cell::F5), 0x20_20_20_20_20_20_20_20_u64);
    }

    // Test for the rank_mask() method
    #[test]
    fn rank_mask_test() {
        assert_eq!(rank_mask(Cell::B2), 0x00_00_00_00_00_00_FF_00_u64);
        assert_eq!(rank_mask(Cell::H7), 0x00_FF_00_00_00_00_00_00_u64);
    }

    // Test for the file_rank_mask() method
    #[test]
    fn file_rank_mask_test() {
        assert_eq!(file_rank_mask(Cell::C6), 0x04_04_FF_04_04_04_04_04_u64);
        assert_eq!(file_rank_mask(Cell::G2), 0x40_40_40_40_40_40_FF_40_u64);
    }

    // Test for the diag_mask() method
    #[test]
    fn diag_mask_test() {
        assert_eq!(diag_mask(Cell::H8), 0x80_40_20_10_08_04_02_01_u64);
    }

    // Test for the antidiag_mask() method
    #[test]
    fn anti_diag_mask_test() {
        assert_eq!(antidiag_mask(Cell::B6), 0x00_01_02_04_08_10_20_40_u64);
    }

    // Test for the diagonals_mask() method
    #[test]
    fn diagonals_mask_test() {
        assert_eq!(diagonals_mask(Cell::C3), 0x80_40_20_11_0A_04_0A_11_u64);
    }

    // Test for the queen_mask() method
    #[test]
    fn queen_mask_test() {
        assert_eq!(queen_mask(Cell::D4), 0x88_49_2A_1C_FF_1C_2A_49_u64);
        assert_eq!(queen_mask(Cell::A8), 0xFF_03_05_09_11_21_41_81_u64);
    }

    // Conversion tests from String to File and from File to String
    #[test]
    fn try_from_string_to_file_tests() {
        assert_eq!(File::try_from("a"), Ok(File::FileA));
        assert_eq!(File::try_from("b"), Ok(File::FileB));
        assert_eq!(File::try_from("c"), Ok(File::FileC));
        assert_eq!(File::try_from("d"), Ok(File::FileD));
        assert_eq!(File::try_from("e"), Ok(File::FileE));
        assert_eq!(File::try_from("f"), Ok(File::FileF));
        assert_eq!(File::try_from("g"), Ok(File::FileG));
        assert_eq!(File::try_from("h"), Ok(File::FileH));
        assert_eq!(
            File::try_from("ah"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
        assert_eq!(
            File::try_from("i"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
        assert_eq!(
            File::try_from("B"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
        assert_eq!(
            File::try_from("à"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
        assert_eq!(
            File::try_from("是"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
        assert_eq!(
            File::try_from("1"),
            Err(AbbaDingoError::IllegalConversionToFile)
        );
    }
    #[test]
    fn file_into_string_tests() {
        let f: String = File::FileC.into();
        assert_eq!(f, "c");
    }

    // Conversion tests from String to Rank and from Rank to String
    #[test]
    fn try_from_string_to_rank_tests() {
        assert_eq!(Rank::try_from("1"), Ok(Rank::Rank1));
        assert_eq!(Rank::try_from("2"), Ok(Rank::Rank2));
        assert_eq!(Rank::try_from("3"), Ok(Rank::Rank3));
        assert_eq!(Rank::try_from("4"), Ok(Rank::Rank4));
        assert_eq!(Rank::try_from("5"), Ok(Rank::Rank5));
        assert_eq!(Rank::try_from("6"), Ok(Rank::Rank6));
        assert_eq!(Rank::try_from("7"), Ok(Rank::Rank7));
        assert_eq!(Rank::try_from("8"), Ok(Rank::Rank8));
        assert_eq!(
            Rank::try_from("9"),
            Err(AbbaDingoError::IllegalConversionToRank)
        );
        assert_eq!(
            Rank::try_from("B"),
            Err(AbbaDingoError::IllegalConversionToRank)
        );
        assert_eq!(
            Rank::try_from("à"),
            Err(AbbaDingoError::IllegalConversionToRank)
        );
        assert_eq!(
            Rank::try_from("是"),
            Err(AbbaDingoError::IllegalConversionToRank)
        );
        assert_eq!(
            Rank::try_from("a"),
            Err(AbbaDingoError::IllegalConversionToRank)
        );
    }
    #[test]
    fn rank_into_string_tests() {
        let r: String = Rank::Rank7.into();
        assert_eq!(r, "7");
    }

    // Conversion tests from String to Cell and from Cell to String
    #[test]
    fn try_from_string_to_cell_tests() {
        assert_eq!(Cell::try_from("g1"), Ok(Cell::G1));
        assert_eq!(Cell::try_from("a7"), Ok(Cell::A7));
        assert_eq!(Cell::try_from("b4"), Ok(Cell::B4));
        assert_eq!(Cell::try_from("c3"), Ok(Cell::C3));

        assert_eq!(
            Cell::try_from(""),
            Err(AbbaDingoError::IllegalConversionToCell)
        );
        assert_eq!(
            Cell::try_from("a"),
            Err(AbbaDingoError::IllegalConversionToCell)
        );
        assert_eq!(
            Cell::try_from("7"),
            Err(AbbaDingoError::IllegalConversionToCell)
        );
        assert_eq!(
            Cell::try_from("é"),
            Err(AbbaDingoError::IllegalConversionToCell)
        );
        assert_eq!(
            Cell::try_from("h22"),
            Err(AbbaDingoError::IllegalConversionToCell)
        );
    }
    #[test]
    fn cell_into_string_tests() {
        assert_eq!(Into::<String>::into(Cell::A2), "a2");
        assert_eq!(Into::<String>::into(Cell::B8), "b8");
        assert_eq!(Into::<String>::into(Cell::C1), "c1");
        assert_eq!(Into::<String>::into(Cell::D4), "d4");
        assert_eq!(Into::<String>::into(Cell::E3), "e3");
        assert_eq!(Into::<String>::into(Cell::F7), "f7");
        assert_eq!(Into::<String>::into(Cell::G5), "g5");
        assert_eq!(Into::<String>::into(Cell::H6), "h6");
    }

    // Display trait tests for File, Rank and Cells
    #[test]
    fn display_file_test() {
        assert_eq!(format!("{}", File::FileA), "a");
        assert_eq!(format!("{}", File::FileB), "b");
        assert_eq!(format!("{}", File::FileC), "c");
        assert_eq!(format!("{}", File::FileD), "d");
        assert_eq!(format!("{}", File::FileE), "e");
        assert_eq!(
            format!("{} {} {}", File::FileF, File::FileG, File::FileH),
            "f g h"
        );
    }
    #[test]
    fn display_rank_test() {
        assert_eq!(format!("{}", Rank::Rank1), "1");
        assert_eq!(format!("{}", Rank::Rank2), "2");
        assert_eq!(format!("{}", Rank::Rank3), "3");
        assert_eq!(format!("{}", Rank::Rank4), "4");
        assert_eq!(format!("{}", Rank::Rank5), "5");
        assert_eq!(
            format!("{} {} {}", Rank::Rank6, Rank::Rank7, Rank::Rank8),
            "6 7 8"
        );
    }
    #[test]
    fn display_cell_test() {
        assert_eq!(format!("{}", Cell::A1), "a1");
        assert_eq!(format!("{}", Cell::B2), "b2");
        assert_eq!(format!("{}", Cell::C3), "c3");
        assert_eq!(format!("{}", Cell::D4), "d4");
        assert_eq!(format!("{}", Cell::E5), "e5");
        assert_eq!(format!("{}", Cell::F6), "f6");
        assert_eq!(format!("{}", Cell::G7), "g7");
        assert_eq!(format!("{}", Cell::H8), "h8");
        assert_eq!(
            format!(
                "{}, {}, {}, {}, {}, {}, {}, {}",
                Cell::A8,
                Cell::B7,
                Cell::C6,
                Cell::D5,
                Cell::E4,
                Cell::F3,
                Cell::G2,
                Cell::H1
            ),
            "a8, b7, c6, d5, e4, f3, g2, h1"
        );
    }
}
