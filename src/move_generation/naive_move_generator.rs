use std::time::Instant;
use crate::board::types::{Color, Move, PieceKind, Square};
use crate::board::{BitboardBoard, Board};
use crate::move_generation::BitboardMoveGenerator;
pub struct NaiveMoveGenerator;

impl BitboardMoveGenerator for NaiveMoveGenerator {
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
                    1 => self.generate_knight_moves(board, square, color, &mut pseudo_legal_moves),
                    2 => self.generate_bishop_moves(board, square, color, &mut pseudo_legal_moves),
                    3 => self.generate_rook_moves(board, square, color, &mut pseudo_legal_moves),
                    4 => self.generate_queen_moves(board, square, color, &mut pseudo_legal_moves),
                    5 => self.generate_king_moves(board, square, color, &mut pseudo_legal_moves),
                    _ => (),
                }
            }
        }

        pseudo_legal_moves
    }
}

impl NaiveMoveGenerator {

    pub fn new() -> NaiveMoveGenerator {Self}
    pub fn generate_pawn_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let all_pieces = board.get_all_pieces_mask();

        let current_piece = board.piece_at(Square(square)).unwrap();
        let (forward, start_rank_mask, promotion_mask, enemy_color) = match color {
            Color::White => (8i8, 0x000000000000FF00u64, 0xFF00000000000000u64 ,Color::Black),
            Color::Black => (-8i8, 0x00FF000000000000u64,0x00000000000000FFu64 ,Color::White),
        };

        let square_mask = 1u64 << square;
        let enemy = board.get_mask_for_color(enemy_color);

        let square_i8 = square as i8;


        let one_forward_square = square_i8 + forward;
        if one_forward_square >= 0 && one_forward_square < 64 {
            let target_mask = 1u64 << one_forward_square;
            if target_mask & all_pieces == 0 {

                if target_mask & promotion_mask != 0 {
                    let promotion_options = [PieceKind::Queen, PieceKind::Rook, PieceKind::Bishop, PieceKind::Knight];

                    for promo in promotion_options.iter() {
                        moves.push(Move {
                            piece: current_piece,
                            from: Square(square),
                            to: Square(one_forward_square as u8),
                            promotion: Some(*promo),
                            captures: None,
                        });
                    }
                } else {
                    moves.push(Move {
                        piece: current_piece,
                        from: Square(square),
                        to: Square(one_forward_square as u8),
                        promotion: None,
                        captures: None,
                    });
                }

                if square_mask & start_rank_mask != 0 {
                    let two_forward_square = square_i8 + forward * 2;
                    if two_forward_square >= 0 && two_forward_square < 64 {
                        let two_mask = 1u64 << two_forward_square;
                        if two_mask & all_pieces == 0 {
                            moves.push(Move {
                                piece: current_piece,
                                from: Square(square),
                                to: Square(two_forward_square as u8),
                                promotion: None,
                                captures: None,
                            });
                        }
                    }
                }
            }
        }

