use crate::board::{BitboardBoard, Board};
use crate::board::types::{Color, PieceKind};

pub trait Evaluator {
    fn evaluate(&self, board: &BitboardBoard) -> i32;
}

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

const PAWN_PST: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_PST: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_PST: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

const QUEEN_PST: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING_PST: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

pub struct SimpleEvaluator;

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, board: &BitboardBoard) -> i32 {
        let mut score = 0;
        score += self.evaluate_material(board);
        score += self.evaluate_pst(board);

        if board.side_to_move() == Color::White {
            score
        } else {
            -score
        }
    }
}

impl SimpleEvaluator {
    fn evaluate_material(&self, board: &BitboardBoard) -> i32 {
        let mut score = 0;
        score += board.bitboards[0].count_ones() as i32 * PAWN_VALUE;
        score += board.bitboards[1].count_ones() as i32 * KNIGHT_VALUE;
        score += board.bitboards[2].count_ones() as i32 * BISHOP_VALUE;
        score += board.bitboards[3].count_ones() as i32 * ROOK_VALUE;
        score += board.bitboards[4].count_ones() as i32 * QUEEN_VALUE;
        score += board.bitboards[5].count_ones() as i32 * KING_VALUE;

        score -= board.bitboards[6].count_ones() as i32 * PAWN_VALUE;
        score -= board.bitboards[7].count_ones() as i32 * KNIGHT_VALUE;
        score -= board.bitboards[8].count_ones() as i32 * BISHOP_VALUE;
        score -= board.bitboards[9].count_ones() as i32 * ROOK_VALUE;
        score -= board.bitboards[10].count_ones() as i32 * QUEEN_VALUE;
        score -= board.bitboards[11].count_ones() as i32 * KING_VALUE;

        score
    }

    fn evaluate_pst(&self, board: &BitboardBoard) -> i32 {
        let mut score = 0;
        score += self.evaluate_pst_for_piece(board.bitboards[0], &PAWN_PST, Color::White);
        score += self.evaluate_pst_for_piece(board.bitboards[1], &KNIGHT_PST, Color::White);
        score += self.evaluate_pst_for_piece(board.bitboards[2], &BISHOP_PST, Color::White);
        score += self.evaluate_pst_for_piece(board.bitboards[3], &ROOK_PST, Color::White);
        score += self.evaluate_pst_for_piece(board.bitboards[4], &QUEEN_PST, Color::White);
        score += self.evaluate_pst_for_piece(board.bitboards[5], &KING_PST, Color::White);

        score -= self.evaluate_pst_for_piece(board.bitboards[6], &PAWN_PST, Color::Black);
        score -= self.evaluate_pst_for_piece(board.bitboards[7], &KNIGHT_PST, Color::Black);
        score -= self.evaluate_pst_for_piece(board.bitboards[8], &BISHOP_PST, Color::Black);
        score -= self.evaluate_pst_for_piece(board.bitboards[9], &ROOK_PST, Color::Black);
        score -= self.evaluate_pst_for_piece(board.bitboards[10], &QUEEN_PST, Color::Black);
        score -= self.evaluate_pst_for_piece(board.bitboards[11], &KING_PST, Color::Black);

        score
    }

    fn evaluate_pst_for_piece(&self, mut bitboard: u64, pst: &[i32; 64], color: Color) -> i32 {
        let mut score = 0;
        while bitboard != 0 {
            let sq = bitboard.trailing_zeros() as usize;
            bitboard &= bitboard - 1;
            score += if color == Color::White { pst[sq] } else { pst[63 - sq] };
        }
        score
    }
}
