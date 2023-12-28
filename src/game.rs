pub struct Ply { // a ply is a half-move in Chess. One 
	start_square: String,
	end_square: String,
	captured_piece: char
}

pub struct Board {
pub	white_pawns: u64,
pub	white_rooks: u64,
pub	white_knights: u64,
pub	white_bishops: u64,
pub	white_queens: u64,
pub	white_king: u64,
	
pub	black_pawns: u64,
pub	black_rooks: u64,
pub	black_knights: u64,
pub	black_bishops: u64,
pub	black_queens: u64,
pub	black_king: u64,

pub	whites_turn: bool,
pub	castling_rights: String,
pub	en_passent_coordinates: u64 // to know if there is en passent on the board somewhere
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

	// all the generated moves are for a given occupency. Elsewhere I would need to loop over the 64 squares and precomile this data for a lookup table

 	pub fn generate_sliding_moves(&self, piece_bitboard: u64, occupancy: u64, directions: Vec<i32>) -> Vec<u32>{
		let mut moves = Vec::new();

    for &direction in &directions {
	    let attacks = self.sliding_attacks(piece_bitboard, occupancy, direction);
	    moves.extend(self.get_square_list(attacks));
    }

    moves
	}
	fn sliding_attacks(&self, piece_bitboard: u64, occupancy: u64, direction: i32) -> u64 { // FIXME: assumes every piece is an enemy piece
		let mut attacks = 0;

		for shift in 1..8 {
			let shift_mask = self.get_shift_mask(shift, direction);
			let new_square = piece_bitboard << shift * (direction as u64);

			if new_square >= 0x8000_0000_0000_0000 {
				break;  // Out of board bounds
			}

			attacks |= new_square & shift_mask;

			// Check if the square is occupied by another piece
			if occupancy & new_square != 0 {
				break;
			}
		}

		attacks
	}

	fn get_shift_mask(&self, shift: u64, direction: i32) -> u64 {
		// Helper function to get a shift mask based on direction
		if direction == 1 {
			1 << shift
		} else if direction == -1 {
			1 << (63 - shift)
		} else {
			0
		}
	}

	fn get_square_list(&self, bitboard: u64) -> Vec<u32> {
		// Helper function to get a list of squares from a bitboard
		(0..64).filter(|&square| (bitboard >> square) & 1 != 0).collect()
	}
}