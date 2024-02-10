mod board_representation;

fn main() {
    let board = board_representation::Board::new();
    println!("{:064b}", 524288u64);

    board.generate_sliding_moves(524288, true, false);
}