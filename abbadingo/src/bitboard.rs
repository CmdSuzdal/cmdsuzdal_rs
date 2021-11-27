//! Definition of the [BitBoard] structure and related methods implementation.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

// -----------------------------------------------------------------------------------
// ansi-term on crates.io
// This is a library for controlling colours and formatting,
// such as red bold text or blue underlined text, on ANSI terminals.
// Rustdoc: https://docs.rs/ansi_term
// Unicode box-drawing characters: https://en.wikipedia.org/wiki/Box-drawing_character
// Chess symbols on unicode: https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
use ansi_term::Colour::{Black, Fixed};
// -----------------------------------------------------------------------------------

use crate::num::FromPrimitive;
use std::fmt;

use crate::bbdefines::*;

/// Structure used to represent an 8x8 square board in a piece centric manner.
///
/// It is a general purpose, set-wise data-structure fitting in one 64-bit register.
/// Each bit represent the "status" of a [Cell] inside the board. For example, a bitboard
/// can represent occupation of a cell by a piece, but also more abstract things like
/// attack and defend sets, move-target sets and so on.
///
/// See the [Bitboard entry page](https://www.chessprogramming.org/Bitboards)
/// in the chess programming wiki for additional details.
///
/// The BitBoard is composed by 8 horizontal rows numbered from 1 to 8, each row called "[Rank]",
/// and 8 vertical columns numbered from A to H, with each column called "[File]".
/// The cells inside the bitboard are identified by the [File]+[Rank] combination. For example
/// the bottom left cell is "A1", and the top right cell is "H8". Each [Cell] is indexed using
/// a number starting from A1 = 0 to H8= 63.
///
/// |            | File A | File B | File C | File D | File E | File F | File G | File H |
/// |         --:|   :-:  |   :-:  |   :-:  |   :-:  |   :-:  |   :-:  |   :-:  |   :-:  |
/// | **Rank 8** | A8 = 56| B8     | C8     | D8     | E8     | F8     | G8     | H8 = 63|
/// | **Rank 7** | A7 = 48| B7     | C7     | D7     | E7     | F7     | G7     | H7     |
/// | **Rank 6** | A6 = 40| B6     | C6     | D6     | E6     | F6     | G6     | H6     |
/// | **Rank 5** | A5 = 32| B5     | C5     | D5     | E5     | F5     | G5     | H5     |
/// | **Rank 4** | A4 = 24| B4     | C4     | D4     | E4     | F4     | G4     | H4     |
/// | **Rank 3** | A3 = 16| B3     | C3     | D3     | E3     | F3     | G3     | H3     |
/// | **Rank 2** | A2 = 8 | B2     | C2     | D2     | E2     | F2     | G2     | H2 = 15|
/// | **Rank 1** | A1 = 0 | B1 = 1 | C1 = 2 | D1 = ..| E1     | F1     | G1     | H1 = 7 |
///
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct BitBoard {
    pub state: BitBoardState,
}
impl BitBoard {
    /// Default constructor for the [BitBoard] struct: instantiate an empty [BitBoard]
    pub fn new() -> BitBoard {
        BitBoard { state: EMPTY_STATE }
    }

    /// [BitBoard] constructor starting from an array of cells.
    ///
    /// The cells in the slice, will be set to active state.
    ///
    /// # Arguments
    ///
    /// * `cells` - A [Cell] slice with the cells to be set on the [BitBoard]
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    /// let mut bb = BitBoard::new();
    /// bb.set_cell(Cell::A1);
    /// bb.set_cell(Cell::A2);
    /// bb.set_cell(Cell::H8);
    /// assert_eq!(bb, BitBoard::from_cells(&[Cell::A1, Cell::A2, Cell::H8]));
    /// ```
    pub fn from_cells(cells: &[Cell]) -> BitBoard {
        let mut bb = BitBoard::new();
        for c in cells {
            bb.set_cell(*c);
        }
        bb
    }

    /// Returns `true` if the [BitBoard] is empty.
    ///
    /// A [Cell] inside a [BitBoard] can be free (or empty) or busy.
    /// A  [BitBoard] is empty if all its cells are empty.
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// // The default constructor returns an empty bitboard:
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  .  . |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  .  .  .  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(BitBoard::new().is_empty(), true);
    ///
    /// // Builds a BitBoard with the E1 cell busy
    /// let bb = BitBoard::from_cells(&[Cell::E1]);
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  .  . |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  .  .  .  .  o  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(bb.is_empty(), false);
    ///```
    pub fn is_empty(&self) -> bool {
        self.state == EMPTY_STATE
    }

