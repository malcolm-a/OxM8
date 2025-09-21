use crate::board::Board;
use crate::piece::{Piece, Color};
use crate::moves::{Moves, MoveType};

const PAWN_VALUE: i8 = 1;
const KNIGHT_VALUE: i8 = 3;
const BISHOP_VALUE: i8 = 3;
const ROOK_VALUE: i8 = 5;
const QUEEN_VALUE: i8 = 9;
const KING_VALUE: i8 = 0;

pub struct Eval {
    
}

impl Eval {
    pub fn match_piece_value(piece: Piece) -> i8 {
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
                board.white_pawns.count_ones() as i32 * PAWN_VALUE as i32
                    + board.white_knights.count_ones() as i32 * KNIGHT_VALUE as i32
                    + board.white_bishops.count_ones() as i32 * BISHOP_VALUE as i32
                    + board.white_rooks.count_ones() as i32 * ROOK_VALUE as i32
                    + board.white_queens.count_ones() as i32 * QUEEN_VALUE as i32
            }
            Color::Black => {
                board.black_pawns.count_ones() as i32 * PAWN_VALUE as i32
                    + board.black_knights.count_ones() as i32 * KNIGHT_VALUE as i32
                    + board.black_bishops.count_ones() as i32 * BISHOP_VALUE as i32
                    + board.black_rooks.count_ones() as i32 * ROOK_VALUE as i32
                    + board.black_queens.count_ones() as i32 * QUEEN_VALUE as i32
            }
        }
    }

    pub fn material_balance(board: &Board) -> i32 {
        Self::material(board, Color::White) - Self::material(board, Color::Black)
    }
    
    pub fn mobility(board: &Board, color: Color) -> i32 {
        let moves = Moves::generate_all_moves(board, color);
        moves.len() / 10 as i32
    }
    
    pub fn mobility_balance(board: &Board) -> i32 {
        (Self::mobility(board, Color::White) - Self::mobility(board, Color::Black))
    }
    
    
    
}