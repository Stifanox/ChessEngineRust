use crate::board::types::{Color, Move, Piece, PieceKind, Square};
use crate::board::{BitboardBoard, Board};
use crate::move_generation::MoveGenerator;

struct NaiveMoveGenerator;

impl MoveGenerator for NaiveMoveGenerator {
    fn generate_moves(&self, board: &BitboardBoard) -> Vec<Move> {
        let mut pseudo_legal_moves: Vec<Move> = Vec::new();
        let color = board.side_to_move();
        let range = match color {
            Color::White => 0..6,
            Color::Black => 6..12,
        };

        for i in range {
            let mut bb = board.bitboards[i];
            while bb != 0 {
                let square = bb.trailing_zeros() as u8;
                bb &= bb - 1;
                match i % 6 {
                    0 => self.generate_pawn_moves(board, square, color, &mut pseudo_legal_moves),
                    _ => (),
                }
            }
        }

        pseudo_legal_moves
    }
}

impl NaiveMoveGenerator {
    fn generate_pawn_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let (forward, start_rank_mask, enemy_color) = match color {
            Color::White => (8, 0x000000000000FF00u64, Color::Black),
            Color::Black => (-8, 0x00FF000000000000u64, Color::White),
        };

        let enemy = board.color_mask(enemy_color);
        let occupied = board.all_pieces_mask();

        let target = square as i8 + forward;
        if target >= 0 && target < 64 && (occupied & (1 << target)) == 0 {
            moves.push(Move::new(
                Square(square),
                Square(target as u8),
                Piece {
                    color,
                    kind: PieceKind::Pawn,
                },
            ));
        }

        if (1u64 << square) & start_rank_mask != 0 {
            let double = square as i8 + 2 * forward;
            if double >= 0 && double < 64 && (occupied & (1 << double)) == 0 {
                moves.push(Move::new(
                    Square(square),
                    Square(double as u8),
                    Piece {
                        color,
                        kind: PieceKind::Pawn,
                    },
                ));
            }
        }

        for &diag in &[forward - 1, forward + 1] {
            let target = square as i8 + diag;
            if target >= 0 && target < 64 && (enemy & (1 << target)) != 0 {
                moves.push(Move::new(
                    Square(square),
                    Square(target as u8),
                    Piece {
                        color,
                        kind: PieceKind::Pawn,
                    },
                ));
            }
        }
    }
}
