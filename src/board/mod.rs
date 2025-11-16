pub mod bitboard;
pub mod types;

use types::{Piece, Square, Move};

pub trait Board {
    fn piece_at(&self, square: Square) -> Option<Piece>;
    fn get_all_pieces(&self) -> Vec<(Square, Piece)>;

    fn generate_moves(&self) -> Vec<Move>;
    fn make_move(&mut self, mv: &Move);
    fn unmake_move(&mut self, mv: &Move);

    fn hash(&self) -> u64;

    fn side_to_move(&self) -> Color;

    fn load_fen(&mut self, fen: &str);
}

pub use bitboard::BitboardBoard;
use crate::board::types::Color;
