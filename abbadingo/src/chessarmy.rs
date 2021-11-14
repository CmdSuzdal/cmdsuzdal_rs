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
    pieces_bmask: [BitBoard; NUM_PIECES_TYPES], // private: pieces bitmask as accessed using the get_pieces() function
    pub colour: ArmyColour,
}

impl ChessArmy {
    /// Default constructor for the [ChessArmy] struct.
    /// Instantiate an empty [ChessArmy] of the given colour
    ///
    /// # Arguments
    ///
    /// * `c` - The colour of the [ChessArmy].
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
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
    ///```
    ///
    pub fn new(c: ArmyColour) -> ChessArmy {
        ChessArmy {
            pieces_bmask: [BitBoard::new(); NUM_PIECES_TYPES],
            colour: c,
        }
    }

    /// Returns an [ChessArmy] of the given colour with the initial chess game position.
    ///
    /// # Arguments
    ///
    /// * `c` - The colour of the [ChessArmy].
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::chessdefines::{ArmyColour, ChessPiece};
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let white_army = ChessArmy::initial(ArmyColour::White);
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
    /// let black_army = ChessArmy::initial(ArmyColour::Black);
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
    ///```
    pub fn initial(c: ArmyColour) -> ChessArmy {
        let mut a = ChessArmy {
            pieces_bmask: [BitBoard::new(); NUM_PIECES_TYPES],
            colour: c,
        };
        a.reset(c);
        a
    }

    /// Gets the BitBoard of the pieces for a [ChessArmy].
    /// This is a convenience method to avoid to continuously cast the
    /// [ChessPiece] to usize when directly accessing the `pieces` bitmasks
    ///
    /// # Arguments
    ///
    /// * `cp` - The [ChessPiece] type to which the [BitBoard] shall be returned.
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::bbdefines::{Cell};
    /// # use abbadingo::bitboard::{BitBoard};
    /// # use abbadingo::chessdefines::{ArmyColour, ChessPiece };
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::initial(ArmyColour::White);
    /// assert_eq!(army.get_pieces(ChessPiece::King), BitBoard::from_cells(&[Cell::E1]));
    ///```
    pub fn get_pieces(&self, cp: ChessPiece) -> BitBoard {
        self.pieces_bmask[cp as usize]
    }

    /// Place some pieces of the given [ChessPiece] type to the [ChessArmy].
    ///
    /// No checks are done: for example if the cells is already occupied by
    /// another piece the operation is obviously incorrect, but this method
    /// silenty accept the operation, with the result that the resulting
    /// Army state is invalid
    ///
    /// # Arguments
    ///
    /// * cp - The [ChessPiece] type to be placed in the [ChessArmy]
    /// * cells - The [Cell]s where to place the pieces
    ///
    /// # Example:
    /// ```
    /// # use abbadingo::bbdefines::{Cell};
    /// # use abbadingo::bitboard::{BitBoard};
    /// # use abbadingo::chessdefines::{ArmyColour, ChessPiece };
    /// # use abbadingo::chessarmy::ChessArmy;
    /// // Place two additional Queens in position G4 and B8 in the initial white army deployment
    /// let mut army = ChessArmy::initial(ArmyColour::White);
    /// army.place_pieces(ChessPiece::Queen, &[Cell::G4, Cell::B8]);
    /// assert_eq!(army.get_pieces(ChessPiece::Queen), BitBoard::from_cells(&[Cell::D1, Cell::G4, Cell::B8]));
    ///```
    pub fn place_pieces(&mut self, cp: ChessPiece, cells: &[Cell]) {
        self.pieces_bmask[cp as usize] |= BitBoard::from_cells(cells);
    }

    /// Returns the number of Pieces (including pawn) of a [ChessArmy].
    ///
    /// # Example
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::initial(ArmyColour::White);
    /// assert_eq!(army.num_pieces(), 16);
    /// ```
    pub fn num_pieces(&self) -> usize {
        self.get_pieces(ChessPiece::King).pop_count()
            + self.get_pieces(ChessPiece::Queen).pop_count()
            + self.get_pieces(ChessPiece::Bishop).pop_count()
            + self.get_pieces(ChessPiece::Knight).pop_count()
            + self.get_pieces(ChessPiece::Rook).pop_count()
            + self.get_pieces(ChessPiece::Pawn).pop_count()
    }

