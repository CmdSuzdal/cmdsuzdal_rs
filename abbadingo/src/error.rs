//! Errors definitions for the [abbadingo](crate) crate.

// Many thanks to Nick Groegen: https://nick.groenen.me/posts/rust-error-handling/
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
/// Enumerates all the possible errors returned by this library
pub enum AbbaDingoError {
    /// Invalid operation on File requested.
    #[error("Invalid operation on File")]
    InvalidOperationOnFile,
    /// Invalid operation on Rank requested.
    #[error("Invalid operation on Rank")]
    InvalidOperationOnRank,
    /// Illegal conversion to File.
    #[error("Illegal conversion to File")]
    IllegalConversionToFile,
    /// Illegal conversion to Rank.
    #[error("Illegal conversion to Rank")]
    IllegalConversionToRank,
}
