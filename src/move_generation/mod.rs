pub mod naive_move_generator;

use crate::board::BitboardBoard;
use crate::board::types::Move;

pub trait BitboardMoveGenerator {
    fn generate_moves(&self, board: &BitboardBoard) -> Vec<Move>;
}
