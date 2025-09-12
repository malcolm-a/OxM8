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
