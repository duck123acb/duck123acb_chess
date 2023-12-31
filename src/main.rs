mod game;

fn main() {
    let board = game::Board::new();
    // let rook = 
    let occupancy = board.all_black_piece_bitboard() | (board.all_white_piece_bitboard() );
    board.generate_sliding_moves(board.white_rooks, occupancy, vec![1, -1]);


}