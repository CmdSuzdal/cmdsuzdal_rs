use abbadingo::bitboard::{BitBoard, Rank, File, Cell};

fn main() {
    let bb = BitBoard::new();
    println!("Hello Bitboard: {:?}", bb);

    let f = File::FileA;
    let r = Rank::Rank1;
    let c = Cell::A1;
    println!("File is {:?}, Rank is {:?}, Cell is {:?}", f, r, c);
}
