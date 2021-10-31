//! Definition of the [ChessArmy] structure and related methods implementation.
//!

use crate::bbdefines::*;
use crate::chessdefines::*;
use crate::bitboard::{BitBoard};

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
        let mut a = ChessArmy { pieces: [BitBoard::new(); NUM_PIECES_TYPES], colour: c };
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
                self.pieces[ChessPiece::Bishop as usize] = BitBoard::from_cells(&[Cell::C1, Cell::F1]);
                self.pieces[ChessPiece::Knight as usize] = BitBoard::from_cells(&[Cell::B1, Cell::G1]);
                self.pieces[ChessPiece::Rook as usize] = BitBoard::from_cells(&[Cell::A1, Cell::H1]);
                self.pieces[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces[ChessPiece::Pawn as usize].set_rank(Rank::Rank2);
            },
            ArmyColour::Black => {
                self.pieces[ChessPiece::King as usize] = BitBoard::from_cells(&[Cell::E8]);
                self.pieces[ChessPiece::Queen as usize] = BitBoard::from_cells(&[Cell::D8]);
                self.pieces[ChessPiece::Bishop as usize] = BitBoard::from_cells(&[Cell::C8, Cell::F8]);
                self.pieces[ChessPiece::Knight as usize] = BitBoard::from_cells(&[Cell::B8, Cell::G8]);
                self.pieces[ChessPiece::Rook as usize] = BitBoard::from_cells(&[Cell::A8, Cell::H8]);
                self.pieces[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces[ChessPiece::Pawn as usize].set_rank(Rank::Rank7);
            },
        }
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
        assert_eq!(a.colour, ArmyColour::White);
        assert_eq!(a.pieces[ChessPiece::King as usize], BitBoard::from_cells(&[Cell::E1]));
        assert_eq!(a.pieces[ChessPiece::Queen as usize], BitBoard::from_cells(&[Cell::D1]));
        assert_eq!(a.pieces[ChessPiece::Bishop as usize], BitBoard::from_cells(&[Cell::C1, Cell::F1]));
        assert_eq!(a.pieces[ChessPiece::Knight as usize], BitBoard::from_cells(&[Cell::B1, Cell::G1]));
        assert_eq!(a.pieces[ChessPiece::Rook as usize], BitBoard::from_cells(&[Cell::A1, Cell::H1]));
        //assert_eq!(a.pieces[ChessPiece::Pawn as usize], BitBoard::from_cells(&[Cell::]));
    }

    #[test]
    fn default_constructor_building_a_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(a.colour, ArmyColour::Black);
        assert_eq!(a.pieces[ChessPiece::King as usize], BitBoard::from_cells(&[Cell::E8]));
        assert_eq!(a.pieces[ChessPiece::Queen as usize], BitBoard::from_cells(&[Cell::D8]));
        assert_eq!(a.pieces[ChessPiece::Bishop as usize], BitBoard::from_cells(&[Cell::C8, Cell::F8]));
        assert_eq!(a.pieces[ChessPiece::Knight as usize], BitBoard::from_cells(&[Cell::B8, Cell::G8]));
        assert_eq!(a.pieces[ChessPiece::Rook as usize], BitBoard::from_cells(&[Cell::A8, Cell::H8]));
        //assert_eq!(a.pieces[ChessPiece::Pawn as usize], BitBoard::from_cells(&[Cell::]));
    }
}