    /// Returns a [BitBoard] with the cells occupied by army pieces (including pawn).
    ///
    /// # Example
    /// ```
    /// # use abbadingo::chessdefines::ArmyColour;
    /// # use abbadingo::bitboard::BitBoard;
    /// # use abbadingo::chessarmy::ChessArmy;
    /// let mut army = ChessArmy::initial(ArmyColour::Black);
    /// assert_eq!(army.occupied_cells(), BitBoard::from(0xFF_FF_00_00_00_00_00_00));
    /// ```
    pub fn occupied_cells(&self) -> BitBoard {
        self.get_pieces(ChessPiece::King)
            | self.get_pieces(ChessPiece::Queen)
            | self.get_pieces(ChessPiece::Pawn)
            | self.get_pieces(ChessPiece::Bishop)
            | self.get_pieces(ChessPiece::Knight)
            | self.get_pieces(ChessPiece::Rook)
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
    /// let army = ChessArmy::initial(ArmyColour::Black);
    /// assert_eq!(army.get_piece_in_cell(Cell::C8), Some(ChessPiece::Bishop));
    /// assert_eq!(army.get_piece_in_cell(Cell::C1), None);
    /// ```
    ///
    pub fn get_piece_in_cell(&self, c: Cell) -> Option<ChessPiece> {
        if self.get_pieces(ChessPiece::King).cell_is_active(c) {
            Some(ChessPiece::King)
        } else if self.get_pieces(ChessPiece::Queen).cell_is_active(c) {
            Some(ChessPiece::Queen)
        } else if self.get_pieces(ChessPiece::Bishop).cell_is_active(c) {
            Some(ChessPiece::Bishop)
        } else if self.get_pieces(ChessPiece::Knight).cell_is_active(c) {
            Some(ChessPiece::Knight)
        } else if self.get_pieces(ChessPiece::Rook).cell_is_active(c) {
            Some(ChessPiece::Rook)
        } else if self.get_pieces(ChessPiece::Pawn).cell_is_active(c) {
            Some(ChessPiece::Pawn)
        } else {
            None
        }
    }

    /// Returns the [BitBoard] with the [Cell]s controlled by all the [ChessArmy] pieces and pawns.
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
    /// # Example
    /// ```
    /// # use abbadingo::bbdefines::{Cell};
    /// # use abbadingo::bitboard::{BitBoard};
    /// # use abbadingo::chessdefines::{ArmyColour};
    /// # use abbadingo::chessarmy::{ChessArmy};
    /// let w_army = ChessArmy::initial(ArmyColour::White);
    /// let b_army = ChessArmy::initial(ArmyColour::Black);
    /// assert_eq!(w_army.controlled_cells(b_army.occupied_cells()), BitBoard::from(0x00_00_00_00_00_FF_FF_7E));
    /// assert_eq!(b_army.controlled_cells(w_army.occupied_cells()), BitBoard::from_cells(&[
    ///     Cell::B8, Cell::C8, Cell::D8, Cell::E8, Cell::F8, Cell::G8,
    ///     Cell::A7, Cell::B7, Cell::C7, Cell::D7, Cell::E7, Cell::F7, Cell::G7, Cell::H7,
    ///     Cell::A6, Cell::B6, Cell::C6, Cell::D6, Cell::E6, Cell::F6, Cell::G6, Cell::H6
    /// ]));
    /// ```
    ///
    pub fn controlled_cells(&self, intf_board: BitBoard) -> BitBoard {
        self.king_controlled_cells()
            | self.queens_controlled_cells(intf_board)
            | self.bishops_controlled_cells(intf_board)
            | self.knights_controlled_cells()
            | self.rooks_controlled_cells(intf_board)
            | self.pawns_controlled_cells()
    }

    // ---------------------------------------------------------------------------
    // PRIVATE METHODS
    // ---------------------------------------------------------------------------

    /// Initialize a [ChessArmy] of the specified colour with the initial standard chess deployment.
    ///
    /// Can be used to reset an already existing [ChessArmy] to initial state
    /// instead to create a new army using the [initial()](crate::chessarmy::ChessArmy::initial) constructor.
    ///
    /// # Arguments
    ///
    /// * `c` - The [ArmyColour] of the new arrangement of the [ChessArmy].
    ///
    fn reset(&mut self, c: ArmyColour) {
        self.colour = c;
        match c {
            ArmyColour::White => {
                self.pieces_bmask[ChessPiece::King as usize] = BitBoard::from_cells(&[Cell::E1]);
                self.pieces_bmask[ChessPiece::Queen as usize] = BitBoard::from_cells(&[Cell::D1]);
                self.pieces_bmask[ChessPiece::Bishop as usize] =
                    BitBoard::from_cells(&[Cell::C1, Cell::F1]);
                self.pieces_bmask[ChessPiece::Knight as usize] =
                    BitBoard::from_cells(&[Cell::B1, Cell::G1]);
                self.pieces_bmask[ChessPiece::Rook as usize] =
                    BitBoard::from_cells(&[Cell::A1, Cell::H1]);
                self.pieces_bmask[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces_bmask[ChessPiece::Pawn as usize].set_rank(Rank::Rank2);
            }
            ArmyColour::Black => {
                self.pieces_bmask[ChessPiece::King as usize] = BitBoard::from_cells(&[Cell::E8]);
                self.pieces_bmask[ChessPiece::Queen as usize] = BitBoard::from_cells(&[Cell::D8]);
                self.pieces_bmask[ChessPiece::Bishop as usize] =
                    BitBoard::from_cells(&[Cell::C8, Cell::F8]);
                self.pieces_bmask[ChessPiece::Knight as usize] =
                    BitBoard::from_cells(&[Cell::B8, Cell::G8]);
                self.pieces_bmask[ChessPiece::Rook as usize] =
                    BitBoard::from_cells(&[Cell::A8, Cell::H8]);
                self.pieces_bmask[ChessPiece::Pawn as usize] = BitBoard::new();
                self.pieces_bmask[ChessPiece::Pawn as usize].set_rank(Rank::Rank7);
            }
        }
    }




    /// Returns the [Cell] with the position of the King.
    ///
    /// This is the only "get_position" function that makes sense because
    /// one and only one King is always present in an Army arrangement
    /// (for this reason it is also not necessaty to return an `Option` here,
    /// because a [ChessArmy] always has the King)
    ///
    fn get_king_position(&self) -> Cell {
        self.get_pieces(ChessPiece::King).active_cell().unwrap()
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
        let mut remaining_pawns = self.get_pieces(ChessPiece::Pawn).pop_count();
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
        let mut remaining = self.get_pieces(ChessPiece::Knight).pop_count();
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
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn bishops_controlled_cells(&self, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining = self.get_pieces(ChessPiece::Bishop).pop_count();
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
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn rooks_controlled_cells(&self, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        let mut remaining = self.get_pieces(ChessPiece::Rook).pop_count();
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
                        num::FromPrimitive::from_i32(file_ndx).unwrap(),
                        r,
                    );
                    if busy_cells_bitboard
                        .cell_is_active(to_cell(num::FromPrimitive::from_i32(file_ndx).unwrap(), r))
                    {
                        break;
                    }
                    file_ndx -= 1;
                }
                // Explore the right side of the rank
                let mut file_ndx = f as usize + 1;
                while file_ndx < NUM_FILES {
                    bb.set_cell_from_file_and_rank(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        r,
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        num::FromPrimitive::from_usize(file_ndx).unwrap(),
                        r,
                    )) {
                        break;
                    }
                    file_ndx += 1;
                }
                // Explore the lower side of the file
                let mut rank_ndx = r as i32 - 1;
                while rank_ndx >= 0 {
                    bb.set_cell_from_file_and_rank(
                        f,
                        num::FromPrimitive::from_i32(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard
                        .cell_is_active(to_cell(f, num::FromPrimitive::from_i32(rank_ndx).unwrap()))
                    {
                        break;
                    }
                    rank_ndx -= 1;
                }
                // Explore the upper side of the file
                let mut rank_ndx = r as usize + 1;
                while rank_ndx < NUM_RANKS {
                    bb.set_cell_from_file_and_rank(
                        f,
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    );
                    if busy_cells_bitboard.cell_is_active(to_cell(
                        f,
                        num::FromPrimitive::from_usize(rank_ndx).unwrap(),
                    )) {
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
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
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
        fake_army.pieces_bmask[ChessPiece::Pawn as usize] |=
            fake_army.get_pieces(ChessPiece::Bishop);
        fake_army.pieces_bmask[ChessPiece::Pawn as usize] |= fake_army.get_pieces(ChessPiece::Rook);

        fake_army.pieces_bmask[ChessPiece::Bishop as usize] =
            fake_army.get_pieces(ChessPiece::Queen);
        fake_army.pieces_bmask[ChessPiece::Queen as usize] = BitBoard::new();
        let mut bb = fake_army.bishops_controlled_cells(intf_board);

        fake_army.pieces_bmask[ChessPiece::Rook as usize] =
            fake_army.get_pieces(ChessPiece::Bishop);
        fake_army.pieces_bmask[ChessPiece::Bishop as usize] = BitBoard::new();
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

    /// Returns the [BitBoard] with the [Cell]s controlled by the [ChessArmy]
    /// pieces of the given [ChessPiece].
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
    /// * `cp`: the type of the piece for which the controlled cells shall be computed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn controlled_cells_by_piece_type(&self, cp: ChessPiece, intf_board: BitBoard) -> BitBoard {
        match cp {
            ChessPiece::King => self.king_controlled_cells(),
            ChessPiece::Queen => self.queens_controlled_cells(intf_board),
            ChessPiece::Bishop => self.bishops_controlled_cells(intf_board),
            ChessPiece::Knight => self.knights_controlled_cells(),
            ChessPiece::Rook => self.rooks_controlled_cells(intf_board),
            ChessPiece::Pawn => self.pawns_controlled_cells(),
        }
    }

    /// Returns the [BitBoard] with the possible moves of a piece placed in the given
    /// position. The piece can be of amy [ChessPiece] type.
    ///
    /// The [ChessArmy] shall have a piece in the given position, otherwise
    /// an empty bitboard is returned. The interference board is used to limit the view of
    /// the given piece (see "controlled cells" functions). If there are no moves (e.g. blocked
    /// piece), an empty Bitboard is returned
    ///
    /// # Arguments
    ///
    /// * `cp`: the [ChessPiece] type of the piece to which the possible moves shall be determined
    /// * `c`: the [Cell] where the Piece is placed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn possible_move_for_piece_in_cell(
        &self,
        cp: ChessPiece,
        c: Cell,
        intf_board: BitBoard,
    ) -> BitBoard {
        match cp {
            ChessPiece::King => self.king_possible_moves(),
            ChessPiece::Pawn => self.possible_moves_for_pawn_in_cell(c, intf_board),
            _ => self.possible_moves_for_regular_piece_in_cell(cp, c, intf_board),
        }
    }

    /// Returns the [BitBoard] with the possible moves of the [ChessArmy] King.
    /// If no move are possible, the empty [BitBoard] is returned
    ///
    /// The king can move in any of its controlled cells that is not
    /// occupied by a piece of its army. As for other pieces we do not
    /// checks here for validity of moves (e.g. placing the king under check);
    /// the "possible moves" functions return the possible moves and not
    /// the valid ones. Check for validity shall be done from caller.
    ///
    fn king_possible_moves(&self) -> BitBoard {
        (self.king_controlled_cells() | self.occupied_cells()) ^ self.occupied_cells()
    }

    /// Returns the [BitBoard] with the possible moves of a 'regular' piece placed in the given
    /// position.
    ///
    /// This function works only form "regular" piece (not King, not Pawns).
    /// The [ChessArmy] shall have a piece in the given position, otherwise
    /// an empty bitboard is returned. The interference board is used to limit the view of
    /// the given piece (see "controlled cells" functions). If there are no moves (e.g. blocked
    /// piece), an empty Bitboard is returned. To avoid overhead no checks are performed on the
    /// correcteness of the assumption above, for example if the given [Cell] does not contain
    /// a piece of the correct type, this function can returns incorrect values.
    ///
    /// # Arguments
    ///
    /// * `cp`: the [ChessPiece] type of the piece to which the possible moves shall be determined
    /// * `c`: the [Cell] where the Piece is placed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn possible_moves_for_regular_piece_in_cell(
        &self,
        cp: ChessPiece,
        c: Cell,
        intf_board: BitBoard,
    ) -> BitBoard {
        let piece_bb = BitBoard::from_cells(&[c]);
        let mut fake_army = *self;
        fake_army.pieces_bmask[ChessPiece::Pawn as usize] |= self.get_pieces(cp) ^ piece_bb;
        fake_army.pieces_bmask[cp as usize] = piece_bb;
        (fake_army.controlled_cells_by_piece_type(cp, intf_board) | self.occupied_cells())
            ^ self.occupied_cells()
    }

    /// Returns the [BitBoard] with the possible moves of a pawn placed in the given
    /// position.
    ///
    /// The [ChessArmy] shall have a pawn in the given position, this function does not perform
    /// any check to avoid overhead. If this condition is not true, incorrect results could be
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `c`: the [Cell] where the Piece is placed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn possible_moves_for_pawn_in_cell(&self, c: Cell, intf_board: BitBoard) -> BitBoard {
        match self.colour {
            ArmyColour::White => self.possible_moves_for_white_pawn_in_cell(c, intf_board),
            ArmyColour::Black => self.possible_moves_for_black_pawn_in_cell(c, intf_board),
        }
    }

    /// Returns the [BitBoard] with the possible moves of a white pawn placed in the given
    /// position.
    ///
    /// The [ChessArmy] shall have a white pawn in the given position, and the pawn shall
    /// be in a valid position (e.g. in a rank from 2 to 7). This function does not
    /// perform any check to avoid overhead. If conditions are not true, incorrect
    /// results could be returned.
    ///
    /// # Arguments
    ///
    /// * `c`: the [Cell] where the Piece is placed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn possible_moves_for_white_pawn_in_cell(&self, c: Cell, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        if let Some(tentative_cell) = n(c) {
            if ((self.occupied_cells() | intf_board) & BitBoard::from_cells(&[tentative_cell]))
                != BitBoard::new()
            {
                // the north cell is free. Add to the BitBoard of possible moves
                bb.set_cell(tentative_cell);
                // If the cell is in the 2nd rank, the pawn can
                // could also perform two steps move
                if rank(c) == Rank::Rank2 {
                    if let Some(tentative_cell) = n(tentative_cell) {
                        if ((self.occupied_cells() | intf_board)
                            & BitBoard::from_cells(&[tentative_cell]))
                            != BitBoard::new()
                        {
                            // the north-north cell is free. Add to the BitBoard of possible moves
                            bb.set_cell(tentative_cell);
                        }
                    }
                }
            }
        }
        bb | (ChessArmy::pawn_controlled_cells(c, self.colour) & intf_board)
    }

    /// Returns the [BitBoard] with the possible moves of a black pawn placed in the given
    /// position.
    ///
    /// The [ChessArmy] shall have a black pawn in the given position, and the pawn shall
    /// be in a valid position (e.g. in a rank from 7 to 2). This function does not
    /// perform any check to avoid overhead. If conditions are not true, incorrect
    /// results could be returned.
    ///
    /// # Arguments
    ///
    /// * `c`: the [Cell] where the Piece is placed
    /// * `intf_board`: A [BitBoard] with pieces limiting the "view" of the [ChessArmy]
    ///
    fn possible_moves_for_black_pawn_in_cell(&self, c: Cell, intf_board: BitBoard) -> BitBoard {
        let mut bb = BitBoard::new();
        if let Some(tentative_cell) = s(c) {
            if ((self.occupied_cells() | intf_board) & BitBoard::from_cells(&[tentative_cell]))
                != BitBoard::new()
            {
                // the south cell is free. Add to the BitBoard of possible moves
                bb.set_cell(tentative_cell);
                // If the cell is in the 7th rank, the pawn can
                // could also perform two steps move
                if rank(c) == Rank::Rank7 {
                    if let Some(tentative_cell) = s(tentative_cell) {
                        if ((self.occupied_cells() | intf_board)
                            & BitBoard::from_cells(&[tentative_cell]))
                            != BitBoard::new()
                        {
                            // the south-south cell is free. Add to the BitBoard of possible moves
                            bb.set_cell(tentative_cell);
                        }
                    }
                }
            }
        }
        bb | (ChessArmy::pawn_controlled_cells(c, self.colour) & intf_board)
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

    // ------------------------------------------------------------
    #[test]
    fn test_get_piece_in_cell_in_initial_white_army() {
        let a = ChessArmy::initial(ArmyColour::White);
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

    // ------------------------------------------------------------
    #[test]
    fn test_get_piece_in_cell_in_initial_black_army() {
        let a = ChessArmy::initial(ArmyColour::Black);
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

    // ------------------------------------------------------------
    #[test]
    fn test_king_controlled_cells_in_initial_white_army() {
        let a = ChessArmy::initial(ArmyColour::White);
        assert_eq!(
            a.king_controlled_cells(),
            BitBoard::from_cells(&[Cell::D1, Cell::F1, Cell::D2, Cell::E2, Cell::F2])
        );
    }
    #[test]
    fn test_king_controlled_cells_in_initial_black_army() {
        let a = ChessArmy::initial(ArmyColour::Black);
        assert_eq!(
            a.king_controlled_cells(),
            BitBoard::from_cells(&[Cell::D8, Cell::F8, Cell::D7, Cell::E7, Cell::F7])
        );
    }

    // ------------------------------------------------------------
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
        let a = ChessArmy::initial(ArmyColour::White);
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
        let a = ChessArmy::initial(ArmyColour::Black);
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
        let a = ChessArmy::initial(ArmyColour::White);
        assert_eq!(
            a.knights_controlled_cells(),
            BitBoard::from_cells(&[Cell::A3, Cell::C3, Cell::D2, Cell::E2, Cell::F3, Cell::H3])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_knights_of_initial_black_army() {
        let a = ChessArmy::initial(ArmyColour::Black);
        assert_eq!(
            a.knights_controlled_cells(),
            BitBoard::from_cells(&[Cell::A6, Cell::C6, Cell::D7, Cell::E7, Cell::F6, Cell::H6])
        );
    }
    #[test]
    fn test_cell_controlled_by_all_bishops_of_initial_white_and_black_army() {
        let a_white = ChessArmy::initial(ArmyColour::White);
        let a_black = ChessArmy::initial(ArmyColour::Black);
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
        let a_white = ChessArmy::initial(ArmyColour::White);
        let a_black = ChessArmy::initial(ArmyColour::Black);
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
        let a_white = ChessArmy::initial(ArmyColour::White);
        let a_black = ChessArmy::initial(ArmyColour::Black);
        assert_eq!(
            a_white.queens_controlled_cells(a_black.occupied_cells()),
            BitBoard::from_cells(&[Cell::C1, Cell::C2, Cell::D2, Cell::E2, Cell::E1])
        );
        assert_eq!(
            a_black.queens_controlled_cells(a_white.occupied_cells()),
            BitBoard::from_cells(&[Cell::C8, Cell::C7, Cell::D7, Cell::E7, Cell::E8])
        );
    }

    #[test]
    fn test_cell_controlled_by_initial_white_and_black_army() {
        let a_white = ChessArmy::initial(ArmyColour::White);
        let a_black = ChessArmy::initial(ArmyColour::Black);
        assert_eq!(
            a_white.controlled_cells(a_black.occupied_cells()),
            BitBoard::from(0x00_00_00_00_00_FF_FF_7E)
        );
        assert_eq!(
            a_black.controlled_cells(a_black.occupied_cells()),
            BitBoard::from(0x7E_FF_FF_00_00_00_00_00)
        );
    }

    // ------------------------------------------------------------------------------
    // Possible Moves tests
    #[test]
    fn test_pieces_moves_for_armies_in_start_position() {
        let w_army = ChessArmy::initial(ArmyColour::White);
        let b_army = ChessArmy::initial(ArmyColour::Black);
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::King,
                Cell::E1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Queen,
                Cell::D1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Bishop,
                Cell::C1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Bishop,
                Cell::F1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Rook,
                Cell::A1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Rook,
                Cell::H1,
                b_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Knight,
                Cell::B1,
                b_army.occupied_cells()
            ),
            BitBoard::from_cells(&[Cell::A3, Cell::C3])
        );
        assert_eq!(
            w_army.possible_move_for_piece_in_cell(
                ChessPiece::Knight,
                Cell::G1,
                b_army.occupied_cells()
            ),
            BitBoard::from_cells(&[Cell::F3, Cell::H3])
        );

        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::King,
                Cell::E8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Queen,
                Cell::D8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Bishop,
                Cell::C8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Bishop,
                Cell::F8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Rook,
                Cell::A8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Rook,
                Cell::H8,
                w_army.occupied_cells()
            ),
            BitBoard::new()
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Knight,
                Cell::B8,
                w_army.occupied_cells()
            ),
            BitBoard::from_cells(&[Cell::A6, Cell::C6])
        );
        assert_eq!(
            b_army.possible_move_for_piece_in_cell(
                ChessPiece::Knight,
                Cell::G8,
                w_army.occupied_cells()
            ),
            BitBoard::from_cells(&[Cell::F6, Cell::H6])
        );
    }