    /// Sets a [BitBoard] [Cell] to busy state, i.e. set the [Cell] bit to 1.
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::new();
    /// bb.set_cell(Cell::E5);
    /// assert_eq!(bb.is_empty(), false);
    ///```
    ///
    pub fn set_cell(&mut self, c: Cell) {
        self.state |= 1 << c as usize;
    }

    /// Resets a [BitBoard] [Cell] to free state, i.e. reset the [Cell] bit to 0.
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::from_cells(&[Cell::A1, Cell::H8]);
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  .  o |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  o  .  .  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(bb.is_empty(), false);
    /// bb.reset_cell(Cell::A1);
    /// bb.reset_cell(Cell::H8);
    /// assert_eq!(bb.is_empty(), true);
    ///```
    ///
    pub fn reset_cell(&mut self, c: Cell) {
        *self &= BitBoard::from(!(1 << c as usize));
    }

    /// Sets all the cells of a [Rank] to busy state, i.e. set all the bits
    /// for the cells of the [Rank] to the value 1.
    ///
    pub fn set_rank(&mut self, r: Rank) {
        *self |= BitBoard::from(RANKS_BBS[r as usize]);
    }

    /// Resets all the cells of a [Rank] to free state, i.e. reset all the bits
    /// for the cells of the [Rank] to the value 0.
    ///
    pub fn reset_rank(&mut self, r: Rank) {
        *self &= BitBoard::from(!(RANKS_BBS[r as usize]));
    }

    /// Sets all the cells of a [File] to busy state.
    ///
    pub fn set_file(&mut self, f: File) {
        *self |= BitBoard::from(FILES_BBS[f as usize]);
    }

    /// Resets all the cells of a [File] to free state.
    ///
    pub fn reset_file(&mut self, f: File) {
        *self &= BitBoard::from(!(FILES_BBS[f as usize]));
    }

    /// Sets all the cells of a [Diagonal] to busy state.
    ///
    pub fn set_diagonal(&mut self, d: Diagonal) {
        *self |= BitBoard::from(DIAGS_BBS[d as usize]);
    }

    /// Sets all the cells of an [AntiDiagonal] to busy state.
    ///
    pub fn set_antidiagonal(&mut self, d: AntiDiagonal) {
        *self |= BitBoard::from(ANTIDIAGS_BBS[d as usize]);
    }

    /// Sets all the cells specified in a slice to busy state.
    ///
    pub fn set_cells(&mut self, cells: &[Cell]) {
        for c in cells {
            self.set_cell(*c);
        }
    }

    /// Returns true if the given [Cell] in the [BitBoard] is in active state
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::new();
    /// bb.set_file(File::FileG);
    /// bb.set_rank(Rank::Rank3);
    /// assert!(bb.cell_is_active(Cell::G3));
    /// assert!(!bb.cell_is_active(Cell::A1));
    ///
    /// ```
    pub fn cell_is_active(&self, c: Cell) -> bool {
        self.state & (1 << c as usize) != 0
    }

    /// Set the Cell at the crossing of the given File and Cell to the active state
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::new();
    /// bb.set_cell_from_file_and_rank(File::FileD, Rank::Rank3);
    /// bb.set_cell_from_file_and_rank(File::FileG, Rank::Rank8);
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  o  . |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  o  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  .  .  .  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(bb.pop_count(), 2);
    /// ```
    pub fn set_cell_from_file_and_rank(&mut self, f: File, r: Rank) {
        *self |= BitBoard::from(single_cell(to_cell(f, r)));
    }

    /// Reset the Cell at the crossing of the given File and Cell to the inactive state
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::new();
    /// bb.set_file(File::FileC);
    /// bb.set_rank(Rank::Rank4);
    /// bb.reset_cell_from_file_and_rank(File::FileC, Rank::Rank4);
    /// //    _________________________
    /// // r8|  .  .  o  .  .  .  .  . |
    /// // r7|  .  .  o  .  .  .  .  . |
    /// // r6|  .  .  o  .  .  .  .  . |
    /// // r5|  .  .  o  .  .  .  .  . |
    /// // r4|  o  o  .  o  o  o  o  o |
    /// // r3|  .  .  o  .  .  .  .  . |
    /// // r2|  .  .  o  .  .  .  .  . |
    /// // r1|  .  .  o  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(bb.pop_count(), 14);
    /// ```
    pub fn reset_cell_from_file_and_rank(&mut self, f: File, r: Rank) {
        *self &= BitBoard::from(!single_cell(to_cell(f, r)));
    }

