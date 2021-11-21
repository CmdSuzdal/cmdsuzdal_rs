//! Base definitions of items used in chess board related structures
//!
//! In this module there are for example definition for Chess pieces, and Army colours

use std::fmt;

use crate::error::AbbaDingoError;
use std::convert::TryFrom;

// ********************************************************************************
// ********************************************************************************
// CONSTs, ENUMs, STRUCTs, DEFINEs
// ********************************************************************************
// ********************************************************************************

/// The colour of a chess Army.
///
/// Army colour can be `White` or `Black`.
#[derive(Debug, Clone, Copy, FromPrimitive, PartialOrd, PartialEq, Eq)]
pub enum ArmyColour {
    White,
    Black,
}

/// The chess pieces.
///
/// The enum with the traditional chess pieces, from King to Pawn.
#[derive(Debug, Clone, Copy, FromPrimitive, PartialOrd, PartialEq, Eq)]
pub enum ChessPiece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

/// The number of possible Pieces types
pub const NUM_PIECES_TYPES: usize = 6;

// ********************************************************************************
// ********************************************************************************
// FUNCTIONS / METHODS / TRAITS
// ********************************************************************************
// ********************************************************************************

/// Display trait for [ChessPiece] structure.
///
/// Display a [ChessPiece] into its String representation ("King", "Queen", ...,"pawn").
///
/// # Example
/// ```
/// # use abbadingo::chessdefines::*;
/// assert_eq!(format!("{}", ChessPiece::Queen), "Queen");
/// assert_eq!(format!("{}", ChessPiece::Pawn), "pawn");
///
impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_repr = (match self {
            ChessPiece::King => "King",
            ChessPiece::Queen => "Queen",
            ChessPiece::Bishop => "Bishop",
            ChessPiece::Knight => "Knight",
            ChessPiece::Rook => "Rook",
            ChessPiece::Pawn => "pawn",
        })
        .to_string();
        write!(f, "{}", str_repr)
    }
}

/// Tentatively convert a &str with a piece in chess notation format
/// to the corresponding [ChessPiece].
///
/// For valid values ("K", "Q", ...) this function returns the
/// corresponding Ok(ChessPiece), otherwise Err(AbbaDingoError::IllegalConversionToChessPiece)
/// is returned.
///
/// # Example
/// ```
/// # use std::convert::TryFrom;
/// # use abbadingo::chessdefines::*;
/// # use abbadingo::error::AbbaDingoError;
/// assert_eq!(ChessPiece::try_from("K"), Ok(ChessPiece::King));
/// assert_eq!(ChessPiece::try_from("p"), Err(AbbaDingoError::IllegalConversionToChessPiece));
/// ```
///
impl TryFrom<&str> for ChessPiece {
    type Error = AbbaDingoError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "K" => Ok(ChessPiece::King),
            "Q" => Ok(ChessPiece::Queen),
            "B" => Ok(ChessPiece::Bishop),
            "N" => Ok(ChessPiece::Knight),
            "R" => Ok(ChessPiece::Rook),
            _ => Err(AbbaDingoError::IllegalConversionToChessPiece),
        }
    }
}