    // ------------------------------------------------------------
    #[test]
    fn test_king_possible_moves_for_a_king_alone_in_e6() {
        let mut a = ChessArmy::new(ArmyColour::White);
        a.place_pieces(ChessPiece::King, &[Cell::E6]);
        assert_eq!(a.king_possible_moves(), BitBoard::from(neighbour(Cell::E6)));
        assert_eq!(
            a.king_possible_moves(),
            BitBoard::from_cells(&[
                Cell::D5,
                Cell::E5,
                Cell::F5,
                Cell::D6,
                Cell::F6,
                Cell::D7,
                Cell::E7,
                Cell::F7
            ])
        );
    }
    #[test]
    fn test_king_possible_moves_for_a_king_alone_in_a1() {
        let mut a = ChessArmy::new(ArmyColour::Black);
        a.place_pieces(ChessPiece::King, &[Cell::A1]);
        assert_eq!(a.num_pieces(), 1);
        assert_eq!(a.king_possible_moves(), BitBoard::from(neighbour(Cell::A1)));
        assert_eq!(
            a.king_possible_moves(),
            BitBoard::from_cells(&[Cell::A2, Cell::B2, Cell::B1])
        );
    }

    #[test]
    fn test_possible_moves_for_kings_in_opposition() {
        let mut a_w = ChessArmy::new(ArmyColour::White);
        let mut a_b = ChessArmy::new(ArmyColour::Black);
        a_w.place_pieces(ChessPiece::King, &[Cell::E5]);
        a_b.place_pieces(ChessPiece::King, &[Cell::E7]);
        assert_eq!(a_w.num_pieces(), 1);
        assert_eq!(a_b.num_pieces(), 1);

        // The king_possible_moves() does not check for move validity
        // so all the moves are returned, also the illegal ones
        assert_eq!(
            a_w.king_possible_moves(),
            BitBoard::from(neighbour(Cell::E5))
        );
        assert_eq!(
            a_b.king_possible_moves(),
            BitBoard::from(neighbour(Cell::E7))
        );
        assert_eq!(
            a_w.king_possible_moves(),
            BitBoard::from_cells(&[
                Cell::D4,
                Cell::E4,
                Cell::F4,
                Cell::D5,
                Cell::F5,
                Cell::D6,
                Cell::E6,
                Cell::F6
            ])
        );
        assert_eq!(
            a_b.king_possible_moves(),
            BitBoard::from_cells(&[
                Cell::D6,
                Cell::E6,
                Cell::F6,
                Cell::D7,
                Cell::F7,
                Cell::D8,
                Cell::E8,
                Cell::F8
            ])
        );
    }

