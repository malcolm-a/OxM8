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
                add_promotions(&mut moves, square, one_forward, false);
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
                        add_promotions(&mut moves, square, capture_left, true);
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
                        add_promotions(&mut moves, square, capture_right, true);
                    } else {
                        moves.push(Moves::new(square, capture_right, MoveType::Capture));
                    }
                }
            }
        }

        // TODO: En passant captures
        if let Some(en_passant_square) = board.en_passant {
            // Left en passant
            if file > 0 && (square as i8 + left_capture_dir) as u8 == en_passant_square {
                moves.push(Moves::new(square, en_passant_square, MoveType::EnPassant));
            }
            // Right en passant
            if file < 7 && (square as i8 + right_capture_dir) as u8 == en_passant_square {
                moves.push(Moves::new(square, en_passant_square, MoveType::EnPassant));
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
}
