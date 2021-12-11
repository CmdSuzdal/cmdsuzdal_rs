//! Definition of the [FenRecord] structure to manage a string containing a chess game position
//! in the [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) (FEN).
//!

// These will be useful later...
//use crate::chessdefines::{ArmyColour};
//use crate::bitboard::BitBoard;


// From Wikipedia (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation):
// Forsyth–Edwards Notation (FEN) is a standard notation for describing a particular board
// position of a chess game. The purpose of FEN is to provide all the necessary information
// to restart a game from a particular position.
//
// FEN is based on a system developed by Scottish newspaper journalist David Forsyth.
// Forsyth's system became popular in the 19th century; Steven J. Edwards extended it
// to support use by computers. FEN is defined in the "Portable Game Notation
// Specification and Implementation Guide" (see documentation area). In the Portable
// Game Notation for chess games, FEN is used to define initial positions other than
// the standard one.[3] FEN does not provide sufficient information to decide whether
// a draw by threefold repetition may be legally claimed or a draw offer may be
// accepted; for that, a different format such as Extended Position Description is needed.
//
// A FEN "record" defines a particular game position, all in one text line and using
//only the ASCII character set. A text file with only FEN data records should have the
// file extension ".fen".[4]
//
// A FEN record contains six fields. The separator between fields is a space.
// The fields are:
//
// 1. Piece placement (from White's perspective). Each rank is described, starting with
//    rank 8 and ending with rank 1; within each rank, the contents of each square are
//    described from file "a" through file "h". Following the Standard Algebraic
//    Notation (SAN), each piece is identified by a single letter taken from the
//    standard English names (pawn = "P", knight = "N", bishop = "B", rook = "R",
//    queen = "Q" and king = "K"). White pieces are designated using upper-case letters
//    ("PNBRQK") while black pieces use lowercase ("pnbrqk"). Empty squares are noted
//    using digits 1 through 8 (the number of empty squares), and "/" separates ranks.
// 2. Active color. "w" means White moves next, "b" means Black moves next.
// 3. Castling availability. If neither side can castle, this is "-". Otherwise, this
//    has one or more letters: "K" (White can castle kingside), "Q" (White can castle
//    queenside), "k" (Black can castle kingside), and/or "q" (Black can castle
//    queenside). A move that temporarily prevents castling does not negate this
//    notation.
// 4. En passant target square in algebraic notation. If there's no en passant target
//    square, this is "-". If a pawn has just made a two-square move, this is the
//    position "behind" the pawn. This is recorded regardless of whether there is a
//    pawn in position to make an en passant capture.
// 5. Halfmove clock: This is the number of halfmoves since the last capture or pawn
//    advance. The reason for this field is that the value is used in the fifty-move
//    rule.
// 6. Fullmove number: The number of the full move. It starts at 1, and is incremented
//    after Black's move.
//

// This will be useful later...
// const DELIM: char = ' ';
pub const INITIAL_STANDARD_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const EMPTY_CHESS_BOARD: &str = "8/8/8/8/8/8/8/8 - - - 0 1";

pub struct FenRecord {
    fen: String,
}

impl FenRecord {
    /// Default constructor for the [FenRecord] struct: instantiate the initial chess standard position
    pub fn new() -> FenRecord {
        FenRecord { fen: INITIAL_STANDARD_POSITION.to_string() }
    }

    /// Return the current value of the FEN record as a string
    /// # Example:
    /// ```
    /// # use abbadingo::fenrecord::{FenRecord, INITIAL_STANDARD_POSITION};
    /// let fr = FenRecord::new();
    /// assert_eq!(fr.fen(), INITIAL_STANDARD_POSITION)
    ///```
    pub fn fen(&self) -> &str {
        &self.fen
    }
}

// ****************************************************************************
// TESTS
// ****************************************************************************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor_instantiate_initial_chess_standard_position() {
        let fr = FenRecord::new();
        assert_eq!(fr.fen(), INITIAL_STANDARD_POSITION)
    }
}