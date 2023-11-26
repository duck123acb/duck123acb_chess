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
struct CastlingRights {
	white_kingside: bool,
	white_queenside: bool,
	black_kingside: bool,
	black_queenside: bool,
}

pub struct Info {
	bitboards: Bitboards,
	whites_turn: bool,
	castling_rights: CastlingRights,
	en_passent_coordinate: String, // to know if there is en passent
	moves_since_capture_or_pawn_push: u32, // for the 50 move rule
	turns: u32 // every white and black move
}

fn update_bitboards() {

}

pub fn load_fen(fen: &str) {
	let fen_fields: Vec<&str> = fen.split(' ').collect(); // 0 = piece_arrangement, 1 = who's turn, 2 = castling rights, 3 = any en_passent squares, 4 = moves countine toward the 50 move rule, 5 = how many turns has happened in a game (a turn is white goes, and black goes)
	println!("{}, {}, {}, {}, {}, {}", fen_fields[0], fen_fields[1], fen_fields[2], fen_fields[3], fen_fields[4], fen_fields[5]);
}

pub fn make_move(coordinate_one: &str, coordinate_two: &str) {

}
pub fn undo_move() {

}