    /// Resets all the cells specified in a slice to free state.
    ///
    pub fn reset_cells(&mut self, cells: &[Cell]) {
        for c in cells {
            self.reset_cell(*c);
        }
    }

    /// Evaluate the number of active (busy) cells in a [BitBoard]
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::from_cells(&[Cell::B2, Cell::G7]);
    /// bb.set_file(File::FileD);
    /// //    _________________________
    /// // r8|  .  .  .  o  .  .  .  . |
    /// // r7|  .  .  .  o  .  .  o  . |
    /// // r6|  .  .  .  o  .  .  .  . |
    /// // r5|  .  .  .  o  .  .  .  . |
    /// // r4|  .  .  .  o  .  .  .  . |
    /// // r3|  .  .  .  o  .  .  .  . |
    /// // r2|  .  o  .  o  .  .  .  . |
    /// // r1|  .  .  .  o  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// assert_eq!(bb.pop_count(), 10);
    ///```
    ///
    pub fn pop_count(&self) -> usize {
        let mut cnt = 0;
        let mut bbs = self.state;
        while bbs != 0 {
            cnt += 1;
            bbs &= bbs - 1; // Reset LS1B
        }
        cnt
    }

    /// Clear the [BitBoard].
    ///
    /// After the call to this method all the Cell are set to free status.
    /// (i.e. the board is empty).
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::from_cells(&[Cell::B2, Cell::G7]);
    /// // BitBoard is not empty:
    /// assert!(!bb.is_empty());
    /// // Clears the BitBoard:
    /// bb.clear();
    /// // Now the BitBoard is empty:
    /// assert!(bb.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.state = EMPTY_STATE;
    }

    /// Given a [BitBoard] with a single active cell, returns the active [Cell],
    /// otherwise returns `None` if [BitBoard] has no active cells or more than
    /// one active [Cell].
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bitboard::*;
    /// # use abbadingo::bbdefines::*;
    ///
    /// let mut bb = BitBoard::from_cells(&[Cell::H3]);
    /// assert_eq!(bb.active_cell(), Some(Cell::H3));
    /// bb.set_cell(Cell::C7);
    /// // More that one cell active...
    /// assert_eq!(bb.active_cell(), None);
    /// ```
    ///
    pub fn active_cell(&self) -> Option<Cell> {
        match self.pop_count() {
            1 => {
                // Here we skip a lot of controls, because we are sure
                // to find a single cell active in the bitboard
                for ndx in 0..NUM_CELLS {
                    if self.cell_is_active(Cell::from_usize(ndx)?) {
                        return Cell::from_usize(ndx);
                    }
                }
                // No active cells found in the BitBoard with pop_count() 1...
                // This should be impossible... Maybe we should panic??
                None
            }
            _ => {
                // If there is more that one active cell in the BitBoard returns None
                None
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Traits implementation for BitBoard structure

/// From trait for the BitBoard struct starting from a BitBoardState.
///
/// Set the internal state of the BitBoard to the specified u64 value interpretated as a bitmask.
/// Each bit set to 1 in the u64 value is set to active in the bitboard
///
/// # Arguments
///
/// * `mask` - The bitmask to use to set the cells in the [BitBoard].
///
/// # Example
/// ```
/// # use abbadingo::bitboard::*;
/// # use abbadingo::bbdefines::*;
/// let mut bb = BitBoard::from(0xFF_00_00_00_00_00_00_FF);
/// assert_eq!(bb, BitBoard::from_cells(&[Cell::A8, Cell::B8, Cell::C8, Cell::D8,
///                                       Cell::E8, Cell::F8, Cell::G8, Cell::H8,
///                                       Cell::A1, Cell::B1, Cell::C1, Cell::D1,
///                                       Cell::E1, Cell::F1, Cell::G1, Cell::H1]));
/// ```
impl From<BitBoardState> for BitBoard {
    fn from(mask: BitBoardState) -> Self {
        let mut bb = BitBoard::new();
        bb.state = mask;
        bb
    }
}

/// Display trait for [BitBoard] structure.
///
/// Represent a bitboard in "ascii" form.
///
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bg_style = Black.on(Fixed(252));
        let mut bb_str: String = "\n".to_string();
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("                                       ")
        ));
        bb_str.push_str(&"\n".to_string());
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("     a   b   c   d   e   f   g   h     ")
        ));
        bb_str.push_str(&"\n".to_string());
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("   ╭───┬───┬───┬───┬───┬───┬───┬───╮   ")
        ));
        for r in (0..8).rev() {
            bb_str.push_str(&"\n".to_string());
            bb_str.push_str(&format!("{}", bg_style.paint(" ")));
            bb_str.push_str(&format!("{}", bg_style.paint((r + 1).to_string())));
            bb_str.push_str(&format!("{}", bg_style.paint(" │ ")));

            //bb_str.push_str(&format!("\n {} │", r + 1));
            for c in 0..8 {
                if self.cell_is_active(to_cell(
                    num::FromPrimitive::from_i32(c).unwrap(),
                    num::FromPrimitive::from_i32(r).unwrap(),
                )) {
                    bb_str.push_str(&format!("{}", bg_style.paint("♟︎ ")));
                } else {
                    bb_str.push_str(&format!("{}", bg_style.paint("  ")));
                }
                bb_str.push_str(&format!("{}", bg_style.paint("│ ")));
            }
            bb_str.push_str(&format!("{}", bg_style.paint((r + 1).to_string())));
            bb_str.push_str(&format!("{}", bg_style.paint(" ")));
            if r > 0 {
                bb_str.push_str(&"\n".to_string());
                bb_str.push_str(&format!(
                    "{}",
                    bg_style.paint("   ├───┼───┼───┼───┼───┼───┼───┼───┤   ")
                ));
            }
        }
        bb_str.push_str(&"\n".to_string());
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("   ╰───┴───┴───┴───┴───┴───┴───┴───╯   ")
        ));
        bb_str.push_str(&"\n".to_string());
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("     a   b   c   d   e   f   g   h     ")
        ));
        bb_str.push_str(&"\n".to_string());
        bb_str.push_str(&format!(
            "{}",
            bg_style.paint("                                       ")
        ));
        bb_str.push_str(&"\n".to_string());
        write!(f, "{}", bb_str)
    }
}

