use crate::board::Board;
use crate::fen::START_FEN;
use crate::moves::{MoveType, Moves};
use crate::piece::{Color, Piece};
use crate::util::{parse_algebraic, pos_to_u8};
use std::io::{self, Write};

pub struct ChessGame {
    board: Board,
    current_player: Color,
    move_history: Vec<String>,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
            current_player: Color::White,
            move_history: Vec::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        Self {
            board: Board::from_fen(fen),
            current_player: Color::White,
            move_history: Vec::new(),
        }
    }

    fn display_board(&self) {
        println!("\n   a b c d e f g h");
        println!("  ┌─────────────────┐");

        for rank in (0..8).rev() {
            print!("{} │ ", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                match self.board.get_piece_at(square) {
                    Some((piece, color)) => {
                        let symbol = match (piece, color) {
                            (Piece::King, Color::White) => "♔",
                            (Piece::Queen, Color::White) => "♕",
                            (Piece::Rook, Color::White) => "♖",
                            (Piece::Bishop, Color::White) => "♗",
                            (Piece::Knight, Color::White) => "♘",
                            (Piece::Pawn, Color::White) => "♙",
                            (Piece::King, Color::Black) => "♚",
                            (Piece::Queen, Color::Black) => "♛",
                            (Piece::Rook, Color::Black) => "♜",
                            (Piece::Bishop, Color::Black) => "♝",
                            (Piece::Knight, Color::Black) => "♞",
                            (Piece::Pawn, Color::Black) => "♟",
                        };
                        print!("{} ", symbol);
                    }
                    None => print!("· "),
                }
            }
            println!("│ {}", rank + 1);
        }

        println!("  └─────────────────┘");
        println!("   a b c d e f g h\n");
    }

    fn display_game_status(&self) {
        let current_color_name = match self.current_player {
            Color::White => "White",
            Color::Black => "Black",
        };

        println!("Current player: {}", current_color_name);

        if Moves::is_in_check(&self.board, self.current_player) {
            if Moves::is_checkmate(&self.board, self.current_player) {
                let winner = match self.current_player {
                    Color::White => "Black",
                    Color::Black => "White",
                };
                println!("🏁 CHECKMATE! {} wins!", winner);
            } else {
                println!("⚠️  {} is in CHECK!", current_color_name);
            }
        } else if Moves::is_stalemate(&self.board, self.current_player) {
            println!("🤝 STALEMATE! The game is a draw.");
        }

        let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);
        println!("Legal moves available: {}", legal_moves.len());
    }

    fn show_legal_moves(&self) {
        let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);

        if legal_moves.is_empty() {
            println!("No legal moves available!");
            return;
        }

        println!("\nLegal moves (showing first 20):");
        for (i, mv) in legal_moves.iter().take(20).enumerate() {
            let piece_name = if let Some((piece, _)) = self.board.get_piece_at(mv.from) {
                format!("{:?}", piece)
            } else {
                "?".to_string()
            };

            let move_desc = match mv.move_type {
                MoveType::Normal => "",
                MoveType::Capture => " (capture)",
                MoveType::Castle => " (castle)",
                MoveType::EnPassant => " (en passant)",
                MoveType::Double => " (double pawn)",
                MoveType::Promotion { piece } => &format!(" (promote to {:?})", piece),
                MoveType::PromotionCapture { piece } => {
                    &format!(" (capture + promote to {:?})", piece)
                }
            };

            print!(
                "{:2}. {} ({}){}",
                i + 1,
                mv.to_algebraic(),
                piece_name,
                move_desc
            );
            if (i + 1) % 2 == 0 {
                println!();
            } else {
                print!("    ");
            }
        }

        if legal_moves.len() % 2 == 1 {
            println!();
        }

        if legal_moves.len() > 20 {
            println!("... and {} more moves", legal_moves.len() - 20);
        }
        println!();
    }

    fn looks_like_move_input(&self, input: &str) -> bool {
        let input = input.trim();

        // Check if it looks like coordinate notation (e.g., "e2e4", "g1f3")
        if input.len() == 4 {
            let chars: Vec<char> = input.chars().collect();
            if chars[0].is_ascii_lowercase()
                && chars[1].is_ascii_digit()
                && chars[2].is_ascii_lowercase()
                && chars[3].is_ascii_digit()
            {
                return true;
            }
        }

        // Check if it looks like coordinate notation with promotion (e.g., "e7e8=Q")
        if input.len() == 6 && input.contains('=') {
            return true;
        }

        // Check if it looks like space-separated coordinates
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 2 {
            if parts[0].len() == 2 && parts[1].len() == 2 {
                let from_chars: Vec<char> = parts[0].chars().collect();
                let to_chars: Vec<char> = parts[1].chars().collect();
                if from_chars[0].is_ascii_lowercase()
                    && from_chars[1].is_ascii_digit()
                    && to_chars[0].is_ascii_lowercase()
                    && to_chars[1].is_ascii_digit()
                {
                    return true;
                }
            }
        }

        false
    }

    fn parse_move_input(&self, input: &str) -> Option<Moves> {
        let input = input.trim();

        // Handle special notations
        match input.to_lowercase().as_str() {
            "o-o" | "0-0" => {
                // Kingside castling
                let king_square = match self.current_player {
                    Color::White => 4,  // e1
                    Color::Black => 60, // e8
                };
                let target_square = match self.current_player {
                    Color::White => 6,  // g1
                    Color::Black => 62, // g8
                };
                return Some(Moves::new(king_square, target_square, MoveType::Castle));
            }
            "o-o-o" | "0-0-0" => {
                // Queenside castling
                let king_square = match self.current_player {
                    Color::White => 4,  // e1
                    Color::Black => 60, // e8
                };
                let target_square = match self.current_player {
                    Color::White => 2,  // c1
                    Color::Black => 58, // c8
                };
                return Some(Moves::new(king_square, target_square, MoveType::Castle));
            }
            _ => {}
        }

        // Try to parse coordinate algebraic notation (e.g., "e2e4", "e7e8=Q")
        if let Some((from, to, promotion)) = parse_algebraic(input) {
            // Find the appropriate move type
            let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);

            for mv in legal_moves {
                if mv.from == from && mv.to == to {
                    match (promotion, &mv.move_type) {
                        (Some(piece), MoveType::Promotion { piece: mv_piece })
                        | (Some(piece), MoveType::PromotionCapture { piece: mv_piece }) => {
                            if piece == *mv_piece {
                                return Some(mv);
                            }
                        }
                        (None, _) if !mv.is_promotion() => {
                            return Some(mv);
                        }
                        _ => continue,
                    }
                }
            }
        }

        // Try simple position parsing (e.g., "e2 e4")
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Some(from), Some(to)) = (pos_to_u8(parts[0]), pos_to_u8(parts[1])) {
                let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);
                for mv in legal_moves {
                    if mv.from == from && mv.to == to && !mv.is_promotion() {
                        return Some(mv);
                    }
                }
            }
        }

        None
    }

    fn make_move(&mut self, mv: Moves) -> bool {
        // Verify the move is legal
        let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);
        if !legal_moves.contains(&mv) {
            return false;
        }

        // Record the move
        self.move_history.push(mv.to_algebraic());

        // Make the move
        self.board.make_move(&mv);

        // Switch players
        self.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    pub fn is_game_over(&self) -> bool {
        Moves::is_checkmate(&self.board, self.current_player)
            || Moves::is_stalemate(&self.board, self.current_player)
    }

    fn show_help(&self) {
        println!("\n=== CHESS GAME HELP ===");
        println!("Move formats:");
        println!("  • Coordinate notation: e2e4, g1f3, a7a8=Q");
        println!("  • Space separated: e2 e4");
        println!("  • Castling: O-O (kingside), O-O-O (queenside)");
        println!("\nCommands:");
        println!("  • 'moves' - Show all legal moves");
        println!("  • 'help' - Show this help");
        println!("  • 'quit' - Exit game");
        println!("  • 'history' - Show move history");
        println!("  • 'fen' - Show current position in FEN notation");
        println!("  • 'status' - Show detailed game status");
        println!();
    }

    fn show_history(&self) {
        if self.move_history.is_empty() {
            println!("No moves played yet.");
            return;
        }

        println!("\nMove History:");
        for (i, mv) in self.move_history.iter().enumerate() {
            if i % 2 == 0 {
                print!("{}. {}", i / 2 + 1, mv);
            } else {
                println!(" {}", mv);
            }
        }
        if self.move_history.len() % 2 == 1 {
            println!();
        }
        println!();
    }

    fn show_detailed_status(&self) {
        println!("\n=== GAME STATUS ===");

        let current_color_name = match self.current_player {
            Color::White => "White",
            Color::Black => "Black",
        };

        println!("Current player: {}", current_color_name);
        println!("Moves played: {}", self.move_history.len());
        println!("Castling rights: {:04b} (KQkq)", self.board.castling_rights);

        if let Some(ep) = self.board.en_passant {
            println!("En passant square: {}", crate::util::u8_to_pos(ep));
        }

        println!("Halfmove clock: {}", self.board.halfmove_clock);
        println!("Fullmove number: {}", self.board.fullmove_number);

        // Check game state
        if Moves::is_in_check(&self.board, Color::White) {
            println!("White is in check!");
        }
        if Moves::is_in_check(&self.board, Color::Black) {
            println!("Black is in check!");
        }

        let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);
        println!(
            "Legal moves for {}: {}",
            current_color_name,
            legal_moves.len()
        );
        println!();
    }

    pub fn run(&mut self) {
        println!("🏰 Welcome to OxM8 Chess! 🏰");
        println!("Type 'help' for commands or enter moves like 'e2e4' or 'g1f3'");

        loop {
            self.display_board();
            self.display_game_status();

            if self.is_game_over() {
                println!("Game Over! Type 'quit' to exit.");
                break;
            }

            print!("Enter move: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }

            let input = input.trim().to_lowercase();

            match input.as_str() {
                "quit" | "exit" | "q" => {
                    println!("Thanks for playing! 👋");
                    break;
                }
                "help" | "h" => {
                    self.show_help();
                    continue;
                }
                "moves" | "m" => {
                    self.show_legal_moves();
                    continue;
                }
                "history" => {
                    self.show_history();
                    continue;
                }
                "fen" => {
                    println!("Current position: {}", crate::fen::to_fen(&self.board));
                    continue;
                }
                "status" => {
                    self.show_detailed_status();
                    continue;
                }
                "" => continue,
                _ => {}
            }

            match self.parse_move_input(&input) {
                Some(mv) => {
                    if self.make_move(mv) {
                        println!("✅ Move played: {}", self.move_history.last().unwrap());
                    } else {
                        println!("❌ Illegal move! Try again.");
                    }
                }
                None => {
                    // Check if the input looks like coordinate notation but is illegal
                    if self.looks_like_move_input(&input) {
                        println!(
                            "❌ Illegal move! '{}' is not a legal move in this position.",
                            input
                        );
                    } else {
                        println!("❌ Invalid move format! Type 'help' for examples.");
                    }
                }
            }
        }
    }

    // Public API methods for external use
    pub fn get_legal_moves(&self) -> Vec<Moves> {
        Moves::generate_legal_moves(&self.board, self.current_player)
    }

    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn try_move(&mut self, from: &str, to: &str) -> Result<(), String> {
        let from_square = pos_to_u8(from).ok_or("Invalid from square")?;
        let to_square = pos_to_u8(to).ok_or("Invalid to square")?;

        let legal_moves = Moves::generate_legal_moves(&self.board, self.current_player);
        for mv in legal_moves {
            if mv.from == from_square && mv.to == to_square && !mv.is_promotion() {
                self.make_move(mv);
                return Ok(());
            }
        }

        Err("No legal move found between those squares".to_string())
    }

    pub fn try_move_algebraic(&mut self, move_str: &str) -> Result<(), String> {
        match self.parse_move_input(move_str) {
            Some(mv) => {
                if self.make_move(mv) {
                    Ok(())
                } else {
                    Err("Move is not legal".to_string())
                }
            }
            None => Err("Could not parse move".to_string()),
        }
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}
