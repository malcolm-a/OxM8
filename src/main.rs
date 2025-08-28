use oxm8::board::Board;
use oxm8::fen::{self, START_FEN, parse_fen, to_fen};
use oxm8::util;

fn test_fen() {
    println!("Original: {}", START_FEN);
    match parse_fen(START_FEN) {
        Ok(board) => println!("Generated: {}", to_fen(&board)),
        Err(e) => println!("Error parsing FEN: {}", e),
    }
}

fn main() {
    test_fen();
}
