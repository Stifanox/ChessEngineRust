use crate::board::{BitboardBoard, Board};
use crate::board::types::{Piece, Square};

mod board;
mod utils;
mod move_generation;

fn main() {
    let board = BitboardBoard::new_startpos();
    let square = Square::from_coords(1,0);
    let result = board.piece_at(square);

    match result {
        None => {
            let (rank,file) = square.to_coords();
            println!("Nothing on rank: {rank} file: {file}")
        }
        Some(piece) => {
            let (rank,file) = square.to_coords();
            let color = piece.color;
            let kind = piece.kind;
            println!("On rank: {rank} file: {file} is {color:?} {kind:?}")
        }
    }

}
