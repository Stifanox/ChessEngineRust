use std::collections::HashMap;
use crate::board::{BitboardBoard, Board};
use crate::board::types::{Move, Piece, Square, GameState};
use crate::evaluation::{Evaluator, SimpleEvaluator};
use crate::search::{Searcher, AlphaBetaSearcher};

pub struct GameManager {
    board: BitboardBoard,
    evaluator: Box<dyn Evaluator>,
    searcher: Box<dyn Searcher>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            board: BitboardBoard::new_startpos(),
            evaluator: Box::new(SimpleEvaluator),
            searcher: Box::new(AlphaBetaSearcher),
        }
    }

    pub fn get_board(&self) -> &BitboardBoard {
        &self.board
    }

    pub fn get_fen(&self) -> String {
        self.board.to_fen()
    }

    pub fn load_from_fen(&mut self, fen: &str) {
        self.board.load_fen(fen);
    }

    pub fn get_evaluation(&self) -> i32 {
        self.evaluator.evaluate(&self.board)
    }

    pub fn get_legal_moves_grouped(&self) -> HashMap<(Piece, Square), Vec<Move>> {
        let mut moves_by_piece: HashMap<(Piece, Square), Vec<Move>> = HashMap::new();
        for mv in self.board.generate_moves() {
            moves_by_piece.entry((mv.piece, mv.from)).or_default().push(mv);
        }
        moves_by_piece
    }

    pub fn make_move_from_notation(&mut self, from: &str, to: &str) -> Result<(), &'static str> {
        let from_square = Square::from_notation(from).ok_or("Invalid from square")?;
        let to_square = Square::from_notation(to).ok_or("Invalid to square")?;

        let legal_moves = self.board.generate_moves();
        let chosen_move = legal_moves.iter().find(|m| m.from == from_square && m.to == to_square);

        match chosen_move {
            Some(mv) => {
                self.board.make_move(mv);
                self.board.switch_side();
                Ok(())
            }
            None => Err("Illegal move"),
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        self.board.make_move(mv);
        self.board.switch_side();
    }

    pub fn get_game_state(&self) -> GameState {
        let legal_moves = self.board.generate_moves();
        if legal_moves.is_empty() {
            if self.board.is_in_check(self.board.side_to_move()) {
                return GameState::Checkmate;
            } else {
                return GameState::Stalemate;
            }
        }
        GameState::InProgress
    }

    pub fn find_best_move(&self) -> Option<Move> {
        self.searcher.search(&self.board, self.evaluator.as_ref())
    }
}
