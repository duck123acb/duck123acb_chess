mod board_representation;

fn main() {
    let board = board_representation::Board::new();
    let board_bits = board.all_white_piece_bitboard() | board.all_black_piece_bitboard();

    println!("{:b}", board_bits); // prints
}