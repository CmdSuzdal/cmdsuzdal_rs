//! Definition of the [ChessArmy] structure and related methods implementation.
//!

use crate::bbdefines::*;
use crate::bitboard::BitBoard;
use crate::chessdefines::*;

/// Structure used to represent a Chess Army.
///
/// A Chess Army is a group of chess pieces of the same colour placed on a Chess Board.
/// It is represented by an [ArmyColour] and by a set of [BitBoard]s, one for each Piece type.
///
pub struct ChessArmy {
    //pieces: HashMap<ChessPiece, BitBoard>,
    pieces: [BitBoard; NUM_PIECES_TYPES],
    colour: ArmyColour,
}

impl ChessArmy {
    /// Default constructor for the [ChessArmy] struct: instantiate a [ChessArmy] of the given colour
    ///
    /// # Arguments
    ///
    /// * `c` - The colour of the [ChessArmy].
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let white_army = ChessArmy::new(ArmyColour::White);
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  .  . |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  P  P  P  P  P  P  P  P |
    /// // r1|  R  N  B  Q  K  B  N  R |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    ///
    /// let black_army = ChessArmy::new(ArmyColour::Black);
    /// //    _________________________
    /// // r8|  r  n  b  q  k  b  n  r |
    /// // r7|  p  p  p  p  p  p  p  p |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  .  .  .  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    pub fn new(c: ArmyColour) -> ChessArmy {
        let mut a = ChessArmy {
            pieces: [BitBoard::new(); NUM_PIECES_TYPES],
            colour: c,
        };
        a.reset(c);
        a
    }

    /// Initialize an [ChessArmy] of the specified colour with the initial standard chess deployment.
    ///
    /// Can be used to reset a [ChessArmy] to initial state
    ///
    /// # Arguments
    ///
    /// * `c` - The colour of the new arrangement of the [ChessArmy].
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::new(ArmyColour::White);
    /// //    _________________________
    /// // r8|  .  .  .  .  .  .  .  . |
    /// // r7|  .  .  .  .  .  .  .  . |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  P  P  P  P  P  P  P  P |
    /// // r1|  R  N  B  Q  K  B  N  R |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    ///
    /// army.reset(ArmyColour::Black);
    /// //    _________________________
    /// // r8|  r  n  b  q  k  b  n  r |
    /// // r7|  p  p  p  p  p  p  p  p |
    /// // r6|  .  .  .  .  .  .  .  . |
    /// // r5|  .  .  .  .  .  .  .  . |
    /// // r4|  .  .  .  .  .  .  .  . |
    /// // r3|  .  .  .  .  .  .  .  . |
    /// // r2|  .  .  .  .  .  .  .  . |
    /// // r1|  .  .  .  .  .  .  .  . |
    /// //     -------------------------
    /// //     fa fb fc fd fe ff fg fh
    /// ```
    pub fn reset(&mut self, c: ArmyColour) {
        self.colour = c;
        match c {
            ArmyColour::White => {
                self.pieces[ChessPiece::King as usize] = BitBoard::from_cells(&[Cell::E1]);
                self.pieces[ChessPiece::Queen as usize] = BitBoard::from_cells(&[Cell::D1]);
                self.pieces[ChessPiece::Bishop as usize] =
                    BitBoard::from_cells(&[Cell::C1, Cell::F1]);
                self.pieces[ChessPiece::Knight as usize] =
                    BitBoard::from_cells(&[Cell::B1, Cell::G1]);
                self.pieces[ChessPiece::Rook as usize] =
                    BitBoard::from_cells(&[Cell::A1, Cell::H1]);
                self.pieces[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces[ChessPiece::Pawn as usize].set_rank(Rank::Rank2);
            }
            ArmyColour::Black => {
                self.pieces[ChessPiece::King as usize] = BitBoard::from_cells(&[Cell::E8]);
                self.pieces[ChessPiece::Queen as usize] = BitBoard::from_cells(&[Cell::D8]);
                self.pieces[ChessPiece::Bishop as usize] =
                    BitBoard::from_cells(&[Cell::C8, Cell::F8]);
                self.pieces[ChessPiece::Knight as usize] =
                    BitBoard::from_cells(&[Cell::B8, Cell::G8]);
                self.pieces[ChessPiece::Rook as usize] =
                    BitBoard::from_cells(&[Cell::A8, Cell::H8]);
                self.pieces[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces[ChessPiece::Pawn as usize].set_rank(Rank::Rank7);
            }
        }
    }

    /// Returns the number of Pieces (including pawn) of a [ChessArmy].
    ///
    /// # Example
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::new(ArmyColour::White);
    /// assert_eq!(army.num_pieces(), 16);
    /// ```
    pub fn num_pieces(&self) -> usize {
        self.pieces[ChessPiece::King as usize].pop_count()
            + self.pieces[ChessPiece::Queen as usize].pop_count()
            + self.pieces[ChessPiece::Bishop as usize].pop_count()
            + self.pieces[ChessPiece::Knight as usize].pop_count()
            + self.pieces[ChessPiece::Rook as usize].pop_count()
            + self.pieces[ChessPiece::Pawn as usize].pop_count()
    }

    /// Returns the [BitBoardState] mask of the cell occupied by army pieces (including pawn).
    ///
    /// # Example
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::new(ArmyColour::Black);
    /// assert_eq!(army.occupied_cells(), 0xFF_FF_00_00_00_00_00_00);
    /// ```
    pub fn occupied_cells(&self) -> BitBoardState {
        self.pieces[ChessPiece::King as usize].state
            | self.pieces[ChessPiece::Queen as usize].state
            | self.pieces[ChessPiece::Pawn as usize].state
            | self.pieces[ChessPiece::Bishop as usize].state
            | self.pieces[ChessPiece::Knight as usize].state
            | self.pieces[ChessPiece::Rook as usize].state
    }

    /// Returns the [ChessPiece] occupying the given [Cell] if one,
    /// or `None` if the [Cell] is free.
    ///
    /// # Arguments
    ///
    /// * `c` - The [Cell] to check.
    ///
    /// # Example
    /// ```
    /// # use abbadingo::bbdefines::{Cell};
    /// # use abbadingo::chessdefines::{ArmyColour, ChessPiece};
    /// # use abbadingo::chessarmy::{ChessArmy};
    /// let army = ChessArmy::new(ArmyColour::Black);
    /// assert_eq!(army.get_piece_in_cell(Cell::C8), Some(ChessPiece::Bishop));
    /// assert_eq!(army.get_piece_in_cell(Cell::C1), None);
    /// ```
    ///
    pub fn get_piece_in_cell(&self, c: Cell) -> Option<ChessPiece> {
        if self.pieces[ChessPiece::King as usize].cell_is_active(c) {
            Some(ChessPiece::King)
        } else if self.pieces[ChessPiece::Queen as usize].cell_is_active(c) {
            Some(ChessPiece::Queen)
        } else if self.pieces[ChessPiece::Bishop as usize].cell_is_active(c) {
            Some(ChessPiece::Bishop)
        } else if self.pieces[ChessPiece::Knight as usize].cell_is_active(c) {
            Some(ChessPiece::Knight)
        } else if self.pieces[ChessPiece::Rook as usize].cell_is_active(c) {
            Some(ChessPiece::Rook)
        } else if self.pieces[ChessPiece::Pawn as usize].cell_is_active(c) {
            Some(ChessPiece::Pawn)
        } else {
            None
        }
    }


    // ---------------------------------------------------------------------------
    // PRIVATE METHODS
    // ---------------------------------------------------------------------------

    /// Returns the [Cell] with the position of the King.
    ///
    /// This is the only "get_position" function that makes sense because
    /// one and only one King is always present in an Army arrangement
    /// (for this reason it is also not necessaty to return an `Option` here,
    /// because a [ChessArmy] always has the King)
    ///
    fn get_king_position(&self) -> Cell {
        //self.pieces[ChessPiece::King as usize]
        self.pieces[ChessPiece::King as usize].active_cell().unwrap()
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] King.
    ///
    fn king_controlled_cells(&self) -> BitBoard {
        BitBoard::from(crate::bbdefines::neighbour(self.get_king_position()))
    }

}
// ****************************************************************************
// TESTS
// ****************************************************************************
// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor_building_a_white_army() {
        let a = ChessArmy::new(ArmyColour::White);
        check_white_initial_placement(&a);
    }

    #[test]
    fn default_constructor_building_a_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        check_black_initial_placement(&a);
    }

    #[test]
    fn test_reset_to_white_method() {
        let mut a = ChessArmy::new(ArmyColour::Black);
        check_black_initial_placement(&a);
        a.reset(ArmyColour::White);
        check_white_initial_placement(&a);
    }

    #[test]
    fn test_get_piece_in_cell_in_initial_white_army() {
        let a = ChessArmy::new(ArmyColour::White);
        assert_eq!(a.get_piece_in_cell(Cell::A1), Some(ChessPiece::Rook));
        assert_eq!(a.get_piece_in_cell(Cell::H1), Some(ChessPiece::Rook));
        assert_eq!(a.get_piece_in_cell(Cell::B1), Some(ChessPiece::Knight));
        assert_eq!(a.get_piece_in_cell(Cell::G1), Some(ChessPiece::Knight));
        assert_eq!(a.get_piece_in_cell(Cell::C1), Some(ChessPiece::Bishop));
        assert_eq!(a.get_piece_in_cell(Cell::F1), Some(ChessPiece::Bishop));
        assert_eq!(a.get_piece_in_cell(Cell::D1), Some(ChessPiece::Queen));
        assert_eq!(a.get_piece_in_cell(Cell::E1), Some(ChessPiece::King));
        assert_eq!(a.get_piece_in_cell(Cell::A2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::B2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::C2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::D2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::E2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::F2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::G2), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::H2), Some(ChessPiece::Pawn));
    }

    #[test]
    fn test_get_piece_in_cell_in_initial_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(a.get_piece_in_cell(Cell::A8), Some(ChessPiece::Rook));
        assert_eq!(a.get_piece_in_cell(Cell::H8), Some(ChessPiece::Rook));
        assert_eq!(a.get_piece_in_cell(Cell::B8), Some(ChessPiece::Knight));
        assert_eq!(a.get_piece_in_cell(Cell::G8), Some(ChessPiece::Knight));
        assert_eq!(a.get_piece_in_cell(Cell::C8), Some(ChessPiece::Bishop));
        assert_eq!(a.get_piece_in_cell(Cell::F8), Some(ChessPiece::Bishop));
        assert_eq!(a.get_piece_in_cell(Cell::D8), Some(ChessPiece::Queen));
        assert_eq!(a.get_piece_in_cell(Cell::E8), Some(ChessPiece::King));
        assert_eq!(a.get_piece_in_cell(Cell::A7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::B7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::C7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::D7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::E7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::F7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::G7), Some(ChessPiece::Pawn));
        assert_eq!(a.get_piece_in_cell(Cell::H7), Some(ChessPiece::Pawn));
    }

    #[test]
    fn test_king_controlled_cells_in_initial_white_army() {
        let a = ChessArmy::new(ArmyColour::White);
        assert_eq!(a.king_controlled_cells(), BitBoard::from_cells(&[Cell::D1, Cell::F1, Cell::D2, Cell::E2, Cell::F2]));
    }
    #[test]
    fn test_king_controlled_cells_in_initial_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(a.king_controlled_cells(), BitBoard::from_cells(&[Cell::D8, Cell::F8, Cell::D7, Cell::E7, Cell::F7]));
    }

    fn check_white_initial_placement(a: &ChessArmy) {
        assert_eq!(a.colour, ArmyColour::White);
        assert_eq!(
            a.pieces[ChessPiece::King as usize],
            BitBoard::from_cells(&[Cell::E1])
        );
        assert_eq!(
            a.pieces[ChessPiece::Queen as usize],
            BitBoard::from_cells(&[Cell::D1])
        );
        assert_eq!(
            a.pieces[ChessPiece::Bishop as usize],
            BitBoard::from_cells(&[Cell::C1, Cell::F1])
        );
        assert_eq!(
            a.pieces[ChessPiece::Knight as usize],
            BitBoard::from_cells(&[Cell::B1, Cell::G1])
        );
        assert_eq!(
            a.pieces[ChessPiece::Rook as usize],
            BitBoard::from_cells(&[Cell::A1, Cell::H1])
        );
        assert_eq!(
            a.pieces[ChessPiece::Pawn as usize],
            BitBoard::from_cells(&[
                Cell::A2,
                Cell::B2,
                Cell::C2,
                Cell::D2,
                Cell::E2,
                Cell::F2,
                Cell::G2,
                Cell::H2
            ])
        );
    }

    fn check_black_initial_placement(a: &ChessArmy) {
        assert_eq!(a.colour, ArmyColour::Black);
        assert_eq!(
            a.pieces[ChessPiece::King as usize],
            BitBoard::from_cells(&[Cell::E8])
        );
        assert_eq!(
            a.pieces[ChessPiece::Queen as usize],
            BitBoard::from_cells(&[Cell::D8])
        );
        assert_eq!(
            a.pieces[ChessPiece::Bishop as usize],
            BitBoard::from_cells(&[Cell::C8, Cell::F8])
        );
        assert_eq!(
            a.pieces[ChessPiece::Knight as usize],
            BitBoard::from_cells(&[Cell::B8, Cell::G8])
        );
        assert_eq!(
            a.pieces[ChessPiece::Rook as usize],
            BitBoard::from_cells(&[Cell::A8, Cell::H8])
        );
        assert_eq!(
            a.pieces[ChessPiece::Pawn as usize],
            BitBoard::from_cells(&[
                Cell::A7,
                Cell::B7,
                Cell::C7,
                Cell::D7,
                Cell::E7,
                Cell::F7,
                Cell::G7,
                Cell::H7
            ])
        );
    }
}
