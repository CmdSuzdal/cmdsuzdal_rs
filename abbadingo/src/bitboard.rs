//! Definition of the [BitBoard] structure and related methods implementation.

use crate::bbdefines::{BitBoardState, Cell, File, Rank, EMPTY_STATE, FILES_BBS, RANKS_BBS};

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
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BitBoard {
    state: BitBoardState,
}
impl BitBoard {
    /// Default constructor for the [BitBoard] struct: instantiate an empty [BitBoard]
    pub fn new() -> BitBoard {
        BitBoard { state: EMPTY_STATE }
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
    /// let bb = BitBoard::from([Cell::E1]);
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
    /// let mut bb = BitBoard::from([Cell::A1, Cell::H8]);
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
        self.state &= !(1 << c as usize);
    }

    /// Sets all the cells of a [Rank] to busy state, i.e. set all the bits
    /// for the cells of the [Rank] to the value 1.
    ///
    pub fn set_rank(&mut self, r: Rank) {
        self.state |= RANKS_BBS[r as usize];
    }

    /// Resets all the cells of a [Rank] to free state, i.e. reset all the bits
    /// for the cells of the [Rank] to the value 0.
    ///
    pub fn reset_rank(&mut self, r: Rank) {
        self.state &= !(RANKS_BBS[r as usize]);
    }

    /// Sets all the cells of a [File] to busy state.
    ///
    pub fn set_file(&mut self, f: File) {
        self.state |= FILES_BBS[f as usize];
    }

    /// Resets all the cells of a [File] to free state.
    ///
    pub fn reset_file(&mut self, f: File) {
        self.state &= !(FILES_BBS[f as usize]);
    }

    /// Sets all the cells specified in a slice to busy state.
    ///
    pub fn set_cells(&mut self, cells: &[Cell]) {
        for c in cells {
            self.set_cell(*c);
        }
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
    /// let mut bb = BitBoard::from([Cell::B2, Cell::G7]);
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
    /// let mut bb = BitBoard::from([Cell::B2, Cell::G7]);
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
}

/// From trait for the BitBoard struct starting from a slice of [Cell]s.
///
/// Converts a slice of cells to a [BitBoard] with all the cells of the slice set to busy (1) status.
///
/// # Arguments
///
/// * `cells` - A [Cell] slice with the cells to be set on the [BitBoard]
/// # Example
///```
/// # use abbadingo::bitboard::*;
/// # use abbadingo::bbdefines::*;
/// let mut bb = BitBoard::new();
/// bb.set_file(File::FileD);
/// assert_eq!(bb, BitBoard::from([Cell::D1, Cell::D2, Cell::D3, Cell::D4,
///                                Cell::D5, Cell::D6, Cell::D7, Cell::D8]));
///```
// The following implementation is the only way I found to work with
// arrays, vectors and slices, but onestly I still do not understand
// it. See:
// https://www.reddit.com/r/rust/comments/70xqpw/using_the_from_trait_not_as_easy_as_i_thought/
impl<'a, T: AsRef<[Cell]>> From<T> for BitBoard {
    fn from(cells: T) -> Self {
        let mut bb = BitBoard::new();
        for c in cells.as_ref().to_vec() {
            bb.set_cell(c);
        }
        bb
    }
}
// ------------------------------------------------------------
// FIXME --- Seems impossible to add the From trait for a single
// Cell because conflicts with the From trait for Cell slices.
// i.e. adding the following code cause compilation failure:
//
// impl From<Cell> for BitBoard {
//     fn from(c: Cell) -> Self {
//         let mut bb = BitBoard::new();
//         bb.set_cell(c);
//         bb
//     }
// }
// Because of this, I was not able to add a method to create a
// BitBoard using a single cell. To do this using the From slice
// trait we have to do this (see tests):
//    let bb = BitBoard::from([Cell::H8]);
//
// but these does not work:
//    let bb = BitBoard::from(Cell::H8);
//    let bb = BitBoard::from(&Cell::H8);

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
    fn init_bitboard_using_a_vector_with_a_cell_in_h8() {
        let bb = BitBoard::from([Cell::H8]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, 0x80_00_00_00_00_00_00_00);
        assert_eq!(bb.pop_count(), 1);
    }
    #[test]
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_diagonal() {
        const BBS_DIAGONAL: BitBoardState = 0x80_40_20_10_08_04_02_01;
        let bb = BitBoard::from([
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
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_antidiagonal() {
        const BBS_ANTIDIAGONAL: BitBoardState = 0x01_02_04_08_10_20_40_80;
        let bb = BitBoard::from([
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
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_diagonal_and_reset_e5() {
        let mut bb = BitBoard::from([
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
        //     -------------------------
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
        //     -------------------------
        //     fa fb fc fd fe ff fg fh
        assert_eq!(bb.state, 0x08_80_41_22_14_89_14_2A);
        assert_eq!(bb.pop_count(), 16);
    }
}
