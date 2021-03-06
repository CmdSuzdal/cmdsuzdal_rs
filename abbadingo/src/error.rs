//! Errors definitions for the [abbadingo](crate) crate.

// Many thanks to Nick Groegen: https://nick.groenen.me/posts/rust-error-handling/
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
/// Enumerates all the possible errors returned by this library
pub enum AbbaDingoError {
    /// Invalid operation on [File](crate::bbdefines::File) requested.
    #[error("Invalid operation on File")]
    InvalidOperationOnFile,
    /// Invalid operation on [Rank](crate::bbdefines::Rank) requested.
    #[error("Invalid operation on Rank")]
    InvalidOperationOnRank,
    /// Illegal conversion to [File](crate::bbdefines::File).
    #[error("Illegal conversion to File")]
    IllegalConversionToFile,
    /// Illegal conversion to [Rank](crate::bbdefines::Rank).
    #[error("Illegal conversion to Rank")]
    IllegalConversionToRank,
    /// Illegal conversion to [Cell](crate::bbdefines::Cell).
    #[error("Illegal conversion to Cell")]
    IllegalConversionToCell,
    /// Illegal conversion to [ChessPiece](crate::chessdefines::ChessPiece).
    #[error("Illegal conversion to ChessPiece")]
    IllegalConversionToChessPiece,
}
