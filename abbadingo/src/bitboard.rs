const EMPTY_STATE: u64 = 0;

#[rustfmt::skip]
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
const FILES_BBS: [u64; 8] = [
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
const RANKS_BBS: [u64; 8] = [
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
    #[derive(Clone, Copy)]
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

#[derive(Default, Debug, PartialEq, Eq)]
pub struct BitBoard {
    state: u64,
}
impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard { state: EMPTY_STATE }
    }
    pub fn is_empty(&self) -> bool {
        self.state == EMPTY_STATE
    }
    pub fn set_cell(&mut self, c: Cell) {
        self.state |= 1 << c as usize;
    }
    pub fn reset_cell(&mut self, c: Cell) {
        self.state &= !(1 << c as usize);
    }
    pub fn set_rank(&mut self, r: Rank) {
        self.state |= RANKS_BBS[r as usize];
    }
    pub fn reset_rank(&mut self, r: Rank) {
        self.state &= !(RANKS_BBS[r as usize]);
    }
    pub fn set_file(&mut self, f: File) {
        self.state |= FILES_BBS[f as usize];
    }
    pub fn reset_file(&mut self, f: File) {
        self.state &= !(FILES_BBS[f as usize]);
    }
    pub fn set_cells(&mut self, cells: &[Cell]) {
        for c in cells {
            self.set_cell(*c);
        }
    }
    pub fn reset_cells(&mut self, cells: &[Cell]) {
        for c in cells {
            self.reset_cell(*c);
        }
    }
}

// ------------------------------------------------------------
// FIXME --- Seems impossible to add the From trait for a single
// Cell because conflicts with the next From trait for Cell slices.
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
// This implementation is the only way I found to work with
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
    }
    #[test]
    fn init_bitboard_using_a_vector_with_a_cell_in_h8() {
        let bb = BitBoard::from([Cell::H8]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, 0x80_00_00_00_00_00_00_00);
    }
    #[test]
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_diagonal() {
        const BBS_DIAGONAL: u64 = 0x80_40_20_10_08_04_02_01;
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
    }

    #[test]
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_antidiagonal() {
        const BBS_ANTIDIAGONAL: u64 = 0x01_02_04_08_10_20_40_80;
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
    }
}
