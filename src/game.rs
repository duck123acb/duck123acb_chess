//TODO: #[allow(dead_code)] when done, for now it's still useful
pub struct Ply { // a ply is a half-move in Chess. One 
	start_square: String,
	end_square: String,
	captured_piece: char
}

pub struct Board {
	pub white_pawns: u64,
	pub white_rooks: u64,
	pub white_knights: u64,
	pub white_bishops: u64,
	pub white_queens: u64,
	pub white_king: u64,
	
	pub black_pawns: u64,
	pub black_rooks: u64,
	pub black_knights: u64,
	pub black_bishops: u64,
	pub black_queens: u64,
	pub black_king: u64,

	pub whites_turn: bool,
	pub castling_rights: String,
	pub en_passent_coordinates: u64 // to know if there is en passent on the board somewhere
}

impl Board {
	pub fn new() -> Board {
		return Board { // starting properties
			white_pawns: 65280,
			white_rooks: 129,
			white_knights: 66,
			white_bishops: 36,
			white_queens: 16,
			white_king: 8,

			black_pawns: 71776119061217280,
			black_rooks: 9295429630892703744,
			black_knights: 4755801206503243776,
			black_bishops: 2594073385365405696,
			black_queens: 1152921504606846976,
			black_king: 576460752303423488,
			
			whites_turn: true,
			castling_rights: "WQwq".to_string(),
			en_passent_coordinates: 0
		}
	}

	
	pub fn make_move(&self, ply: Ply) {
	
	}
	pub fn undo_move(&self, ply: Ply) { // assumes that the lastest move played is the one we are undoing

	}

	pub fn all_white_piece_bitboard(&self) -> u64 {
		self.white_pawns | self.white_rooks | self.white_knights | self.white_bishops | self.white_queens | self.white_king
	}
	pub fn all_black_piece_bitboard(&self) -> u64 {
		self.black_pawns | self.black_rooks | self.black_knights | self.black_bishops | self.black_queens | self.black_king
	}

	// all the generated moves are for a given square. Elsewhere I would need to loop over the 64 squares and precomile this data for a lookup table
 	pub fn generate_sliding_moves(&self, piece_bitboard: u64, orthagonal: bool, diagonal: bool) -> Vec<u32>{
		let mut moves = Vec::new();
		let is_piece_white = piece_bitboard & self.all_white_piece_bitboard() != 0;
		let friendly_bitboard = if is_piece_white { self.all_white_piece_bitboard() } else { self.all_black_piece_bitboard() };
		let enemy_bitboard = if is_piece_white { self.all_black_piece_bitboard() } else { self.all_white_piece_bitboard() };

		if orthagonal {
			let orthagonal_directions = [1, -1, 8, -1]; // 1 is left, -1 is right, 8 is up, -8 is down
			for &direction in &orthagonal_directions {
				let attacks = self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
				moves.extend(self.get_square_list(attacks));
			}
		}
		if diagonal {
			let diagonal_directions = [9, -9, 7, -7]; // 7 is up-right, -7 is down-right, 9 is up-left, -9 is down-left
			for &direction in &diagonal_directions {
				let attacks = self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
				moves.extend(self.get_square_list(attacks));
			}
		}

    moves
	}
	fn attacks_in_a_direction(&self, piece_bitboard: u64, friendly_occupency: u64, enemy_occupancy: u64, direction: i32) -> u64 {
		let mut attacks = 0;

		for shift in 1..8 {
			let new_square = piece_bitboard << shift * (direction as u64); // FIXME: sometimes when shifting, it overflows. This happens when we go out of bounds of the board; this in turn creates a number larger than 64 bits

			if friendly_occupency & new_square != 0 { // check
				break;
			}

			attacks |= new_square;
			
			if enemy_occupancy & new_square != 0 { // check if the square is occupied by an enemy piece. Enemy pieces are capturable
				break;
			}
		}

		attacks
	}

	// TODO: move helper functions into a bitboard_utilty module
	fn get_shift_mask(&self, shift: u64, direction: i32) -> u64 {
		// helper function to get a shift mask based on direction
		if direction == 1 {
			1 << shift
		} else if direction == -1 {
			1 << (63 - shift)
		} else {
			0
		}
	}
	fn get_square_list(&self, bitboard: u64) -> Vec<u32> {
		// helper function to get a list of squares from a bitboard
		(0..64).filter(|&square| (bitboard >> square) & 1 != 0).collect()
	}
}