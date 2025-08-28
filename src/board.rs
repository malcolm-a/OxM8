use crate::piece::{Piece, Color};

pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub to_move: bool,          // true for white
    pub halfmove_clock: u16,    
    pub fullmove_number: u16,   
    pub en_passant: Option<u8>, // index 0..63
    pub castling_rights: u8,    // 4 bits: KQkq
}

impl Board {
    
    fn new() -> Self {
        Self {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,

            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,

            to_move: true,
            halfmove_clock: 0,
            fullmove_number: 1,
            en_passant: None,
            castling_rights: 0b1111,
        }
    }
    
    fn bb_mut(&mut self, piece: Piece, color: Color) -> &mut u64 {
        match (piece, color) {
            (Piece::Pawn, Color::White) => &mut self.white_pawns,
            (Piece::Knight, Color::White) => &mut self.white_knights,
            (Piece::Bishop, Color::White) => &mut self.white_bishops,
            (Piece::Rook, Color::White) => &mut self.white_rooks,
            (Piece::Queen, Color::White) => &mut self.white_queens,
            (Piece::King, Color::White) => &mut self.white_king,
            (Piece::Pawn, Color::Black) => &mut self.black_pawns,
            (Piece::Knight, Color::Black) => &mut self.black_knights,
            (Piece::Bishop, Color::Black) => &mut self.black_bishops,
            (Piece::Rook, Color::Black) => &mut self.black_rooks,
            (Piece::Queen, Color::Black) => &mut self.black_queens,
            (Piece::King, Color::Black) => &mut self.black_king,
        }
    }

    pub fn set_piece(&mut self, piece: Piece, color: Color, square: u8) {
        let bitboard = self.bb_mut(piece, color);
        *bitboard |= 1 << square;
    }
    
    pub fn remove_piece(&mut self, piece: Piece, color: Color, square: u8) {
        let bitboard = self.bb_mut(piece, color);
        *bitboard &= !(1 << square);
    }
    
    pub fn set_en_passant(&mut self, square: Option<u8>) {
        self.en_passant = square;
    }
    
    pub fn set_castling_rights(&mut self, rights: u8) {
        self.castling_rights = rights;
    }
    
}
