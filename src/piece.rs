#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn(Color), Knight(Color), Bishop(Color), Rook(Color), Queen(Color), King(Color)   
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White, Black
}

