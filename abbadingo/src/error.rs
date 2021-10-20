// Many thanks to Nick Groegen: https://nick.groenen.me/posts/rust-error-handling/
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
/// Enumerates all the possible errors returned by this library
pub enum AbbaDingoError {
    /// Invalid operation of File requested. e.g. required to compute
    /// the 'west' File of a Cell in the A File 
    #[error("Invalid operation on File")]
    InvalidOperationOnFile,
    /// Invalid operation of Rank requested. e.g. required to compute
    /// the 'south' File of a Cell in the first Rank
    #[error("Invalid operation on Rank")]
    InvalidOperationOnRank,
}