struct Bitboards {
	white_pawns: u64,
	white_rooks: u64,
	white_knights: u64,
	white_bishops: u64,
	white_queens: u64,
	white_king: u64,
	white_all: u64,
	black_pawns: u64,
	black_rooks: u64,
	black_knights: u64,
	black_bishops: u64,
	black_queens: u64,
	black_king: u64,
	black_all: u64,
}

pub struct Info {
	white_can_castle_kingside: bool,
	white_can_castle_queenside: bool,
	black_can_castle_kingside: bool,
	black_can_castle_queenside: bool,
	// game_end: e
}

pub fn load_fen(fen: &str) {
	println!("{}", fen);
}

pub fn update_bitboards() {

}

pub fn make_move(coordinate_one: &str, coordinate_two: &str) {

}
pub fn undo_move() {

}