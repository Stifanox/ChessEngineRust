use crate::board::{BitboardBoard, Board};
use crate::board::types::{Piece, Square};
use crate::move_generation::BitboardMoveGenerator;
use crate::move_generation::naive_move_generator::NaiveMoveGenerator;
use crate::utils::bitboards;
use crate::utils::bitboards::utils::utils::print_board_state;

mod board;
mod utils;
mod move_generation;
mod tests;

fn main() {
    let mut board = BitboardBoard::new_startpos();
    let naive_generation = NaiveMoveGenerator::new();
    board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let moves = naive_generation.generate_moves(&board);
    print_board_state(&board)
}
