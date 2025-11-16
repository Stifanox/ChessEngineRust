use crate::board::{BitboardBoard, Board};
use crate::board::types::{Color, Square};
use crate::move_generation::naive_move_generator::NaiveMoveGenerator;

#[test]
fn white_pawn_double_push() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/8/8/8/P7/8 w - - 0 1");
    let mut moves = Vec::new();
    let pawn_square = Square::from_coords(0, 1).0;
    NaiveMoveGenerator.generate_pawn_moves(&board, pawn_square, Color::White, &mut moves);
    let mut move_destinations: Vec<u8> = moves.iter().map(|m| m.to.0).collect();
    move_destinations.sort();
    assert_eq!(move_destinations, vec![16, 24]);
}

#[test]
fn black_pawn_double_push() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/p7/8/8/8/8/8/8 b - - 0 1");
    let mut moves = Vec::new();
    let pawn_square = Square::from_coords(0, 6).0;
    NaiveMoveGenerator.generate_pawn_moves(&board, pawn_square, Color::Black, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["a6", "a5"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn white_pawn_capture() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/p1p5/1P6/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let pawn_square = Square::from_coords(1, 3).0;
    NaiveMoveGenerator.generate_pawn_moves(&board, pawn_square, Color::White, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["b5", "a5", "c5"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn black_pawn_capture() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/1p6/P1P5/8/8/8 b - - 0 1");
    let mut moves = Vec::new();
    let pawn_square = Square::from_coords(1, 4).0;
    NaiveMoveGenerator.generate_pawn_moves(&board, pawn_square, Color::Black, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["b4", "a4", "c4"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn knight_moves_center() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/3N4/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let knight_square = Square::from_coords(3, 4).0;
    NaiveMoveGenerator.generate_knight_moves(&board, knight_square, Color::White, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["c3", "b4", "b6", "c7", "e7", "f6", "f4", "e3"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn knight_moves_corner() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("N7/8/8/8/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let knight_square = Square::from_coords(0, 7).0;
    NaiveMoveGenerator.generate_knight_moves(&board, knight_square, Color::White, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    assert_eq!(move_destinations, vec!["b6", "c7"]);
}

#[test]
fn bishop_moves() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/3B4/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let bishop_square = Square::from_coords(3, 4).0;
    NaiveMoveGenerator.generate_bishop_moves(&board, bishop_square, Color::White, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["a8", "b7", "c6", "e4", "f3", "g2", "h1", "a2", "b3", "c4", "e6", "f7", "g8"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn rook_moves() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/3R4/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let rook_square = Square::from_coords(3, 4).0;
    NaiveMoveGenerator.generate_rook_moves(&board, rook_square, Color::White, &mut moves);
    let mut move_destinations: Vec<u8> = moves.iter().map(|m| m.to.0).collect();
    move_destinations.sort();
    assert_eq!(move_destinations, vec![3, 11, 19, 27, 32, 33, 34, 36, 37, 38, 39, 43, 51, 59]);
}

#[test]
fn queen_moves() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/3Q4/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let queen_square = Square::from_coords(3, 4).0;
    NaiveMoveGenerator.generate_queen_moves(&board, queen_square, Color::White, &mut moves);
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["a8", "b7", "c6", "e4", "f3", "g2", "h1", "a2", "b3", "c4", "e6", "f7", "g8", "a5", "b5", "c5", "e5", "f5", "g5", "h5", "d1", "d2", "d3", "d4", "d6", "d7", "d8"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}

#[test]
fn king_moves() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("8/8/8/3K4/8/8/8/8 w - - 0 1");
    let mut moves = Vec::new();
    let king_square = Square::from_coords(3, 4).0;
    NaiveMoveGenerator.generate_king_moves(&board, king_square, Color::White, &mut moves);
    let mut move_destinations: Vec<u8> = moves.iter().map(|m| m.to.0).collect();
    move_destinations.sort();
    assert_eq!(move_destinations, vec![26, 27, 28, 34, 36, 42, 43, 44]);
}

#[test]
fn king_moves_in_check() {
    let mut board = BitboardBoard::new_empty(None);
    board.load_fen("r1b2rk1/pppp1ppp/2n1p3/4Pn2/3PNP2/2PB1Q2/PP4Pq/R4RK1 w - - 0 14");
    let moves = board.generate_moves();
    let mut move_destinations: Vec<String> = moves.iter().map(|m| m.to.to_notation()).collect();
    move_destinations.sort();
    let mut expected = vec!["f2", "h2"];
    expected.sort();
    assert_eq!(move_destinations, expected);
}
