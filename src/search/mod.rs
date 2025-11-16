use crate::board::{BitboardBoard, Board};
use crate::board::types::{Move, GameState};
use crate::evaluation::{Evaluator, SimpleEvaluator};

const SEARCH_DEPTH: u8 = 4;

pub trait Searcher {
    fn search(&self, board: &BitboardBoard, evaluator: &dyn Evaluator) -> Option<Move>;
}

pub struct AlphaBetaSearcher;

impl Searcher for AlphaBetaSearcher {
    fn search(&self, board: &BitboardBoard, evaluator: &dyn Evaluator) -> Option<Move> {
        let mut best_move = None;
        let mut best_score = -i32::MAX;
        let mut alpha = -i32::MAX;
        let beta = i32::MAX;

        let mut moves = board.generate_moves();
        moves.sort_by_key(|m| m.captures.is_some());

        for mv in moves {
            let mut new_board = board.clone();
            new_board.make_move(&mv);
            new_board.switch_side();
            let score = -self.alphabeta(&new_board, evaluator, SEARCH_DEPTH - 1, -beta, -alpha);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
            alpha = alpha.max(best_score);
        }

        best_move
    }
}

impl AlphaBetaSearcher {
    fn alphabeta(&self, board: &BitboardBoard, evaluator: &dyn Evaluator, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return evaluator.evaluate(board);
        }

        let mut moves = board.generate_moves();
        if moves.is_empty() {
            if board.is_in_check(board.side_to_move()) {
                return -i32::MAX + 100; // Checkmate
            } else {
                return 0; // Stalemate
            }
        }

        moves.sort_by_key(|m| m.captures.is_some());

        for mv in moves {
            let mut new_board = board.clone();
            new_board.make_move(&mv);
            new_board.switch_side();
            let score = -self.alphabeta(&new_board, evaluator, depth - 1, -beta, -alpha);
            if score >= beta {
                return beta; // Pruning
            }
            alpha = alpha.max(score);
        }

        alpha
    }
}
