#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King  
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White, Black
}

pub fn piece_to_char(piece: Piece, color: Color) -> char {
    match (piece, color) {
        (Piece::Pawn, Color::White) => 'P',
        (Piece::Pawn, Color::Black) => 'p',
        (Piece::Knight, Color::White) => 'N',
        (Piece::Knight, Color::Black) => 'n',
        (Piece::Bishop, Color::White) => 'B',
        (Piece::Bishop, Color::Black) => 'b',
        (Piece::Rook, Color::White) => 'R',
        (Piece::Rook, Color::Black) => 'r',
        (Piece::Queen, Color::White) => 'Q',
        (Piece::Queen, Color::Black) => 'q',
        (Piece::King, Color::White) => 'K',
        (Piece::King, Color::Black) => 'k',
    }
}

pub fn piece_to_sp_char(piece: Piece, color: Color) -> char {
    match (piece, color) {
        (Piece::Pawn, Color::White) => '♙',
        (Piece::Pawn, Color::Black) => '♟',
        (Piece::Knight, Color::White) => '♘',
        (Piece::Knight, Color::Black) => '♞',
        (Piece::Bishop, Color::White) => '♗',
        (Piece::Bishop, Color::Black) => '♝',
        (Piece::Rook, Color::White) => '♖',
        (Piece::Rook, Color::Black) => '♜',
        (Piece::Queen, Color::White) => '♕',
        (Piece::Queen, Color::Black) => '♛',
        (Piece::King, Color::White) => '♔',
        (Piece::King, Color::Black) => '♚',
    }
}

pub fn piece_to_sp_string(piece: Piece, color: Color) -> String {
    match (piece, color) {
        (Piece::Pawn, Color::White) => "White Pawn".to_string(),
        (Piece::Pawn, Color::Black) => "Black Pawn".to_string(),
        (Piece::Knight, Color::White) => "White Knight".to_string(),
        (Piece::Knight, Color::Black) => "Black Knight".to_string(),
        (Piece::Bishop, Color::White) => "White Bishop".to_string(),
        (Piece::Bishop, Color::Black) => "Black Bishop".to_string(),
        (Piece::Rook, Color::White) => "White Rook".to_string(),
        (Piece::Rook, Color::Black) => "Black Rook".to_string(),
        (Piece::Queen, Color::White) => "White Queen".to_string(),
        (Piece::Queen, Color::Black) => "Black Queen".to_string(),
        (Piece::King, Color::White) => "White King".to_string(),
        (Piece::King, Color::Black) => "Black King".to_string(),
    }
}
