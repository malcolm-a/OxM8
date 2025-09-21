use crate::piece::Piece;

/// Convert chess position (e.g. "a1", "h8") to a single byte index (0-63)
/// file 'a'-'h' becomes 0-7, rank 1-8 becomes 0-7
/// Formula: (rank - 1) * 8 + file_index
/// Example: "a1" -> (1-1) * 8 + 0 = 0, "h8" -> (8-1) * 8 + 7 = 63
pub fn pos_to_u8(pos: &str) -> Option<u8> {
    let (file, rank) = pos.split_at(1);
    let file = file.chars().next().unwrap();
    let rank = rank.parse::<u8>().ok()?;

    Some((rank - 1) * 8 + (file as u8 - b'a'))
}

pub fn u8_to_pos(square: u8) -> String {
    let file = (square % 8) as u8 + b'a';
    let rank = (square / 8) as u8 + 1;
    format!("{}{}", char::from(file), rank)
}

/// Convert a move to coordinate algebraic notation (e.g., "e2e4", "a7a8=Q")
pub fn move_to_algebraic(from: u8, to: u8, promotion: Option<Piece>) -> String {
    let from_file = (from % 8) as u8 + b'a';
    let from_rank = (from / 8) + 1;
    let to_file = (to % 8) as u8 + b'a';
    let to_rank = (to / 8) + 1;

    let promotion_suffix = match promotion {
        Some(Piece::Queen) => "=Q",
        Some(Piece::Rook) => "=R",
        Some(Piece::Bishop) => "=B",
        Some(Piece::Knight) => "=N",
        _ => "",
    };

    format!(
        "{}{}{}{}{}",
        from_file as char, from_rank, to_file as char, to_rank, promotion_suffix
    )
}

