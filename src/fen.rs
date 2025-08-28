use crate::board::Board;
use crate::piece::{Color, Piece};
use crate::util::*;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_fen(fen: &str) -> Result<Board, String> {
    let mut board = Board::new();

    let parts: [&str; 6] = fen
        .split(' ')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "Invalid FEN format")?;
    let [
        position,
        to_move,
        castling_rights,
        en_passant,
        halfmove_clock,
        fullmove_number,
    ] = parts;

    // Position
    for (rank, row) in position.split('/').rev().enumerate() {
        let mut file = 0;
        for char in row.chars() {
            let square = (rank * 8 + file) as u8;
            match char {
                '1'..='8' => file += char.to_digit(10).unwrap() as usize - 1, // -1 because we add that +1 back to the file for the square
                'p' => board.set_piece(Piece::Pawn, Color::Black, square),
                'n' => board.set_piece(Piece::Knight, Color::Black, square),
                'b' => board.set_piece(Piece::Bishop, Color::Black, square),
                'r' => board.set_piece(Piece::Rook, Color::Black, square),
                'q' => board.set_piece(Piece::Queen, Color::Black, square),
                'k' => board.set_piece(Piece::King, Color::Black, square),
                'P' => board.set_piece(Piece::Pawn, Color::White, square),
                'N' => board.set_piece(Piece::Knight, Color::White, square),
                'B' => board.set_piece(Piece::Bishop, Color::White, square),
                'R' => board.set_piece(Piece::Rook, Color::White, square),
                'Q' => board.set_piece(Piece::Queen, Color::White, square),
                'K' => board.set_piece(Piece::King, Color::White, square),
                _ => {}
            }
            file += 1;
        }
    }

    // To move
    if to_move == "w" {
        board.to_move = true;
    } else if to_move == "b" {
        board.to_move = false;
    } else {
        return Err("Invalid active color in FEN".to_string());
    }

    // Castling rights (e.g. KQk => 0b1110)
    board.castling_rights = 0;
    for char in castling_rights.chars() {
        match char {
            'K' => board.castling_rights |= 0b1000,
            'Q' => board.castling_rights |= 0b0100,
            'k' => board.castling_rights |= 0b0010,
            'q' => board.castling_rights |= 0b0001,
            _ => {}
        }
    }

    // En passant
    if en_passant == "-" {
        board.en_passant = None;
    } else {
        board.en_passant = pos_to_u8(en_passant);
    }

    // Halfmove clock
    board.halfmove_clock = halfmove_clock
        .parse()
        .map_err(|_| "Invalid halfmove clock")?;

    // Fullmove number
    board.fullmove_number = fullmove_number
        .parse()
        .map_err(|_| "Invalid fullmove number")?;

    // Return
    Ok(board)
}

pub fn to_fen(board: &Board) -> String {
    let mut fen = String::new();

    // Pieces
    for rank in (0..8).rev() {
        let mut empty_count = 0;

        for file in 0..8 {
            let square = (rank * 8 + file) as u8;

            if let Some((piece, color)) = board.get_piece_at(square) {
                // Empty square count
                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }

                // Add piece character
                let c = match (piece, color) {
                    (Piece::Pawn, Color::White) => 'P',
                    (Piece::Knight, Color::White) => 'N',
                    (Piece::Bishop, Color::White) => 'B',
                    (Piece::Rook, Color::White) => 'R',
                    (Piece::Queen, Color::White) => 'Q',
                    (Piece::King, Color::White) => 'K',
                    (Piece::Pawn, Color::Black) => 'p',
                    (Piece::Knight, Color::Black) => 'n',
                    (Piece::Bishop, Color::Black) => 'b',
                    (Piece::Rook, Color::Black) => 'r',
                    (Piece::Queen, Color::Black) => 'q',
                    (Piece::King, Color::Black) => 'k',
                };
                fen.push(c);
            } else {
                empty_count += 1;
            }
        }

        // Handle any remaining empty squares at the end of a rank
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }

        // Add rank separator (except after the last rank)
        if rank > 0 {
            fen.push('/');
        }
    }

    // To move
    fen.push(' ');
    if board.to_move {
        fen.push('w');
    } else {
        fen.push('b');
    }

    // Castling rights
    fen.push(' ');
    if board.castling_rights & 0b1000 != 0 {
        fen.push('K');
    }
    if board.castling_rights & 0b0100 != 0 {
        fen.push('Q');
    }
    if board.castling_rights & 0b0010 != 0 {
        fen.push('k');
    }
    if board.castling_rights & 0b0001 != 0 {
        fen.push('q');
    }
    if board.castling_rights == 0 {
        fen.push('-');
    }

    // En passant
    fen.push(' ');
    match board.en_passant {
        None => fen.push('-'),
        Some(square) => fen.push_str(&u8_to_pos(square)),
    }

    // Halfmove clock
    fen.push(' ');
    fen.push_str(&board.halfmove_clock.to_string());

    // Fullmove number
    fen.push(' ');
    fen.push_str(&board.fullmove_number.to_string());

    fen
}
