use abbadingo::bbdefines::{Rank, File, to_cell};
use abbadingo::bitboard::BitBoard;

fn main() {
    let bb = BitBoard::new();
    println!("Hello Bitboard: {:?}", bb);

    let f = File::FileG;
    let r = Rank::Rank3;
    let c = to_cell(f, r);
    println!("File is {:?}, Rank is {:?}, Cell is {:?}", f, r, c);
}
