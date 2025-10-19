use crate::board::{BitboardBoard, Board};
use crate::board::types::{Color, Square};
use crate::move_generation::naive_move_generator::NaiveMoveGenerator;

#[test]
fn white_pawn_moves() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/8/8/8/8/7P w HAha - 0 1");
    let mut moves = Vec::new();
    NaiveMoveGenerator.generate_pawn_moves(&board, 7, Color::White, &mut moves);

    let expected: Vec<u8> = vec![15];
    let actual: Vec<u8> = moves.iter().map(|m| m.to.0).collect();
    assert_eq!(expected, actual);
}

#[test]
fn white_pawn_capture() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/8 w HAha - 0 1");
    let mut moves = Vec::new();
    NaiveMoveGenerator.generate_pawn_moves(&board, Square::from_coords(3,3).0, Color::White, &mut moves);

    let expected: Vec<u8> = vec![Square::from_coords(3,4).0, Square::from_coords(4,4).0];
    let actual: Vec<u8> = moves.iter().map(|m| m.to.0).collect();
    assert_eq!(expected, actual);
}