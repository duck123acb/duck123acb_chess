mod board_representation;

fn main() {
    let board = board_representation::Board::new();
    // println!("{:b}", board_bits);

    board.generate_sliding_moves(0x0000000000000800, true, false);
}