use crate::board::Board;
use crate::piece::{Color, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castle,
    Promotion { piece: Piece },
    Double,
    PromotionCapture { piece: Piece },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moves {
    pub from: u8,
    pub to: u8,
    pub move_type: MoveType,
}

impl Moves {
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Self {
        Moves {
            from,
            to,
            move_type,
        }
    }

    pub fn pawn_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();

        // Direction variables based on color
        let (forward_dir, start_rank, promotion_rank, left_capture_dir, right_capture_dir) =
            match color {
                Color::White => (8, 1, 6, 7, 9),    // White moves "up" the board
                Color::Black => (-8, 6, 1, -9, -7), // Black moves "down" the board
            };

        let rank = square / 8;
        let file = square % 8;

        // Forward moves
        let one_forward = (square as i8 + forward_dir) as u8;
        if one_forward < 64 && board.get_piece_at(one_forward).is_none() {
            if rank == promotion_rank {
                // Promotion
                Self::add_promotions(&mut moves, square, one_forward, false);
            } else {
                // Normal forward move
                moves.push(Moves::new(square, one_forward, MoveType::Normal));

                // Double pawn push from starting position
                if rank == start_rank {
                    let two_forward = (square as i8 + forward_dir * 2) as u8;
                    if board.get_piece_at(two_forward).is_none() {
                        moves.push(Moves::new(square, two_forward, MoveType::Double));
                    }
                }
            }
        }

        // Left diagonal capture
        if file > 0 {
            let capture_left = (square as i8 + left_capture_dir) as u8;
            if let Some((_, enemy_color)) = board.get_piece_at(capture_left) {
                if enemy_color != color {
                    if rank == promotion_rank {
                        Self::add_promotions(&mut moves, square, capture_left, true);
                    } else {
                        moves.push(Moves::new(square, capture_left, MoveType::Capture));
                    }
                }
            }
        }

        // Right diagonal capture
        if file < 7 {
            let capture_right = (square as i8 + right_capture_dir) as u8;
            if let Some((_, enemy_color)) = board.get_piece_at(capture_right) {
                if enemy_color != color {
                    if rank == promotion_rank {
                        Self::add_promotions(&mut moves, square, capture_right, true);
                    } else {
                        moves.push(Moves::new(square, capture_right, MoveType::Capture));
                    }
                }
            }
        }

        // En passant captures
        if let Some(en_passant_square) = board.en_passant {
            // Left en passant (capturing to the left)
            if file > 0 {
                let left_square = (square as i8 + left_capture_dir) as u8;
                if left_square == en_passant_square {
                    // The enemy pawn should be on the same rank as us, one file to the left
                    let enemy_pawn_square = square - 1;
                    let enemy_color = if color == Color::White { Color::Black } else { Color::White };
                    if let Some((piece, pawn_color)) = board.get_piece_at(enemy_pawn_square) {
                        if piece == Piece::Pawn && pawn_color == enemy_color {
                            moves.push(Moves::new(square, en_passant_square, MoveType::EnPassant));
                        }
                    }
                }
            }
            // Right en passant (capturing to the right)
            if file < 7 {
                let right_square = (square as i8 + right_capture_dir) as u8;
                if right_square == en_passant_square {
                    // The enemy pawn should be on the same rank as us, one file to the right
                    let enemy_pawn_square = square + 1;
                    let enemy_color = if color == Color::White { Color::Black } else { Color::White };
                    if let Some((piece, pawn_color)) = board.get_piece_at(enemy_pawn_square) {
                        if piece == Piece::Pawn && pawn_color == enemy_color {
                            moves.push(Moves::new(square, en_passant_square, MoveType::EnPassant));
                        }
                    }
                }
            }
        }

        moves
    }

    fn add_promotions(moves: &mut Vec<Moves>, from: u8, to: u8, is_capture: bool) {
        let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

        for piece in promotion_pieces {
            let move_type = if is_capture {
                MoveType::PromotionCapture { piece }
            } else {
                MoveType::Promotion { piece }
            };
            // This just adds Move objects to a list - no board changes!
            moves.push(Moves::new(from, to, move_type));
        }
    }

    /// Generate all legal moves for a given color
    pub fn generate_all_moves(board: &Board, color: Color) -> Vec<Moves> {
        let mut all_moves = Vec::new();

        // Generate pawn moves
        let pawn_squares = board.get_piece_squares(color, Piece::Pawn);
        for square in pawn_squares {
            let pawn_moves = Self::pawn_moves(board, square, color);
            all_moves.extend(pawn_moves);
        }

        // TODO: Add moves for other pieces

        all_moves
    }

    /// Convert a move to simple algebraic notation
    pub fn to_algebraic(&self) -> String {
        let from_file = (self.from % 8) as u8 + b'a';
        let from_rank = (self.from / 8) + 1;
        let to_file = (self.to % 8) as u8 + b'a';
        let to_rank = (self.to / 8) + 1;

        let promotion_suffix = match self.move_type {
            MoveType::Promotion { piece } | MoveType::PromotionCapture { piece } => {
                match piece {
                    Piece::Queen => "=Q",
                    Piece::Rook => "=R",
                    Piece::Bishop => "=B",
                    Piece::Knight => "=N",
                    _ => "",
                }
            }
            _ => "",
        };

        format!(
            "{}{}{}{}{}",
            from_file as char,
            from_rank,
            to_file as char,
            to_rank,
            promotion_suffix
        )
    }

    /// Check if a move is a promotion
    pub fn is_promotion(&self) -> bool {
        matches!(self.move_type, MoveType::Promotion { .. } | MoveType::PromotionCapture { .. })
    }

    /// Check if a move is a capture
    pub fn is_capture(&self) -> bool {
        matches!(self.move_type, MoveType::Capture | MoveType::EnPassant | MoveType::PromotionCapture { .. })
    }

    pub fn knight_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();
        let knight_offsets = [
            (2, 1), (1, 2), (-1, 2), (-2, 1),
            (-2, -1), (-1, -2), (1, -2), (2, -1),
        ];
        let rank = square / 8;
        let file = square % 8;
        for (dr, df) in &knight_offsets {
            let new_rank = rank as i8 + dr;
            let new_file = file as i8 + df;
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let to_square = (new_rank * 8 + new_file) as u8;
                if let Some((_, piece_color)) = board.get_piece_at(to_square) {
                    if piece_color != color {
                        let move_type = if piece_color == color { MoveType::Normal } else { MoveType::Capture };
                        moves.push(Moves::new(square, to_square, move_type));
                    }
                } else {
                    moves.push(Moves::new(square, to_square, MoveType::Normal));
                }
            }
        }
        moves
    }

    pub fn king_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();
        let king_offsets = [
            (1, 0), (1, 1), (0, 1), (-1, 1),
            (-1, 0), (-1, -1), (0, -1), (1, -1),
        ];
        let rank = square / 8;
        let file = square % 8;
        for (dr, df) in &king_offsets {
            let new_rank = rank as i8 + dr;
            let new_file = file as i8 + df;
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let to_square = (new_rank * 8 + new_file) as u8;
                if let Some((_, piece_color)) = board.get_piece_at(to_square) {
                    if piece_color != color {
                        let move_type = if piece_color == color { MoveType::Normal } else { MoveType::Capture };
                        moves.push(Moves::new(square, to_square, move_type));
                    }
                } else {
                    moves.push(Moves::new(square, to_square, MoveType::Normal));
                }
            }
        }
        // todo!(castling rights);
        moves
    }

    pub fn rook_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let rank = square / 8;
        let file = square % 8;

        for (dr, df) in &directions {
            let mut new_rank = rank as i8;
            let mut new_file = file as i8;

            loop {
                new_rank += dr;
                new_file += df;

                if new_rank < 0 || new_rank >= 8 || new_file < 0 || new_file >= 8 {
                    break;
                }

                let to_square = (new_rank * 8 + new_file) as u8;
                if let Some((_, piece_color)) = board.get_piece_at(to_square) {
                    if piece_color != color {
                        moves.push(Moves::new(square, to_square, MoveType::Capture));
                    }
                    break; // Stop on first piece encountered
                } else {
                    moves.push(Moves::new(square, to_square, MoveType::Normal));
                }
            }
        }

        moves
    }
    
    pub fn bishop_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]; // Diagonal directions
        let rank = square / 8;
        let file = square % 8;

        for (dr, df) in &directions {
            let mut new_rank = rank as i8;
            let mut new_file = file as i8;

            loop {
                new_rank += dr;
                new_file += df;

                if new_rank < 0 || new_rank >= 8 || new_file < 0 || new_file >= 8 {
                    break;
                }

                let to_square = (new_rank * 8 + new_file) as u8;
                if let Some((_, piece_color)) = board.get_piece_at(to_square) {
                    if piece_color != color {
                        moves.push(Moves::new(square, to_square, MoveType::Capture));
                    }
                    break; // Stop on first piece encountered
                } else {
                    moves.push(Moves::new(square, to_square, MoveType::Normal));
                }
            }
        }

        moves
    }
    
    pub fn queen_moves(board: &Board, square: u8, color: Color) -> Vec<Moves> {
        let mut moves = Vec::new();

        moves.extend(Self::rook_moves(board, square, color));
        moves.extend(Self::bishop_moves(board, square, color));
        
        moves
    }
}
