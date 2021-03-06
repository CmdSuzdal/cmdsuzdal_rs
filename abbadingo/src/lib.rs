//! Abba-dingo is a library with the implementation of several components useful to
//! implement board games related software (e.g. chess). The library for example
//! defines structures that can be used represent a chessboard, but also other types
//! of boards, using a bitboard-based approach.

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod bbdefines;
pub mod bitboard;
pub mod chessarmy;
pub mod chessdefines;
pub mod chessmove;
pub mod error;
pub mod fenrecord;
pub mod hexboard;
