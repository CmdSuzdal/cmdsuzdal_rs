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
#[derive(Debug, Copy, Clone)]
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

    /// Returns a [BitBoard] with the cells occupied by army pieces (including pawn).
    ///
    /// # Example
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::bitboard::BitBoard;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::new(ArmyColour::Black);
    /// assert_eq!(army.occupied_cells(), BitBoard::from(0xFF_FF_00_00_00_00_00_00));
    /// ```
    pub fn occupied_cells(&self) -> BitBoard {
        BitBoard::from(
            self.pieces[ChessPiece::King as usize].state
                | self.pieces[ChessPiece::Queen as usize].state
                | self.pieces[ChessPiece::Pawn as usize].state
                | self.pieces[ChessPiece::Bishop as usize].state
                | self.pieces[ChessPiece::Knight as usize].state
                | self.pieces[ChessPiece::Rook as usize].state,
        )
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
        self.pieces[ChessPiece::King as usize]
            .active_cell()
            .unwrap()
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] King.
    ///
    fn king_controlled_cells(&self) -> BitBoard {
        BitBoard::from(crate::bbdefines::neighbour(self.get_king_position()))
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] Pawns.
    ///
    fn pawns_controlled_cells(&self) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining_pawns = self.pieces[ChessPiece::Pawn as usize].pop_count();
        let mut cell_ndx = Cell::A2 as usize; // needless to check first and last rank

        while cell_ndx < Cell::A8 as usize && remaining_pawns > 0 {
            // We can unwrap safely here... cell_ndx is always valid
            if let Some(ChessPiece::Pawn) =
                self.get_piece_in_cell(num::FromPrimitive::from_usize(cell_ndx).unwrap())
            {
                bb |= ChessArmy::pawn_controlled_cells(
                    num::FromPrimitive::from_usize(cell_ndx).unwrap(),
                    self.colour,
                );
                remaining_pawns -= 1;
            }
            cell_ndx += 1;
        }
        bb
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] Knights.
    ///
    fn knights_controlled_cells(&self) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining = self.pieces[ChessPiece::Knight as usize].pop_count();
        let mut cell_ndx = Cell::A1 as usize;

        while cell_ndx <= Cell::H8 as usize && remaining > 0 {
            // We can unwrap safely here... cell_ndx is always valid
            if let Some(ChessPiece::Knight) =
                self.get_piece_in_cell(num::FromPrimitive::from_usize(cell_ndx).unwrap())
            {
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), 2, 1)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), 1, 2)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), -1, 2)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), -2, 1)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), -2, -1)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), -1, -2)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), 1, -2)
                {
                    bb.set_cell(cell);
                }
                if let Some(cell) =
                    calc_cell_after_steps(num::FromPrimitive::from_usize(cell_ndx).unwrap(), 2, -1)
                {
                    bb.set_cell(cell);
                }
                remaining -= 1;
            }
            cell_ndx += 1;
        }
        bb
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] Bishops.
    ///
    /// The "interference board" is provided to add a set of cell occupied by some
    /// other pieces. This, together with the cell occupied by the [ChessArmy] itself,
    /// can limit the view of the current army pieces.
    ///
    /// The normal use of the interference board is to pass the position of the
    /// pieces of the enemy army (see the ChessBoard class)
    ///
    /// # Arguments
    ///
    /// `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn bishops_controlled_cells(&self, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining = self.pieces[ChessPiece::Bishop as usize].pop_count();
        let busy_cells_bitboard = self.occupied_cells() | intf_board;
        let mut cell_ndx = Cell::A1 as usize;

        while cell_ndx <= Cell::H8 as usize && remaining > 0 {
            // We can unwrap safely here... cell_ndx is always valid
            if let Some(ChessPiece::Bishop) =
                self.get_piece_in_cell(num::FromPrimitive::from_usize(cell_ndx).unwrap())
            {
                let f = file(num::FromPrimitive::from_usize(cell_ndx).unwrap());
                let r = rank(num::FromPrimitive::from_usize(cell_ndx).unwrap());

                // Bishop found in position cell_ndx, (file f, rank r)
                // Eplore diagonal and antidiagonals for controlled
                // cells. The cells are controlled until a busy cell
                // is found: the busy cell is the last controlled one.

                // Explore the left-lower section of the diagonal
                let mut file_ndx = f as i32 - 1;
                let mut rank_ndx = r as i32 - 1;
                while file_ndx >= 0 && rank_ndx >= 0 {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(),
                        num::FromPrimitive::from_i32(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(),
                        num::FromPrimitive::from_i32(rank_ndx).unwrap(),
                    )) {
                        break;
                    }
                    file_ndx -= 1;
                    rank_ndx -= 1;
                }
                // Explore the right-upper section of the diagonal
                let mut file_ndx = f as usize + 1;
                let mut rank_ndx = r as usize + 1;
                while file_ndx < NUM_FILES && rank_ndx < NUM_RANKS {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    )) {
                        break;
                    }
                    file_ndx += 1;
                    rank_ndx += 1;
                }
                // Explore the left-upper section of the antidiagonal
                let mut file_ndx = f as i32 - 1;
                let mut rank_ndx = r as usize + 1;
                while file_ndx >= 0 && rank_ndx < NUM_RANKS {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(),
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(),
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    )) {
                        break;
                    }
                    file_ndx -= 1;
                    rank_ndx += 1;
                }
                // Explore the right-lower section of the antidiagonal
                let mut file_ndx = f as usize + 1;
                let mut rank_ndx = r as i32 - 1;
                while file_ndx < NUM_FILES && rank_ndx >= 0 {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        num::FromPrimitive::from_i32(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        num::FromPrimitive::from_i32(rank_ndx).unwrap(),
                    )) {
                        break;
                    }
                    file_ndx += 1;
                    rank_ndx -= 1;
                }
                remaining -= 1;
            }
            cell_ndx += 1;
        }
        bb
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] rooks.
    ///
    /// The "interference board" is provided to add a set of cell occupied by some
    /// other pieces. This, together with the cell occupied by the [ChessArmy] itself,
    /// can limit the view of the current army pieces.
    ///
    /// The normal use of the interference board is to pass the position of the
    /// pieces of the enemy army (see the ChessBoard class)
    ///
    /// # Arguments
    ///
    /// `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn rooks_controlled_cells(&self, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining = self.pieces[ChessPiece::Rook as usize].pop_count();
        let busy_cells_bitboard = self.occupied_cells() | intf_board;
        let mut cell_ndx = Cell::A1 as usize;

        while cell_ndx <= Cell::H8 as usize && remaining > 0 {
            // We can unwrap safely here... cell_ndx is always valid
            if let Some(ChessPiece::Rook) =
                self.get_piece_in_cell(num::FromPrimitive::from_usize(cell_ndx).unwrap())
            {
                let f = file(num::FromPrimitive::from_usize(cell_ndx).unwrap());
                let r = rank(num::FromPrimitive::from_usize(cell_ndx).unwrap());

                // Rook found in position cell_ndx, (file f, rank r)
                // Eplore rank and file for controlled cells.
                // The cells are controlled until a busy cell
                // is found: the busy cell is the last controlled one.

                // Explore the left side of the rank
                let mut file_ndx = f as i32 - 1;
                while file_ndx >= 0 {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(), r);
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_i32(file_ndx).unwrap(), r)) {
                        break;
                    }
                    file_ndx -= 1;
                }
                // Explore the right side of the rank
                let mut file_ndx = f as usize + 1;
                while file_ndx < NUM_FILES {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(), r);
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(), r)) {
                        break;
                    }
                    file_ndx += 1;
                }
                // Explore the lower side of the file
                let mut rank_ndx = r as i32 - 1;
                while rank_ndx >= 0 {
                    bb.set_cell_from_file_and_rank(
                        f, num::FromPrimitive::from_i32(rank_ndx).unwrap());
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        f, num::FromPrimitive::from_i32(rank_ndx).unwrap())) {
                        break;
                    }
                    rank_ndx -= 1;
                }
                // Explore the upper side of the file
                let mut rank_ndx = r as usize + 1;
                while rank_ndx < NUM_RANKS {
                    bb.set_cell_from_file_and_rank(
                        f, num::FromPrimitive::from_usize(rank_ndx).unwrap());
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        f, num::FromPrimitive::from_usize(rank_ndx).unwrap())) {
                        break;
                    }
                    rank_ndx += 1;
                }
                remaining -= 1;
            }
            cell_ndx += 1;
        }
        bb
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy] queens.
    ///
    /// The "interference board" is provided to add a set of cell occupied by some
    /// other pieces. This, together with the cell occupied by the [ChessArmy] itself,
    /// can limit the view of the current army pieces.
    ///
    /// The normal use of the interference board is to pass the position of the
    /// pieces of the enemy army (see the ChessBoard class)
    ///
    /// # Arguments
    ///
    /// `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn queens_controlled_cells(&self, intf_board: BitBoard) -> BitBoard {

        // Cells controlled by Queens is the union of the cells
        // controlled by rooks and bishops in the same position
        // of the queens. The code below is quite tricky... we have
        // to convert bishops and rooks in pawn to mantain interference
        // and avoid to signal wrong controlled cells and than:
        //  - place Bishops in the Queens positions and compute the controlled cells
        //  - place Rooks in the Queens positions and add the controlled cells
        //
        let mut fake_army = *self;
        fake_army.pieces[ChessPiece::Pawn as usize] |= fake_army.pieces[ChessPiece::Bishop as usize];
        fake_army.pieces[ChessPiece::Pawn as usize] |= fake_army.pieces[ChessPiece::Rook as usize];

        fake_army.pieces[ChessPiece::Bishop as usize] = fake_army.pieces[ChessPiece::Queen as usize];
        fake_army.pieces[ChessPiece::Queen as usize] = BitBoard::new();
        let mut bb = fake_army.bishops_controlled_cells(intf_board);

        fake_army.pieces[ChessPiece::Rook as usize] = fake_army.pieces[ChessPiece::Bishop as usize];
        fake_army.pieces[ChessPiece::Bishop as usize] = BitBoard::new();
        bb |= fake_army.rooks_controlled_cells(intf_board);
        bb
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by a pawn
    /// in the given position.
    ///
    /// [ChessArmy] type-associated function that, given a [Cell]
    /// and a [ChessArmy] colour (black or white), returns the BitBoard with
    /// the cell controlled by a pawn of the given colour placed in that cells
    ///
    /// # Arguments:
    ///
    /// * `c`: the [Cell] where the pawn is placed
    /// * `ac`: The [ArmyColour] of the pawn
    ///
    fn pawn_controlled_cells(c: Cell, ac: ArmyColour) -> BitBoard {
        let mut bb = BitBoard::new();
        match ac {
            ArmyColour::White => {
                if let Some(cell) = nw(c) {
                    bb.set_cell(cell);
                }
                if let Some(cell) = ne(c) {
                    bb.set_cell(cell);
                }
            }
            ArmyColour::Black => {
                if let Some(cell) = sw(c) {
                    bb.set_cell(cell);
                }
                if let Some(cell) = se(c) {
                    bb.set_cell(cell);
                }
            }
        }
        bb
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
        assert_eq!(
            a.king_controlled_cells(),
            BitBoard::from_cells(&[Cell::D1, Cell::F1, Cell::D2, Cell::E2, Cell::F2])
        );
    }
    #[test]
    fn test_king_controlled_cells_in_initial_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a.king_controlled_cells(),
            BitBoard::from_cells(&[Cell::D8, Cell::F8, Cell::D7, Cell::E7, Cell::F7])
        );
    }

    #[test]
    fn test_cell_controlled_by_single_pawn() {
        assert_eq!(
            ChessArmy::pawn_controlled_cells(Cell::E2, ArmyColour::White),
            BitBoard::from_cells(&[Cell::D3, Cell::F3])
        );
        assert_eq!(
            ChessArmy::pawn_controlled_cells(Cell::H6, ArmyColour::Black),
            BitBoard::from_cells(&[Cell::G5])
        );
        assert_eq!(
            ChessArmy::pawn_controlled_cells(Cell::E8, ArmyColour::White),
            BitBoard::new()
        );
    }
    #[test]
    fn test_cell_controlled_by_all_pawns_of_initial_white_army() {
        let a = ChessArmy::new(ArmyColour::White);
        assert_eq!(
            a.pawns_controlled_cells(),
            BitBoard::from_cells(&[
                Cell::A3,
                Cell::B3,
                Cell::C3,
                Cell::D3,
                Cell::E3,
                Cell::F3,
                Cell::G3,
                Cell::H3
            ])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_pawns_of_initial_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a.pawns_controlled_cells(),
            BitBoard::from_cells(&[
                Cell::A6,
                Cell::B6,
                Cell::C6,
                Cell::D6,
                Cell::E6,
                Cell::F6,
                Cell::G6,
                Cell::H6
            ])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_knights_of_initial_white_army() {
        let a = ChessArmy::new(ArmyColour::White);
        assert_eq!(
            a.knights_controlled_cells(),
            BitBoard::from_cells(&[Cell::A3, Cell::C3, Cell::D2, Cell::E2, Cell::F3, Cell::H3])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_knights_of_initial_black_army() {
        let a = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a.knights_controlled_cells(),
            BitBoard::from_cells(&[Cell::A6, Cell::C6, Cell::D7, Cell::E7, Cell::F6, Cell::H6])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_bishops_of_initial_white_and_black_army() {
        let a_white = ChessArmy::new(ArmyColour::White);
        let a_black = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a_white.bishops_controlled_cells(a_black.occupied_cells()),
            BitBoard::from_cells(&[Cell::B2, Cell::D2, Cell::E2, Cell::G2])
        );
        assert_eq!(
            a_black.bishops_controlled_cells(a_white.occupied_cells()),
            BitBoard::from_cells(&[Cell::B7, Cell::D7, Cell::E7, Cell::G7])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_rooks_of_initial_white_and_black_army() {
        let a_white = ChessArmy::new(ArmyColour::White);
        let a_black = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a_white.rooks_controlled_cells(a_black.occupied_cells()),
            BitBoard::from_cells(&[Cell::A2, Cell::B1, Cell::G1, Cell::H2])
        );
        assert_eq!(
            a_black.rooks_controlled_cells(a_white.occupied_cells()),
            BitBoard::from_cells(&[Cell::A7, Cell::B8, Cell::G8, Cell::H7])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_queens_of_initial_white_and_black_army() {
        let a_white = ChessArmy::new(ArmyColour::White);
        let a_black = ChessArmy::new(ArmyColour::Black);
        assert_eq!(
            a_white.queens_controlled_cells(a_black.occupied_cells()),
            BitBoard::from_cells(&[Cell::C1, Cell::C2, Cell::D2, Cell::E2, Cell::E1])
        );
        assert_eq!(
            a_black.queens_controlled_cells(a_white.occupied_cells()),
            BitBoard::from_cells(&[Cell::C8, Cell::C7, Cell::D7, Cell::E7, Cell::E8])
        );
    }

    // ------------------------------------------------------------------------------
    // utility (non-test) functions
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
