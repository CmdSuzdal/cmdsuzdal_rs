use abbadingo::bbdefines::{Cell};
use abbadingo::bitboard::{BitBoard};
use abbadingo::chessdefines::{ArmyColour, ChessPiece};
use abbadingo::chessarmy::ChessArmy;

// ------------------------------------------------------------
#[test]
fn itest_create_a_white_army_with_initial_position() {
    let a = ChessArmy::new(ArmyColour::White);
    check_white_initial_placement(&a);
}

// ------------------------------------------------------------
#[test]
fn itest_create_a_black_army_with_initial_position() {
    let a = ChessArmy::new(ArmyColour::Black);
    check_black_initial_placement(&a);
}

// ------------------------------------------------------------
#[test]
fn itest_modify_a_black_army_in_a_white_army_using_the_reset_fn() {
    let mut a = ChessArmy::new(ArmyColour::Black);
    check_black_initial_placement(&a);
    a.reset(ArmyColour::White);
    check_white_initial_placement(&a);
}

// ------------------------------------------------------------------------------
// utility (non-test) functions
fn check_white_initial_placement(a: &ChessArmy) {
    assert_eq!(a.colour, ArmyColour::White);
    assert_eq!(
        a.get_pieces(ChessPiece::King),
        BitBoard::from_cells(&[Cell::E1])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Queen),
        BitBoard::from_cells(&[Cell::D1])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Bishop),
        BitBoard::from_cells(&[Cell::C1, Cell::F1])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Knight),
        BitBoard::from_cells(&[Cell::B1, Cell::G1])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Rook),
        BitBoard::from_cells(&[Cell::A1, Cell::H1])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Pawn),
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
        a.get_pieces(ChessPiece::King),
        BitBoard::from_cells(&[Cell::E8])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Queen),
        BitBoard::from_cells(&[Cell::D8])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Bishop),
        BitBoard::from_cells(&[Cell::C8, Cell::F8])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Knight),
        BitBoard::from_cells(&[Cell::B8, Cell::G8])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Rook),
        BitBoard::from_cells(&[Cell::A8, Cell::H8])
    );
    assert_eq!(
        a.get_pieces(ChessPiece::Pawn),
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
