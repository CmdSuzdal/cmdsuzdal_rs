use abbadingo::bbdefines::{to_cell, Cell, File, Rank};
use abbadingo::bitboard::BitBoard;
use abbadingo::hexboard::HexCell;

fn main() {
    println!("--------------------------------------------------");
    let f = File::FileA;
    println!("Hello File: {:?} = '{}'", f, f);
    println!("Hello Rank: {:?} = '{}'", Rank::Rank3, Rank::Rank3);
    println!("Hello Cell: {:?} = '{}'", Cell::D4, Cell::D4);

    println!("--------------------------------------------------");
    let bb = BitBoard::new();
    println!("Hello BitBoard: {:?}", bb);

    let f = File::FileG;
    let r = Rank::Rank3;
    let c = to_cell(f, r);
    println!("File is {:?}, Rank is {:?}, Cell is {:?}", f, r, c);

    println!("--------------------------------------------------");
    let xb = HexCell::new();
    println!("Hello HexCell: {:?}", xb);

    println!("--------------------------------------------------");
}
