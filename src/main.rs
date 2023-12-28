mod game;

fn main() {
    let board = game::Board::new();
    let occupancy = board.black_bishops;
    board.generate_sliding_moves(board.white_rooks, occupancy, vec![1, -1]);
}