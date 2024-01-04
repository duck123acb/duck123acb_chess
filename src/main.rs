mod game;

fn main() {
    let board = game::Board::new();
    board.generate_sliding_moves(board.bitboards.white_rooks, true, false); // TODO: change piece_bitboard input

    let x = 65280u64;

    let mut i = 0;
    let mut zeroes = "".to_string();
    while i < x.leading_zeros() {
        zeroes += "0";
        i += 1;
    }
    println!("{}{:b}", zeroes, x);
}