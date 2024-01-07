//TODO: #[allow(dead_code)] when done, for now it's still useful
pub struct Move { // TODO: change MovePiece struct to involve integers
	start_square: String,
	end_square: String,
	captured_piece: Option<Piece>,
	promoted_piece: Option<Piece>,
	en_passent: bool,
	castling: Option<CastlingType>

	/*
	
		source_square: u8,
    destination_square: u8,
    promotion_piece: u8,
    special_flags: u8
		
		*/
}

enum Piece {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}
enum Colour {
	
}
enum CastlingType {
	Kingside,
	Queenside
}

pub struct Bitboards {
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
	pub black_king: u64
}
impl Bitboards {
	pub fn all_white_piece_bitboard(&self) -> u64 {
		self.white_pawns | self.white_rooks | self.white_knights | self.white_bishops | self.white_queens | self.white_king
	}
	pub fn all_black_piece_bitboard(&self) -> u64 {
		self.black_pawns | self.black_rooks | self.black_knights | self.black_bishops | self.black_queens | self.black_king
	}
}

pub struct Board {
	pub bitboards: Bitboards,

	pub whites_turn: bool,
	pub castling_rights: String,
	pub en_passent_coordinates: u64 // to know if there is en passent on the board somewhere
}
impl Board {
	pub fn new() -> Board {


		return Board { // starting properties
			bitboards: Bitboards {
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
			},
			
			whites_turn: true,
			castling_rights: "WQwq".to_string(),
			en_passent_coordinates: 0
		}
	}

	
	pub fn make_move(&self, ply: Move) {
		
	}
	pub fn undo_move(&self, ply: Move) { // assumes that the lastest move played is the one we are undoing

	}

	// fn get_piece_type(&self, coordinate: &str) -> char{
		
	// }

	// fn coordinate_to_bit(&self, coordinate: &str) -> u64 {
		
	// }
	fn bit_to_coordinate(&self, bitboard: u64) -> &str {
		"asdf" // TODO: make function 🗿
	}

	// all the generated moves are for a given square. Elsewhere I would need to loop over the 64 squares and precomile this data for a lookup table
 	pub fn generate_sliding_moves(&self, piece_bitboard: u64, orthagonal: bool, diagonal: bool) -> Vec<Move>{
		let mut moves = Vec::new();
		let is_piece_white = piece_bitboard & self.bitboards.all_white_piece_bitboard() != 0;
		let friendly_bitboard = if is_piece_white { self.bitboards.all_white_piece_bitboard() } else { self.bitboards.all_black_piece_bitboard() };
		let enemy_bitboard = if is_piece_white { self.bitboards.all_black_piece_bitboard() } else { self.bitboards.all_white_piece_bitboard() };

		if orthagonal {
			let orthagonal_directions = [1, -1, 8, -1]; // 1 is left, -1 is right, 8 is up, -8 is down
			for &direction in &orthagonal_directions {
				let attacks = self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
				moves.extend(self.get_move_list(piece_bitboard, attacks));
			}
		}
		if diagonal {
			let diagonal_directions = [9, -9, 7, -7]; // 7 is up-right, -7 is down-right, 9 is up-left, -9 is down-left
			for &direction in &diagonal_directions {
				let attacks = self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
				moves.extend(self.get_move_list(piece_bitboard, attacks));
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

	fn get_move_list(&self, piece_bitboard: u64, piece_attacks: u64) -> Vec<Move> {
		let piece_coordinate = self.bit_to_coordinate(piece_bitboard);
		let mut temp_bitboard = piece_attacks;
		let mut moves: Vec<Move> = vec![];
		
		while temp_bitboard != 0 {
			let isolated_bit = 1 << temp_bitboard.trailing_zeros();

			let piece_move = Move { 
				start_square: piece_coordinate.to_string(),
				end_square: self.bit_to_coordinate(isolated_bit).to_string(),
				captured_piece: None, // TODO: handle piece captures
				promoted_piece: None, // TODO: handle promotions
				en_passent: false, // TODO: handle en passent,
				castling: None // TODO: handle castling
			};
			moves.push(piece_move);

			temp_bitboard &= temp_bitboard - 1; // clear the least significant set bit for the next iteration
		}

		moves
	}
}