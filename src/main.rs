use oxm8::board::Board;
use oxm8::fen::{START_FEN, parse_fen, to_fen};
use oxm8::moves::{Moves, MoveType};
use oxm8::piece::{Color, Piece};

fn test_fen(fen: &str) {
    println!("Original: {}", fen);
    match parse_fen(fen) {
        Ok(board) => println!("Generated: {}", to_fen(&board)),
        Err(e) => println!("Error parsing FEN: {}", e),
    }
}

fn demonstrate_moves() {
    println!("\n=== Visual Chess Move Demonstrations ===\n");

    // 1. Pawn Promotion
    println!("1. PAWN PROMOTION");
    println!("White pawn on a7 promotes to Queen:");
    let mut board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1");
    println!("Before promotion:");
    board.display();

    let promotion_move = Moves::new(48, 56, MoveType::Promotion {
        piece: Piece::Queen
    });
    board.make_move(&promotion_move);
    println!("After promotion to Queen (a7-a8=Q):");
    board.display();

    // 2. En Passant Capture
    println!("\n2. EN PASSANT CAPTURE");
    println!("White pawn captures black pawn en passant:");
    let mut board = Board::from_fen("8/8/8/pP6/8/8/8/8 w - a6 0 1");
    println!("Before en passant (black pawn on a5, white on b5, en passant available on a6):");
    board.display();

    let en_passant_move = Moves::new(33, 40, MoveType::EnPassant); // b5 to a6
    board.make_move(&en_passant_move);
    println!("After en passant capture (b5xa6):");
    board.display();

    // 3. Double Pawn Push
    println!("\n3. DOUBLE PAWN PUSH");
    println!("White pawn moves two squares from starting position:");
    let mut board = Board::from_fen(START_FEN);
    println!("Before double pawn push:");
    board.display();

    let double_move = Moves::new(8, 24, MoveType::Double); // a2 to a4
    board.make_move(&double_move);
    println!("After double pawn push (a2-a4), en passant now available on a3:");
    board.display();
    println!("En passant square set to: {:?}", board.en_passant);

    // 4. Pawn Capture
    println!("\n4. PAWN CAPTURE");
    println!("White pawn captures black rook:");
    let mut board = Board::from_fen("8/8/8/8/8/1r6/2P5/8 w - - 0 1");
    println!("Before capture (white pawn on c2, black rook on b3):");
    board.display();

    let capture_move = Moves::new(10, 17, MoveType::Capture); // c2 to b3
    board.make_move(&capture_move);
    println!("After pawn capture (c2xb3):");
    board.display();

    // 5. Promotion with Capture
    println!("\n5. PROMOTION WITH CAPTURE");
    println!("White pawn captures and promotes:");
    let mut board = Board::from_fen("2r5/1P6/8/8/8/8/8/8 w - - 0 1");
    println!("Before promotion capture (white pawn on b7, black rook on c8):");
    board.display();

    let promotion_capture = Moves::new(49, 58, MoveType::PromotionCapture {
        piece: Piece::Queen
    }); // b7 to c8=Q
    board.make_move(&promotion_capture);
    println!("After promotion capture (b7xc8=Q):");
    board.display();

    // 6. Complex Position with Multiple Move Options
    println!("\n6. COMPLEX POSITION - MOVE GENERATION");
    println!("Showing all available pawn moves in a complex position:");
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    board.display();

    let white_moves = Moves::generate_all_moves(&board, Color::White);
    println!("White has {} possible pawn moves:", white_moves.len());
    for mv in &white_moves {
        let move_description = match mv.move_type {
            MoveType::Normal => "Normal move",
            MoveType::Double => "Double pawn push",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "En passant capture",
            MoveType::Promotion { piece } => &format!("Promotion to {:?}", piece),
            MoveType::PromotionCapture { piece } => &format!("Promotion capture to {:?}", piece),
            MoveType::Castle => "Castle",
        };
        println!("  {}: {}", mv.to_algebraic(), move_description);
    }
}

fn main() {
    println!("=== OxM8 Chess Engine Demo ===\n");

    // Display a complex position
    let fen1 = "r1k5/1pQ1p3/2p4p/p2p2r1/6n1/1P4Bq/P1P2P2/R3R1K1 b - - 2 29";
    let board = Board::from_fen(fen1);
    println!("Complex position:");
    board.display();

    // Test FEN parsing
    println!("\n=== FEN Parsing Tests ===");
    test_fen(START_FEN);
    test_fen(fen1);

    // Quick pawn moves demo
    println!("\n=== Pawn Moves Demo ===");
    let board = Board::from_fen(START_FEN);
    println!("Starting position:");
    board.display();

    let white_moves = Moves::generate_all_moves(&board, Color::White);
    println!("White has {} possible pawn moves:", white_moves.len());
    for mv in white_moves.iter().take(5) {
        println!("  {}: {:?}", mv.to_algebraic(), mv.move_type);
    }
    if white_moves.len() > 5 {
        println!("  ... and {} more", white_moves.len() - 5);
    }

    // Visual move demonstrations
    demonstrate_moves();

    println!("\nRun 'cargo test' to see comprehensive pawn movement tests!");
}