/// Parse coordinate algebraic notation into components (e.g., "e2e4" -> (12, 28, None))
pub fn parse_algebraic(algebraic: &str) -> Option<(u8, u8, Option<Piece>)> {
    if algebraic.len() < 4 {
        return None;
    }

    let from_square = pos_to_u8(&algebraic[0..2])?;
    let to_square = pos_to_u8(&algebraic[2..4])?;

    let promotion = if algebraic.len() >= 6 && &algebraic[4..5] == "=" {
        match algebraic.chars().nth(5)? {
            'Q' => Some(Piece::Queen),
            'R' => Some(Piece::Rook),
            'B' => Some(Piece::Bishop),
            'N' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };

    Some((from_square, to_square, promotion))
}

/// Get the file (column) of a square (0-7, where 0 is 'a' file)
pub fn get_file(square: u8) -> u8 {
    square % 8
}

/// Get the rank (row) of a square (0-7, where 0 is rank 1)
pub fn get_rank(square: u8) -> u8 {
    square / 8
}

/// Check if two squares are on the same file
pub fn same_file(square1: u8, square2: u8) -> bool {
    get_file(square1) == get_file(square2)
}

/// Check if two squares are on the same rank
pub fn same_rank(square1: u8, square2: u8) -> bool {
    get_rank(square1) == get_rank(square2)
}

/// Check if two squares are on the same diagonal
pub fn same_diagonal(square1: u8, square2: u8) -> bool {
    let file_diff = (get_file(square1) as i8 - get_file(square2) as i8).abs();
    let rank_diff = (get_rank(square1) as i8 - get_rank(square2) as i8).abs();
    file_diff == rank_diff && file_diff != 0
}

/// Get the distance between two squares (Manhattan distance)
pub fn manhattan_distance(square1: u8, square2: u8) -> u8 {
    let file_diff = (get_file(square1) as i8 - get_file(square2) as i8).abs() as u8;
    let rank_diff = (get_rank(square1) as i8 - get_rank(square2) as i8).abs() as u8;
    file_diff + rank_diff
}

/// Get the maximum distance between two squares (Chebyshev distance)
pub fn king_distance(square1: u8, square2: u8) -> u8 {
    let file_diff = (get_file(square1) as i8 - get_file(square2) as i8).abs() as u8;
    let rank_diff = (get_rank(square1) as i8 - get_rank(square2) as i8).abs() as u8;
    file_diff.max(rank_diff)
}

/// Convert standard algebraic notation to coordinate notation
/// Examples: "Nf3" -> "g1f3", "Ke2" -> "e1e2", "Qxd5" -> "d1d5"
pub fn algebraic_to_coordinate(
    algebraic: &str,
    board: &crate::board::Board,
    color: crate::piece::Color,
) -> Option<String> {
    use crate::moves::Moves;
    use crate::piece::Piece;

    let algebraic = algebraic.trim();

    // Handle castling
    match algebraic.to_lowercase().as_str() {
        "o-o" | "0-0" => {
            return match color {
                crate::piece::Color::White => Some("e1g1".to_string()),
                crate::piece::Color::Black => Some("e8g8".to_string()),
            };
        }
        "o-o-o" | "0-0-0" => {
            return match color {
                crate::piece::Color::White => Some("e1c1".to_string()),
                crate::piece::Color::Black => Some("e8c8".to_string()),
            };
        }
        _ => {}
    }

    // Parse the move
    let mut chars: Vec<char> = algebraic.chars().collect();
    if chars.is_empty() {
        return None;
    }

    // Remove check/checkmate symbols
    if chars.last() == Some(&'+') || chars.last() == Some(&'#') {
        chars.pop();
    }

    // Parse promotion (e.g., "e8=Q")
    let promotion = if chars.len() >= 3 && chars[chars.len() - 2] == '=' {
        let promo_char = chars[chars.len() - 1];
        chars.drain(chars.len() - 2..);
        match promo_char {
            'Q' => Some(Piece::Queen),
            'R' => Some(Piece::Rook),
            'B' => Some(Piece::Bishop),
            'N' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };

    // Determine piece type and target square
    let (piece_type, target_square, disambiguation) = if chars.len() >= 2
        && chars[chars.len() - 2].is_ascii_lowercase()
        && chars[chars.len() - 1].is_ascii_digit()
    {
        // Extract target square (last 2 characters)
        let target_file = chars[chars.len() - 2];
        let target_rank = chars[chars.len() - 1];
        let target_str = format!("{}{}", target_file, target_rank);
        let target_square = pos_to_u8(&target_str)?;

        // Remove target square from consideration
        chars.drain(chars.len() - 2..);

        // Remove capture symbol if present
        if chars.last() == Some(&'x') {
            chars.pop();
        }

        // Determine piece and disambiguation
        if chars.is_empty() {
            // Pawn move (no piece symbol)
            (Piece::Pawn, target_square, None)
        } else {
            // First character is piece type
            let piece_char = chars[0];
            let piece = match piece_char {
                'K' => Piece::King,
                'Q' => Piece::Queen,
                'R' => Piece::Rook,
                'B' => Piece::Bishop,
                'N' => Piece::Knight,
                _ => return None, // Invalid piece
            };

            // Rest is disambiguation (file or rank)
            let disambiguation = if chars.len() > 1 {
                Some(chars[1..].iter().collect::<String>())
            } else {
                None
            };

            (piece, target_square, disambiguation)
        }
    } else {
        return None;
    };

    // Find all legal moves that match the criteria
    let legal_moves = Moves::generate_legal_moves(board, color);
    let mut matching_moves = Vec::new();

    for mv in legal_moves {
        // Check if this move goes to the target square
        if mv.to != target_square {
            continue;
        }

        // Check if the piece type matches
        if let Some((piece, _)) = board.get_piece_at(mv.from) {
            if piece != piece_type {
                continue;
            }
        } else {
            continue;
        }

        // Check promotion
        match (promotion, &mv.move_type) {
            (Some(promo_piece), crate::moves::MoveType::Promotion { piece })
            | (Some(promo_piece), crate::moves::MoveType::PromotionCapture { piece }) => {
                if promo_piece != *piece {
                    continue;
                }
            }
            (None, crate::moves::MoveType::Promotion { .. })
            | (None, crate::moves::MoveType::PromotionCapture { .. }) => {
                continue; // Promotion expected but not specified
            }
            (Some(_), _) => continue, // Promotion specified but move is not promotion
            (None, _) => {}           // No promotion, any non-promotion move is fine
        }

        matching_moves.push(mv);
    }

    // Handle disambiguation
    if let Some(ref disambig) = disambiguation {
        matching_moves.retain(|mv| {
            let from_file = (mv.from % 8) as u8 + b'a';
            let from_rank = (mv.from / 8) + 1;

            if disambig.len() == 1 {
                let disambig_char = disambig.chars().next().unwrap();
                if disambig_char.is_ascii_lowercase() {
                    // File disambiguation
                    from_file == disambig_char as u8
                } else if disambig_char.is_ascii_digit() {
                    // Rank disambiguation
                    from_rank == disambig_char.to_digit(10).unwrap() as u8
                } else {
                    false
                }
            } else if disambig.len() == 2 {
                // Full square disambiguation
                let from_square_str = format!("{}{}", from_file as char, from_rank);
                from_square_str == *disambig
            } else {
                false
            }
        });
    }

    // Should have exactly one matching move
    if matching_moves.len() == 1 {
        let mv = matching_moves[0];
        let from_str = u8_to_pos(mv.from);
        let to_str = u8_to_pos(mv.to);
        Some(format!("{}{}", from_str, to_str))
    } else {
        None // Ambiguous or no legal move found
    }
}
