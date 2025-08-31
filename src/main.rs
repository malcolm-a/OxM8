use oxm8::board::Board;
use oxm8::fen::{START_FEN, parse_fen, to_fen};



fn test_fen(fen: &str) {
    println!("Original: {}", fen);
    match parse_fen(fen) {
        Ok(board) => println!("Generated: {}", to_fen(&board)),
        Err(e) => println!("Error parsing FEN: {}", e),
    }
}

fn main() {
    let fen1 = "r1k5/1pQ1p3/2p4p/p2p2r1/6n1/1P4Bq/P1P2P2/R3R1K1 b - - 2 29";
    let b = Board::from_fen(fen1);
    b.display();
    test_fen(START_FEN);
    test_fen(fen1);
}
