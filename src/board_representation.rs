//TODO: #[allow(dead_code)] when done, for now it's still useful
pub struct Move {
	start_square: u64,
	end_square: u64,
	captured_piece: Option<PieceType>,
	promoted_piece: Option<PieceType>,
	en_passent: bool,
	castling: Option<CastlingType>
}

#[derive(Copy, Clone)]
pub enum PieceType {
	WhitePawn,
	WhiteRook,
	WhiteKnight,
	WhiteBishop,
	WhiteQueen,
	WhiteKing,
	BlackPawn,
	BlackRook,
	BlackKnight,
	BlackBishop,
	BlackQueen,
	BlackKing
}
impl PieceType {
	fn iter() -> impl Iterator<Item = PieceType> {
		// Define an array containing all enum variants
		const PIECES: [PieceType; 12] = [
			PieceType::WhitePawn,
			PieceType::WhiteRook,
			PieceType::WhiteKnight,
			PieceType::WhiteBishop,
			PieceType::WhiteQueen,
			PieceType::WhiteKing,
			PieceType::BlackPawn,
			PieceType::BlackRook,
			PieceType::BlackKnight,
			PieceType::BlackBishop,
			PieceType::BlackQueen,
			PieceType::BlackKing
		];

		// Return an iterator over the array
		PIECES.into_iter()
	}
}

enum CastlingType {
	Kingside,
	Queenside
}

pub struct Board {
	pub bitboards: [u64; 12],

	pub whites_turn: bool,
	pub castling_rights: String,
	pub en_passent_coordinates: u64 // to know if there is en passent on the board somewhere
}
impl Board {
	pub fn new() -> Board {

		return Board { // starting properties
			bitboards: [
				65280, // white pawns
				129, // white rooks
				66, // white knights
				36, // white bishops
				16, // white queens
				8, // white king

				71776119061217280, // black pawns
				9295429630892703744, // black rooks
				4755801206503243776, // black knights
				2594073385365405696, // black bishops
				1152921504606846976, // black queens
				576460752303423488 // black king
			],

			whites_turn: true,
			castling_rights: "WQwq".to_string(),
			en_passent_coordinates: 0
		}
	}

	pub fn all_white_piece_bitboard(&self) -> u64 {
		self.bitboards[PieceType::WhitePawn as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteKing as usize]
	}
	pub fn all_black_piece_bitboard(&self) -> u64 {
		self.bitboards[PieceType::BlackPawn as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackKing as usize]
	}

	
	pub fn make_move(&self, ply: Move) {
		/*
		move piece on square one to square two

		if there is a piece being captured
			remove that piece on the bitboard
		
		if the king is castling
			move rook as well

		if the move is a promotion
			add the promoting piece to the respective bitboard
			remove pawn from pawn bitboard
		 */
	}
	pub fn undo_move(&self, ply: Move) { // assumes that the lastest move played is the one we are undoing

	}

	// all the generated moves are for a given square. Elsewhere I would need to loop over the 64 squares and precomile this data for a lookup table
 	pub fn generate_sliding_moves(&self, piece_bitboard: u64, orthagonal: bool, diagonal: bool) -> Vec<Move>{
		let mut moves = Vec::new();
		let is_piece_white = piece_bitboard & self.all_white_piece_bitboard() != 0;
		let friendly_bitboard = if is_piece_white { self.all_white_piece_bitboard() } else { self.all_black_piece_bitboard() };
		let enemy_bitboard = if is_piece_white { self.all_black_piece_bitboard() } else { self.all_white_piece_bitboard() };

		if orthagonal {
			const ORTHAGONAL_DIRECTIONS: [i32; 4] = [1, -1, 8, -1]; // 1 is left, -1 is right, 8 is up, -8 is down
			for &direction in &ORTHAGONAL_DIRECTIONS {
				let attacks = self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
				moves.extend(self.get_move_list(piece_bitboard, attacks));
			}
		}
		if diagonal {
			const DIAGONAL_DIRECTIONS: [i32; 4] = [9, -9, 7, -7]; // 7 is up-right, -7 is down-right, 9 is up-left, -9 is down-left
			for &direction in &DIAGONAL_DIRECTIONS {
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
		let mut temp_bitboard = piece_attacks;
		let mut moves: Vec<Move> = vec![];
		
		while temp_bitboard != 0 {
			let mut piece_move = Move {
				start_square: piece_bitboard,
				end_square: 1 << temp_bitboard.trailing_zeros(), // the least signifigant bit,
				captured_piece: None,
				promoted_piece: None, // TODO: handle promotions
				en_passent: false, // TODO: handle en passent,
				castling: None // TODO: handle castling
			};

			for piece_type in PieceType::iter() { // get captured piece
				if self.bitboards[piece_type as usize] & piece_move.end_square != 0 {
					piece_move.captured_piece = Some(piece_type);
					break;
				}
			}

			moves.push(piece_move);

			temp_bitboard &= temp_bitboard - 1; // clear the least significant bit for the next iteration
		}

		moves
	}
}