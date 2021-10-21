use abbadingo::bbdefines::{Rank, File, to_cell};
use abbadingo::bitboard::BitBoard;
use abbadingo::hexboard::HexCell;

fn main() {

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
