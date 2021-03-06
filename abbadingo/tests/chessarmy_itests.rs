use abbadingo::bbdefines::Cell;
use abbadingo::bitboard::BitBoard;
use abbadingo::chessarmy::ChessArmy;
use abbadingo::chessdefines::{ArmyColour, ChessPiece};

// ------------------------------------------------------------
#[test]
fn itest_create_a_white_army_with_initial_position() {
    let a = ChessArmy::initial(ArmyColour::White);
    check_white_initial_placement(&a);
}

// ------------------------------------------------------------
#[test]
fn itest_create_a_black_army_with_initial_position() {
    let a = ChessArmy::initial(ArmyColour::Black);
    check_black_initial_placement(&a);
}

// ------------------------------------------------------------
#[test]
fn itest_create_empty_armies_using_the_empty_constructor() {
    let white_empty_army = ChessArmy::new(ArmyColour::White);
    assert_eq!(white_empty_army.num_pieces(), 0);
    assert_eq!(white_empty_army.colour, ArmyColour::White);
    assert!(white_empty_army.get_pieces(ChessPiece::King).is_empty());
    assert!(white_empty_army.get_pieces(ChessPiece::Queen).is_empty());
    assert!(white_empty_army.get_pieces(ChessPiece::Bishop).is_empty());
    assert!(white_empty_army.get_pieces(ChessPiece::Knight).is_empty());
    assert!(white_empty_army.get_pieces(ChessPiece::Rook).is_empty());
    assert!(white_empty_army.get_pieces(ChessPiece::Pawn).is_empty());
    let black_empty_army = ChessArmy::new(ArmyColour::Black);
    assert_eq!(black_empty_army.num_pieces(), 0);
    assert_eq!(black_empty_army.colour, ArmyColour::Black);
    assert!(black_empty_army.get_pieces(ChessPiece::King).is_empty());
    assert!(black_empty_army.get_pieces(ChessPiece::Queen).is_empty());
    assert!(black_empty_army.get_pieces(ChessPiece::Bishop).is_empty());
    assert!(black_empty_army.get_pieces(ChessPiece::Knight).is_empty());
    assert!(black_empty_army.get_pieces(ChessPiece::Rook).is_empty());
    assert!(black_empty_army.get_pieces(ChessPiece::Pawn).is_empty());
}

// ------------------------------------------------------------
#[test]
fn itest_initial_white_army_starting_from_empty_chessboard_and_placing_pieces() {
    let mut a = ChessArmy::new(ArmyColour::White);
    a.place_pieces(ChessPiece::King, &[Cell::E1]);
    a.place_pieces(ChessPiece::Queen, &[Cell::D1]);
    a.place_pieces(ChessPiece::Bishop, &[Cell::C1, Cell::F1]);
    a.place_pieces(ChessPiece::Knight, &[Cell::B1, Cell::G1]);
    a.place_pieces(ChessPiece::Rook, &[Cell::A1, Cell::H1]);
    a.place_pieces(
        ChessPiece::Pawn,
        &[
            Cell::A2,
            Cell::B2,
            Cell::C2,
            Cell::D2,
            Cell::E2,
            Cell::F2,
            Cell::G2,
            Cell::H2,
        ],
    );
    check_white_initial_placement(&a);
}
// ------------------------------------------------------------
#[test]
fn itest_initial_black_army_starting_from_empty_chessboard_and_placing_pieces() {
    let mut a = ChessArmy::new(ArmyColour::Black);
    a.place_pieces(ChessPiece::King, &[Cell::E8]);
    a.place_pieces(ChessPiece::Queen, &[Cell::D8]);
    a.place_pieces(ChessPiece::Bishop, &[Cell::C8, Cell::F8]);
    a.place_pieces(ChessPiece::Knight, &[Cell::B8, Cell::G8]);
    a.place_pieces(ChessPiece::Rook, &[Cell::A8, Cell::H8]);
    a.place_pieces(
        ChessPiece::Pawn,
        &[
            Cell::A7,
            Cell::B7,
            Cell::C7,
            Cell::D7,
            Cell::E7,
            Cell::F7,
            Cell::G7,
            Cell::H7,
        ],
    );
    check_black_initial_placement(&a);
}

// ------------------------------------------------------------
#[test]
fn itest_test_possible_moves_for_armies_in_start_position() {
    let w_army = ChessArmy::initial(ArmyColour::White);
    let b_army = ChessArmy::initial(ArmyColour::Black);
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::King,
            Cell::E1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Queen,
            Cell::D1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Bishop,
            Cell::C1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Bishop,
            Cell::F1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Rook,
            Cell::A1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Rook,
            Cell::H1,
            b_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Knight,
            Cell::B1,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::A3, Cell::C3])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Knight,
            Cell::G1,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::F3, Cell::H3])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::A2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::A3, Cell::A4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::B2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::B3, Cell::B4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::C2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::C3, Cell::C4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::D2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::D3, Cell::D4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::E2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::E3, Cell::E4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::F2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::F3, Cell::F4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::G2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::G3, Cell::G4])
    );
    assert_eq!(
        w_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::H2,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::H3, Cell::H4])
    );

    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::King,
            Cell::E8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Queen,
            Cell::D8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Bishop,
            Cell::C8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Bishop,
            Cell::F8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Rook,
            Cell::A8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Rook,
            Cell::H8,
            w_army.occupied_cells()
        ),
        BitBoard::new()
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Knight,
            Cell::B8,
            w_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::A6, Cell::C6])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Knight,
            Cell::G8,
            w_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::F6, Cell::H6])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::A7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::A6, Cell::A5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::B7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::B6, Cell::B5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::C7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::C6, Cell::C5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::D7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::D6, Cell::D5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::E7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::E6, Cell::E5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::F7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::F6, Cell::F5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::G7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::G6, Cell::G5])
    );
    assert_eq!(
        b_army.possible_moves_for_piece_in_cell(
            ChessPiece::Pawn,
            Cell::H7,
            b_army.occupied_cells()
        ),
        BitBoard::from_cells(&[Cell::H6, Cell::H5])
    );
}

// ------------------------------------------------------------------------------
// utility (non-test) functions
fn check_white_initial_placement(a: &ChessArmy) {
    assert_eq!(a.colour, ArmyColour::White);
    assert_eq!(a.num_pieces(), 16);
    assert_eq!(
        a.occupied_cells(),
        BitBoard::from(0x00_00_00_00_00_00_FF_FF)
    );
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
    assert_eq!(a.num_pieces(), 16);
    assert_eq!(
        a.occupied_cells(),
        BitBoard::from(0xFF_FF_00_00_00_00_00_00)
    );
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
