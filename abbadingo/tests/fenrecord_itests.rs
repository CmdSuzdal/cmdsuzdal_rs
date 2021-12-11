use abbadingo::fenrecord::{FenRecord, INITIAL_STANDARD_POSITION};

// ------------------------------------------------------------
#[test]
fn itest_print_default_fen_record() {
    let fr = FenRecord::new();
    println!("FEN Record created with the default constructor: {}", fr.fen());
    println!("FEN Record with initial standard position      : {}", INITIAL_STANDARD_POSITION);
}
