mod board_representation;

fn main() {
    let board = board_representation::Board::new();

    board.generate_sliding_moves(34359738368u64, true, false); // TODO: use a bit that is actually in the bitboards
}