    #[test]
    fn test_possible_moves_for_kings_obstructed_by_friends() {
        let mut a_w = ChessArmy::new(ArmyColour::White);
        let mut a_b = ChessArmy::new(ArmyColour::Black);
        a_w.place_pieces(ChessPiece::King, &[Cell::B2]);
        a_w.place_pieces(ChessPiece::Queen, &[Cell::A2]);
        a_w.place_pieces(ChessPiece::Rook, &[Cell::A1]);
        a_w.place_pieces(ChessPiece::Knight, &[Cell::C1, Cell::A3]);
        a_w.place_pieces(ChessPiece::Pawn, &[Cell::B3, Cell::C3, Cell::D3]);
        a_b.place_pieces(ChessPiece::King, &[Cell::H8]);
        a_b.place_pieces(ChessPiece::Pawn, &[Cell::G7, Cell::F6, Cell::E6]);
        a_b.place_pieces(ChessPiece::Rook, &[Cell::H7]);
        assert_eq!(a_w.num_pieces(), 8);
        assert_eq!(a_b.num_pieces(), 5);

        assert_eq!(
            a_w.possible_move_for_piece_in_cell(ChessPiece::King, Cell::B2, a_b.occupied_cells()),
            BitBoard::from_cells(&[Cell::B1, Cell::C2])
        );
        assert_eq!(
            a_b.possible_move_for_piece_in_cell(ChessPiece::King, Cell::H8, a_w.occupied_cells()),
            BitBoard::from_cells(&[Cell::G8])
        );
    }
}
