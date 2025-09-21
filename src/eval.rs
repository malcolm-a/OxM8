use crate::board::Board;
use crate::moves::{MoveType, Moves};
use crate::piece::{Color, Piece};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 0;

pub struct Eval {}

impl Eval {
    pub fn match_piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => PAWN_VALUE,
            Piece::Knight => KNIGHT_VALUE,
            Piece::Bishop => BISHOP_VALUE,
            Piece::Rook => ROOK_VALUE,
            Piece::Queen => QUEEN_VALUE,
            Piece::King => KING_VALUE,
        }
    }

    pub fn material(board: &Board, color: Color) -> i32 {
        match color {
            Color::White => {
                board.white_pawns.count_ones() as i32 * PAWN_VALUE
                    + board.white_knights.count_ones() as i32 * KNIGHT_VALUE
                    + board.white_bishops.count_ones() as i32 * BISHOP_VALUE
                    + board.white_rooks.count_ones() as i32 * ROOK_VALUE
                    + board.white_queens.count_ones() as i32 * QUEEN_VALUE
            }
            Color::Black => {
                board.black_pawns.count_ones() as i32 * PAWN_VALUE
                    + board.black_knights.count_ones() as i32 * KNIGHT_VALUE
                    + board.black_bishops.count_ones() as i32 * BISHOP_VALUE
                    + board.black_rooks.count_ones() as i32 * ROOK_VALUE
                    + board.black_queens.count_ones() as i32 * QUEEN_VALUE
            }
        }
    }

    pub fn material_balance(board: &Board) -> i32 {
        Self::material(board, Color::White) - Self::material(board, Color::Black)
    }

    pub fn mobility(board: &Board, color: Color) -> i32 {
        let moves = Moves::generate_all_moves(board, color);
        moves.len() as i32 * 10
    }

    pub fn mobility_balance(board: &Board) -> i32 {
        Self::mobility(board, Color::White) - Self::mobility(board, Color::Black)
    }

    pub fn pawn_structure(board: &Board, color: Color) -> i32 {
        let mut score = 0;
        let (pawns, enemy_pawns) = match color {
            Color::White => (board.white_pawns, board.black_pawns),
            Color::Black => (board.black_pawns, board.white_pawns),
        };

        for i in 0..64 {
            if (pawns & (1 << i)) != 0 {
                // Isolated pawn
                let file = i % 8;
                let mut is_isolated = true;
                if file > 0 && (pawns & (1 << (i - 1))) != 0 {
                    is_isolated = false;
                }
                if file < 7 && (pawns & (1 << (i + 1))) != 0 {
                    is_isolated = false;
                }
                if is_isolated {
                    score -= 30;
                }

                // Doubled pawn
                let rank = i / 8;
                for r in 0..rank {
                    if (pawns & (1 << (r * 8 + file))) != 0 {
                        score -= 20;
                        break;
                    }
                }

                // Passed pawn
                let mut is_passed = true;

                // Check correct direction based on color
                let ranks_to_check: Vec<usize> = match color {
                    Color::White => (rank + 1..8).collect(), // White moves up (toward rank 7)
                    Color::Black => (0..rank).rev().collect(), // Black moves down (toward rank 0)
                };

                for r in ranks_to_check {
                    // Check same file
                    if (enemy_pawns & (1 << (r * 8 + file))) != 0 {
                        is_passed = false;
                        break;
                    }

                    // Check left diagonal file
                    if file > 0 && (enemy_pawns & (1 << (r * 8 + file - 1))) != 0 {
                        is_passed = false;
                        break;
                    }

                    // Check right diagonal file
                    if file < 7 && (enemy_pawns & (1 << (r * 8 + file + 1))) != 0 {
                        is_passed = false;
                        break;
                    }
                }

                if is_passed {
                    score += 30;
                }
            }
        }

        score
    }

    pub fn pawn_structure_balance(board: &Board) -> i32 {
        Self::pawn_structure(board, Color::White) - Self::pawn_structure(board, Color::Black)
    }

    pub fn evaluate(board: &Board) -> i32 {
        let material = Self::material_balance(board);
        let mobility = Self::mobility_balance(board);
        let pawn_structure = Self::pawn_structure_balance(board);

        material + mobility + pawn_structure
    }

    pub fn alpha_beta(
        board: &Board,
        depth: u8,
        alpha: i32,
        beta: i32,
        maximizing_player: bool,
    ) -> i32 {
        if depth == 0 {
            return Self::evaluate(board);
        }

        let color = if maximizing_player {
            Color::White
        } else {
            Color::Black
        };
        let moves = Moves::generate_all_moves(board, color);

        if maximizing_player {
            let mut max_eval = i32::MIN;
            let mut alpha = alpha;

            for mv in moves {
                let mut new_board = board.clone();
                new_board.make_move(&mv);
                let eval = Self::alpha_beta(&new_board, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // Beta cut-off
                }
            }

            max_eval
        } else {
            let mut min_eval = i32::MAX;
            let mut beta = beta;

            for mv in moves {
                let mut new_board = board.clone();
                new_board.make_move(&mv);
                let eval = Self::alpha_beta(&new_board, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // Alpha cut-off
                }
            }

            min_eval
        }
    }
}
