mod naive_move_generator;

use crate::board::BitboardBoard;
use crate::board::types::Move;

pub trait MoveGenerator {
    fn generate_moves(&self, board: &BitboardBoard) -> Vec<Move>;
}
