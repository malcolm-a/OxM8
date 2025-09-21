use oxm8::board::Board;
use oxm8::fen::{START_FEN, parse_fen, to_fen};
use oxm8::game::ChessGame;
use oxm8::moves::{MoveType, Moves};
use oxm8::piece::Color;

fn test_fen(fen: &str) {
    println!("Original: {}", fen);
    match parse_fen(fen) {
        Ok(board) => println!("Generated: {}", to_fen(&board)),
        Err(e) => println!("Error parsing FEN: {}", e),
    }
}

fn demonstrate_engine_capabilities() {
    println!("\n=== OxM8 Chess Engine Capabilities ===\n");

    // 1. FEN parsing and generation
    println!("1. FEN PARSING & GENERATION");
    test_fen(START_FEN);
    test_fen("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 1");

    // 2. Move generation
    println!("\n2. MOVE GENERATION");
    let board = Board::from_fen(START_FEN);
    println!("Starting position:");
    board.display();

    let all_moves = Moves::generate_all_moves(&board, Color::White);
    let legal_moves = Moves::generate_legal_moves(&board, Color::White);

    println!("Total pseudo-legal moves: {}", all_moves.len());
    println!("Legal moves: {}", legal_moves.len());

    println!("Sample legal moves:");
    for mv in legal_moves.iter().take(8) {
        let piece_info = if let Some((piece, _)) = board.get_piece_at(mv.from) {
            format!("{:?}", piece)
        } else {
            "Unknown".to_string()
        };
        println!(
            "  {} ({}): {:?}",
            mv.to_algebraic(),
            piece_info,
            mv.move_type
        );
    }
    if legal_moves.len() > 8 {
        println!("  ... and {} more", legal_moves.len() - 8);
    }

    // 3. Special moves demonstration
    println!("\n3. SPECIAL MOVES");

    // Castling
    println!("\nCastling:");
    let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    let all_moves = Moves::generate_all_moves(&board, Color::White);
    let castle_moves: Vec<&Moves> = all_moves
        .iter()
        .filter(|m| matches!(m.move_type, MoveType::Castle))
        .collect();

    for mv in castle_moves {
        let castle_type = if mv.to == 6 { "Kingside" } else { "Queenside" };
        println!(
            "  {} castling available: {}",
            castle_type,
            mv.to_algebraic()
        );
    }

    // En passant
    println!("\nEn passant:");
    let board = Board::from_fen("8/8/8/pP6/8/8/8/8 w - a6 0 1");
    let all_moves = Moves::generate_all_moves(&board, Color::White);
    let ep_moves: Vec<&Moves> = all_moves
        .iter()
        .filter(|m| matches!(m.move_type, MoveType::EnPassant))
        .collect();

    for mv in ep_moves {
        println!("  En passant capture: {}", mv.to_algebraic());
    }

    // Promotion
    println!("\nPromotion:");
    let board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1");
    let all_moves = Moves::generate_all_moves(&board, Color::White);
    let promo_moves: Vec<&Moves> = all_moves.iter().filter(|m| m.is_promotion()).collect();

    for mv in promo_moves {
        if let MoveType::Promotion { piece } = mv.move_type {
            println!("  Promote to {:?}: {}", piece, mv.to_algebraic());
        }
    }

    // 4. Game state detection
    println!("\n4. GAME STATE DETECTION");

    // Check
    let board = Board::from_fen("4k3/8/8/8/4Q3/8/8/4K3 b - - 0 1");
    println!(
        "Black king in check: {}",
        Moves::is_in_check(&board, Color::Black)
    );

    // Checkmate
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/5PPP/R6K b - - 0 1");
    println!(
        "Black in checkmate: {}",
        Moves::is_checkmate(&board, Color::Black)
    );

    // Stalemate
    let board = Board::from_fen("k7/8/1K6/1Q6/8/8/8/8 b - - 0 1");
    println!(
        "Black in stalemate: {}",
        Moves::is_stalemate(&board, Color::Black)
    );

    println!("\n=== Ready to Play! ===");
    println!("The engine supports:");
    println!("✅ All piece movements (including castling, en passant, promotion)");
    println!("✅ Legal move generation and validation");
    println!("✅ Check, checkmate, and stalemate detection");
    println!("✅ FEN import/export for position analysis");
    println!("✅ Complete chess rule implementation");
}

fn main() {
    println!("🏰 Welcome to OxM8 Chess Engine! 🏰");

    // Show engine capabilities
    demonstrate_engine_capabilities();

    // Ask user what they want to do
    println!("\nWhat would you like to do?");
    println!("1. Play interactive chess game");
    println!("2. Run engine demonstrations");
    println!("3. Exit");

    use std::io::{self, Write};
    print!("\nEnter choice (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        match input.trim() {
            "1" => {
                println!("\nStarting interactive chess game...\n");
                let mut game = ChessGame::new();
                game.run();
            }
            "2" => {
                println!("\nRunning additional engine demonstrations...");
                // Could add more advanced demonstrations here
                println!("Check the source code and tests for more examples!");
                println!("Run 'cargo test' to see comprehensive test coverage.");
            }
            "3" | "" => {
                println!("Thanks for checking out OxM8 Chess! 👋");
            }
            _ => {
                println!("Invalid choice. Exiting...");
            }
        }
    }
}
