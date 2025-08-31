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
    
    pub fn new() -> Self {
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
    
    pub fn get_bb(&self, piece: Piece, color: Color) -> u64 {
        match (piece, color) {
            (Piece::Pawn, Color::White) => self.white_pawns,
            (Piece::Knight, Color::White) => self.white_knights,
            (Piece::Bishop, Color::White) => self.white_bishops,
            (Piece::Rook, Color::White) => self.white_rooks,
            (Piece::Queen, Color::White) => self.white_queens,
            (Piece::King, Color::White) => self.white_king,
            (Piece::Pawn, Color::Black) => self.black_pawns,
            (Piece::Knight, Color::Black) => self.black_knights,
            (Piece::Bishop, Color::Black) => self.black_bishops,
            (Piece::Rook, Color::Black) => self.black_rooks,
            (Piece::Queen, Color::Black) => self.black_queens,
            (Piece::King, Color::Black) => self.black_king,
        }
    }
    
    fn get_bb_mut(&mut self, piece: Piece, color: Color) -> &mut u64 {
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
        let bitboard = self.get_bb_mut(piece, color);
        *bitboard |= 1 << square;
    }
    
    pub fn remove_piece(&mut self, piece: Piece, color: Color, square: u8) {
        let bitboard = self.get_bb_mut(piece, color);
        *bitboard &= !(1 << square);
    }
    
    pub fn get_piece_at(&self, square: u8) -> Option<(Piece, Color)> {
        if self.white_pawns & (1 << square) != 0 { return Some((Piece::Pawn, Color::White)); }
        if self.white_knights & (1 << square) != 0 { return Some((Piece::Knight, Color::White)); }
        if self.white_bishops & (1 << square) != 0 { return Some((Piece::Bishop, Color::White)); }
        if self.white_rooks & (1 << square) != 0 { return Some((Piece::Rook, Color::White)); }
        if self.white_queens & (1 << square) != 0 { return Some((Piece::Queen, Color::White)); }
        if self.white_king & (1 << square) != 0 { return Some((Piece::King, Color::White)); }
        if self.black_pawns & (1 << square) != 0 { return Some((Piece::Pawn, Color::Black)); }
        if self.black_knights & (1 << square) != 0 { return Some((Piece::Knight, Color::Black)); }
        if self.black_bishops & (1 << square) != 0 { return Some((Piece::Bishop, Color::Black)); }
        if self.black_rooks & (1 << square) != 0 { return Some((Piece::Rook, Color::Black)); }
        if self.black_queens & (1 << square) != 0 { return Some((Piece::Queen, Color::Black)); }
        if self.black_king & (1 << square) != 0 { return Some((Piece::King, Color::Black)); }
        None
    }
    
}