// ----------------------------------------------------------------------------
// Operators overloading
impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: Self) -> Self {
        BitBoard {
            state: self.state | rhs.state,
        }
    }
}
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.state |= rhs.state;
    }
}
impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: Self) -> Self {
        BitBoard {
            state: self.state & rhs.state,
        }
    }
}
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.state &= rhs.state;
    }
}
impl BitXor for BitBoard {
    type Output = BitBoard;
    fn bitxor(self, rhs: Self) -> Self {
        BitBoard {
            state: self.state ^ rhs.state,
        }
    }
}
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.state ^= rhs.state;
    }
}

// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn by_default_a_new_bitboard_is_empty() {
        let bb = BitBoard::new();
        assert_eq!(bb.state, EMPTY_STATE);
        assert_eq!(bb.is_empty(), true);
        assert_eq!(bb.pop_count(), 0);
    }
    #[test]
    fn create_bitboard_using_a_vector_with_a_cell_in_h8() {
        let bb = BitBoard::from_cells(&[Cell::H8]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, 0x80_00_00_00_00_00_00_00);
        assert_eq!(bb.pop_count(), 1);
    }
    #[test]
    fn create_bitboard_using_a_cells_vector_with_active_cell_in_diagonal() {
        const BBS_DIAGONAL: BitBoardState = 0x80_40_20_10_08_04_02_01;
        let bb = BitBoard::from_cells(&[
            Cell::A1,
            Cell::B2,
            Cell::C3,
            Cell::D4,
            Cell::E5,
            Cell::F6,
            Cell::G7,
            Cell::H8,
        ]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, BBS_DIAGONAL);
        assert_eq!(bb.pop_count(), 8);
    }

    #[test]
    fn create_bitboard_using_a_cells_vector_with_active_cell_in_antidiagonal() {
        const BBS_ANTIDIAGONAL: BitBoardState = 0x01_02_04_08_10_20_40_80;
        let bb = BitBoard::from_cells(&[
            Cell::A8,
            Cell::B7,
            Cell::C6,
            Cell::D5,
            Cell::E4,
            Cell::F3,
            Cell::G2,
            Cell::H1,
        ]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, BBS_ANTIDIAGONAL);
        assert_eq!(bb.pop_count(), 8);
    }

    #[test]
    fn create_bitboard_using_a_cells_vector_with_active_cell_in_diagonal_and_reset_e5() {
        let mut bb = BitBoard::from_cells(&[
            Cell::A1,
            Cell::B2,
            Cell::C3,
            Cell::D4,
            Cell::E5,
            Cell::F6,
            Cell::G7,
            Cell::H8,
        ]);
        bb.reset_cell(Cell::E5);
        assert_eq!(bb.state, 0x80_40_20_00_08_04_02_01);
        assert_eq!(bb.pop_count(), 7);
    }

    #[test]
    fn set_even_ranks_in_bitboard() {
        let mut bb = BitBoard::new();
        bb.set_rank(Rank::Rank2);
        bb.set_rank(Rank::Rank4);
        bb.set_rank(Rank::Rank6);
        bb.set_rank(Rank::Rank8);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(
            bb.state,
            RANKS_BBS[Rank::Rank2 as usize]
                | RANKS_BBS[Rank::Rank4 as usize]
                | RANKS_BBS[Rank::Rank6 as usize]
                | RANKS_BBS[Rank::Rank8 as usize]
        );
        assert_eq!(bb.state, 0xFF_00_FF_00_FF_00_FF_00);
        assert_eq!(bb.pop_count(), 32);
    }

    #[test]
    fn set_odd_files_in_bitboard() {
        let mut bb = BitBoard::new();
        bb.set_file(File::FileA);
        bb.set_file(File::FileC);
        bb.set_file(File::FileE);
        bb.set_file(File::FileG);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(
            bb.state,
            FILES_BBS[File::FileA as usize]
                | FILES_BBS[File::FileC as usize]
                | FILES_BBS[File::FileE as usize]
                | FILES_BBS[File::FileG as usize]
        );
        assert_eq!(bb.state, 0x55_55_55_55_55_55_55_55);
        assert_eq!(bb.pop_count(), 32);
    }

    #[test]
    fn set_cells_in_bitboard() {
        let mut bb = BitBoard::new();
        bb.set_cells(&[
            Cell::D1,
            Cell::D2,
            Cell::D3,
            Cell::D4,
            Cell::D5,
            Cell::D6,
            Cell::D7,
            Cell::D8,
            Cell::A3,
            Cell::B3,
            Cell::C3,
            Cell::E3,
            Cell::F3,
            Cell::G3,
            Cell::H3,
        ]);
        //    _________________________
        // r8|  .  .  .  o  .  .  .  . |
        // r7|  .  .  .  o  .  .  .  . |
        // r6|  .  .  .  o  .  .  .  . |
        // r5|  .  .  .  o  .  .  .  . |
        // r4|  .  .  .  o  .  .  .  . |
        // r3|  o  o  o  o  o  o  o  o |
        // r2|  .  .  .  o  .  .  .  . |
        // r1|  .  .  .  o  .  .  .  . |
        //     -------------------------
        //     fa fb fc fd fe ff fg fh
        assert_eq!(bb.state, 0x08_08_08_08_08_FF_08_08);
        assert_eq!(bb.pop_count(), 15);

        bb.set_cells(&[
            Cell::B1,
            Cell::C2,
            Cell::E4,
            Cell::F5,
            Cell::G6,
            Cell::H7,
            Cell::F1,
            Cell::E2,
            Cell::C4,
            Cell::B5,
            Cell::A6,
        ]);
        //    _________________________
        // r8|  .  .  .  o  .  .  .  . |
        // r7|  .  .  .  o  .  .  .  x |
        // r6|  x  .  .  o  .  .  x  . |
        // r5|  .  x  .  o  .  x  .  . |
        // r4|  .  .  x  o  x  .  .  . |
        // r3|  o  o  o  o  o  o  o  o |
        // r2|  .  .  x  o  x  .  .  . |
        // r1|  .  x  .  o  .  x  .  . |
        //    -------------------------
        //     fa fb fc fd fe ff fg fh
        assert_eq!(bb.state, 0x08_88_49_2A_1C_FF_1C_2A);
        assert_eq!(bb.pop_count(), 26);

        bb.reset_cells(&[
            Cell::D2,
            Cell::D4,
            Cell::D5,
            Cell::D6,
            Cell::D7,
            Cell::B3,
            Cell::C3,
            Cell::E3,
            Cell::F3,
            Cell::G3,
        ]);
        //    _________________________
        // r8|  .  .  .  o  .  .  .  . |
        // r7|  .  .  .  .  .  .  .  x |
        // r6|  x  .  .  .  .  .  x  . |
        // r5|  .  x  .  .  .  x  .  . |
        // r4|  .  .  x  .  x  .  .  . |
        // r3|  o  .  .  o  .  .  .  o |
        // r2|  .  .  x  .  x  .  .  . |
        // r1|  .  x  .  o  .  x  .  . |
        //    -------------------------
        //     fa fb fc fd fe ff fg fh
        assert_eq!(bb.state, 0x08_80_41_22_14_89_14_2A);
        assert_eq!(bb.pop_count(), 16);
    }

    // Test on cell_is_active() function
    #[test]
    fn check_active_cells_when_the_whole_rank_2_is_active() {
        let mut bb = BitBoard::new();
        bb.set_rank(Rank::Rank2);
        assert!(bb.cell_is_active(Cell::A2));
        assert!(bb.cell_is_active(Cell::B2));
        assert!(bb.cell_is_active(Cell::C2));
        assert!(bb.cell_is_active(Cell::D2));
        assert!(bb.cell_is_active(Cell::E2));
        assert!(bb.cell_is_active(Cell::F2));
        assert!(bb.cell_is_active(Cell::G2));
        assert!(bb.cell_is_active(Cell::H2));
        assert!(!bb.cell_is_active(Cell::A1));
        assert!(!bb.cell_is_active(Cell::G3));
    }
    #[test]
    fn check_active_cells_when_the_whole_file_d_is_active() {
        let mut bb = BitBoard::new();
        bb.set_file(File::FileC);
        assert!(bb.cell_is_active(Cell::C1));
        assert!(bb.cell_is_active(Cell::C2));
        assert!(bb.cell_is_active(Cell::C3));
        assert!(bb.cell_is_active(Cell::C4));
        assert!(bb.cell_is_active(Cell::C5));
        assert!(bb.cell_is_active(Cell::C6));
        assert!(bb.cell_is_active(Cell::C7));
        assert!(bb.cell_is_active(Cell::C8));
        assert!(!bb.cell_is_active(Cell::B3));
        assert!(!bb.cell_is_active(Cell::D5));
    }
    #[test]
    fn check_active_cells_when_the_whole_diagonal_10_is_active() {
        let mut bb = BitBoard::new();
        bb.set_diagonal(Diagonal::Diag10);
        assert!(bb.cell_is_active(Cell::D1));
        assert!(bb.cell_is_active(Cell::E2));
        assert!(bb.cell_is_active(Cell::F3));
        assert!(bb.cell_is_active(Cell::G4));
        assert!(bb.cell_is_active(Cell::H5));
        assert!(!bb.cell_is_active(Cell::E1));
        assert!(!bb.cell_is_active(Cell::G6));
    }
    #[test]
    fn check_active_cells_when_the_whole_antidiagonal_5_is_active() {
        let mut bb = BitBoard::new();
        bb.set_antidiagonal(AntiDiagonal::AntiDiag5);
        assert!(bb.cell_is_active(Cell::F1));
        assert!(bb.cell_is_active(Cell::E2));
        assert!(bb.cell_is_active(Cell::D3));
        assert!(bb.cell_is_active(Cell::C4));
        assert!(bb.cell_is_active(Cell::B5));
        assert!(bb.cell_is_active(Cell::A6));
        assert!(!bb.cell_is_active(Cell::G2));
        assert!(!bb.cell_is_active(Cell::B6));
    }

    #[test]
    fn test_bitor_operators() {
        let mut bb1 = BitBoard::from_cells(&[Cell::A1, Cell::H8]);
        let bb2 = BitBoard::from_cells(&[Cell::A8, Cell::H1]);
        let bb3 = BitBoard::from_cells(&[Cell::D4, Cell::E5]);
        bb1 = bb1 | bb2;
        assert_eq!(
            bb1,
            BitBoard::from_cells(&[Cell::A1, Cell::A8, Cell::H1, Cell::H8])
        );
        bb1 |= bb3;
        assert_eq!(
            bb1,
            BitBoard::from_cells(&[Cell::A1, Cell::A8, Cell::D4, Cell::E5, Cell::H1, Cell::H8])
        );
    }
    #[test]
    fn test_bitand_operators() {
        let mut bb1 = BitBoard::from(0xFF_FF_FF_FF_FF_FF_FF_FF);
        let bb2 = BitBoard::from(0xFF_FF_FF_FF_00_00_00_00);
        let bb3 = BitBoard::from(0x33_33_33_33_33_33_33_33);
        bb1 = bb1 & bb2;
        assert_eq!(bb1, BitBoard::from(0xFF_FF_FF_FF_00_00_00_00));
        bb1 &= bb3;
        assert_eq!(bb1, BitBoard::from(0x33_33_33_33_00_00_00_00));
    }
}
