use abbadingo::bbdefines::*;
use abbadingo::bitboard::BitBoard;
use abbadingo::hexboard::HexCell;

fn main() {
    println!("--------------------------------------------------");
    let f = File::FileA;
    println!("Hello File: {:?} = '{}'", f, f);
    println!("Hello Rank: {:?} = '{}'", Rank::Rank3, Rank::Rank3);
    println!("Hello Cell: {:?} = '{}'", Cell::D4, Cell::D4);

    println!("--------------------------------------------------");
    let f = File::FileG;
    let r = Rank::Rank3;
    let c = to_cell(f, r);
    println!("File is {:?}, Rank is {:?}, Cell is {:?}", f, r, c);

    println!("--------------------------------------------------");
    let mut bb = BitBoard::new();
    bb.set_file(File::FileH);
    bb.set_rank(Rank::Rank8);
    bb.set_diagonal(Diagonal::Diag7);
    bb.set_antidiagonal(AntiDiagonal::AntiDiag6);
    println!("Hello BitBoard: {:?}", bb);
    println!("Bitboard: {}", bb);

    println!("--------------------------------------------------");
    let xb = HexCell::new();
    println!("Hello HexCell: {:?}", xb);

    println!("--------------------------------------------------");
}
