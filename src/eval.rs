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
        (Self::mobility(board, Color::White) - Self::mobility(board, Color::Black))
    }
    
    
    
}