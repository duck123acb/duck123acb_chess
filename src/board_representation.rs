// pub struct Move {
// 	start_square: u64,
// 	end_square: u64,
// 	captured_piece: Option<PieceType>,
// 	promoted_piece: Option<PieceType>,
// 	en_passent: bool,
// 	castling: Option<CastlingType>
// }

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

// enum CastlingType {
// 	Kingside,
// 	Queenside
// }

pub struct Board {
	pub bitboards: [u64; 12],

	pub whites_turn: bool,
	pub castling_rights: String, // TODO: when implementing castling, fix this type
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

	// TODO: do we need this to be public?
	pub fn all_white_piece_bitboard(&self) -> u64 {
		self.bitboards[PieceType::WhitePawn as usize] | self.bitboards[PieceType::WhiteRook as usize] | self.bitboards[PieceType::WhiteKnight as usize] | self.bitboards[PieceType::WhiteBishop as usize] | self.bitboards[PieceType::WhiteQueen as usize] | self.bitboards[PieceType::WhiteKing as usize]
	}
	pub fn all_black_piece_bitboard(&self) -> u64 {
		self.bitboards[PieceType::BlackPawn as usize] | self.bitboards[PieceType::BlackRook as usize] | self.bitboards[PieceType::BlackKnight as usize] | self.bitboards[PieceType::BlackBishop as usize] | self.bitboards[PieceType::BlackQueen as usize] | self.bitboards[PieceType::BlackKing as usize]
	}

	// pub fn load_fen(fen: String) {
	// 	// idk do something??
	// }

	// pub fn make_move(&self, ply: Move) {
	// 	/*
	// 	move piece on square one to square two

	// 	if there is a piece being captured
	// 		remove that piece on the bitboard
		
	// 	if the king is castling
	// 		move rook as well

	// 	if the move is a promotion
	// 		add the promoting piece to the respective bitboard
	// 		remove pawn from pawn bitboard
	// 	 */
	// }
	// pub fn undo_move(&self, ply: Move) { // assumes that the lastest move played is the one we are undoing

	// }

	/* SLIDING PIECE MOVE GEN */
	// all the generated moves are for a given square. Elsewhere I would need to loop over the 64 squares and precompile this data for a lookup table
 	pub fn generate_sliding_moves(&self, piece_bitboard: u64, white_bitboard: u64, black_bitboard: u64, orthagonal: bool, diagonal: bool) -> u64{
		let mut movement_mask = 0;
		let is_piece_white = piece_bitboard & white_bitboard != 0; // to determine friends and enemies
		println!("{}", is_piece_white);
		let friendly_bitboard = if is_piece_white {
			white_bitboard
		} else { 
			black_bitboard
		};
		let enemy_bitboard = if is_piece_white {
			white_bitboard
		} else {
			black_bitboard
		};

		if orthagonal {
			const ORTHAGONAL_DIRECTIONS: [i32; 4] = [1, -1, 8, -8]; // 1 is left, -1 is right, 8 is up, -8 is down
			for &direction in &ORTHAGONAL_DIRECTIONS {
				movement_mask |= self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
			}
		}
		if diagonal {
			const DIAGONAL_DIRECTIONS: [i32; 4] = [9, -9, 7, -7]; // 7 is up-right, -7 is down-right, 9 is up-left, -9 is down-left
			for &direction in &DIAGONAL_DIRECTIONS {
				movement_mask |= self.attacks_in_a_direction(piece_bitboard, friendly_bitboard, enemy_bitboard, direction);
			}
		}

    movement_mask
	}
	fn attacks_in_a_direction(&self, piece_bitboard: u64, friendly_occupency: u64, enemy_occupancy: u64, direction: i32) -> u64 { // returns a bitboard with all the squares it attacks
		let mut attacks = 0;

		for shift in 1..8 {
			let new_square = if direction > 0 {
				piece_bitboard << shift * direction
			} else {
				piece_bitboard >> shift * (direction * -1)
			};

			if friendly_occupency & new_square != 0 { // stop the search before adding a capture of a friendly piece
				break;
			}

			attacks |= new_square;
			
			if enemy_occupancy & new_square != 0 { // stop the search after adding a capture of an enemy piece
				break;
			}

			if new_square & 0x8181818181818181 != 0 || new_square & 0xFF000000000000FF != 0 { // stop the search if the new square is on the edge of the board
				break;
			}
		}

		attacks
	}
}