        let diagonal_shifts: [i8; 2] = [7, 9];
        for shift in diagonal_shifts {
            let dir = if color == Color::White { shift } else { -shift };
            let target = square_i8 + dir;

            if target >= 0 && target < 64 {
                let target_mask = 1u64 << target;
                if target_mask & enemy != 0 {
                    moves.push(Move {
                        piece: current_piece,
                        from: Square(square),
                        to: Square(target as u8),
                        promotion: None,
                        captures: board.piece_at(Square(target as u8)),
                    });
                }
            }
        }
    }

    pub fn generate_rook_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_piece = board.piece_at(Square(square)).unwrap();
        let own_mask = board.get_mask_for_color(color);
        let enemy_mask = board.get_enemy_bitboard();

        let directions: [i8; 4] = [8, -8, 1, -1];

        for &dir in directions.iter() {
            let mut target = square as i8;
            loop {
                target += dir;
                if target < 0 || target >= 64 {
                    break;
                }

                if dir == 1 && target % 8 == 0 { break; }
                if dir == -1 && target % 8 == 7 { break; }

                let target_mask = 1u64 << target;

                if target_mask & own_mask != 0 { break; }

                moves.push(Move {
                    piece: current_piece,
                    from: Square(square),
                    to: Square(target as u8),
                    promotion: None,
                    captures: if target_mask & enemy_mask != 0 { board.piece_at(Square(target as u8)) } else { None },
                });

                if target_mask & enemy_mask != 0 { break; }
            }
        }
    }

    pub fn generate_bishop_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_piece = board.piece_at(Square(square)).unwrap();
        let own_mask = board.get_mask_for_color(color);
        let enemy_mask = board.get_enemy_bitboard();

        let directions: [i8; 4] = [7, 9, -7, -9];

        for &dir in directions.iter() {
            let mut target = square as i8;
            loop {
                target += dir;
                if target < 0 || target >= 64 {
                    break;
                }

                let file_diff = ((square as i8) % 8) - (target % 8);
                if dir == 7 && file_diff != -1 { break; }
                if dir == 9 && file_diff != 1 { break; }
                if dir == -7 && file_diff != 1 { break; }
                if dir == -9 && file_diff != -1 { break; }

                let target_mask = 1u64 << target;

                if target_mask & own_mask != 0 { break; }

                moves.push(Move {
                    piece: current_piece,
                    from: Square(square),
                    to: Square(target as u8),
                    promotion: None,
                    captures: if target_mask & enemy_mask != 0 { board.piece_at(Square(target as u8)) } else { None },
                });

                if target_mask & enemy_mask != 0 { break; }
            }
        }
    }

    pub fn generate_knight_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_piece = board.piece_at(Square(square)).unwrap();
        let own_mask = board.get_mask_for_color(color);
        let enemy_mask = board.get_enemy_bitboard();

        let knight_offsets = [15, 17, 10, 6, -15, -17, -10, -6];

        for &offset in knight_offsets.iter() {
            let target = square as i8 + offset;
            if target < 0 || target >= 64 { continue; }

            // check if it doesn't go over the board
            let file_diff = ((square as i8) % 8) - (target % 8) as i8;
            if file_diff.abs() > 2 { continue; }

            let target_mask = 1u64 << target;
            if target_mask & own_mask != 0 { continue; }

            moves.push(Move {
                piece: current_piece,
                from: Square(square),
                to: Square(target as u8),
                promotion: None,
                captures: if target_mask & enemy_mask != 0 { board.piece_at(Square(target as u8)) } else { None },
            });
        }
    }

    pub fn generate_king_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_piece = board.piece_at(Square(square)).unwrap();
        let own_mask = board.get_mask_for_color(color);
        let enemy_mask = board.get_enemy_bitboard();

        let king_offsets = [8, -8, 1, -1, 9, 7, -9, -7];

        for &offset in king_offsets.iter() {
            let target = square as i8 + offset;
            if target < 0 || target >= 64 { continue; }

            // check edges
            if (offset == 1 || offset == 9 || offset == -7) && square % 8 == 7 { continue; }
            if (offset == -1 || offset == 7 || offset == -9) && square % 8 == 0 { continue; }

            let target_mask = 1u64 << target;
            if target_mask & own_mask != 0 { continue; }

            moves.push(Move {
                piece: current_piece,
                from: Square(square),
                to: Square(target as u8),
                promotion: None,
                captures: if target_mask & enemy_mask != 0 { board.piece_at(Square(target as u8)) } else { None },
            });
        }
    }

    pub fn generate_queen_moves(
        &self,
        board: &BitboardBoard,
        square: u8,
        color: Color,
        moves: &mut Vec<Move>,
    ) {
        self.generate_rook_moves(board, square, color, moves);
        self.generate_bishop_moves(board, square, color, moves);
    }
}
