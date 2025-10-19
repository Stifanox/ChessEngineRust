use crate::move_generation::BitboardMoveGenerator;
use crate::move_generation::naive_move_generator::NaiveMoveGenerator;
use super::Board;
use super::types::{Color, Move, Piece, PieceKind, Square};

pub struct BitboardBoard {
    pub bitboards: [u64; 12],
    side_to_move: Color,
    move_generator: Box<dyn BitboardMoveGenerator>,
}

impl BitboardBoard {
    pub fn new_empty(move_generator: Option<Box<dyn BitboardMoveGenerator>>) -> Self {
        match move_generator {
            None => {
                Self {
                    bitboards: [0; 12],
                    side_to_move: Color::White,
                    move_generator: Box::new(NaiveMoveGenerator::new()),
                }
            }
            Some(generator) => {
                Self {
                    bitboards: [0; 12],
                    side_to_move: Color::White,
                    move_generator: generator
                }
            }
        }

    }

    pub fn new_startpos() -> Self {
        let mut board = Self::new_empty(None);

        // white pawns
        board.bitboards[0] = 0x000000000000FF00;
        // white piece
        board.bitboards[1] = 0x0000000000000042;
        board.bitboards[2] = 0x0000000000000024;
        board.bitboards[3] = 0x0000000000000081;
        board.bitboards[4] = 0x0000000000000008;
        board.bitboards[5] = 0x0000000000000010;

        // black pawns
        board.bitboards[6] = 0x00FF000000000000;
        // black piece
        board.bitboards[7] = 0x4200000000000000;
        board.bitboards[8] = 0x2400000000000000;
        board.bitboards[9] = 0x8100000000000000;
        board.bitboards[10] = 0x0800000000000000;
        board.bitboards[11] = 0x1000000000000000;

        board
    }
}

impl Board for BitboardBoard {

    fn piece_at(&self, sq: Square) -> Option<Piece> {
        let mask = 1u64 << sq.0;
        for (i, &bb) in self.bitboards.iter().enumerate() {
            if bb & mask != 0 {
                let color = if i < 6 { Color::White } else { Color::Black };
                let kind = match i % 6 {
                    0 => PieceKind::Pawn,
                    1 => PieceKind::Knight,
                    2 => PieceKind::Bishop,
                    3 => PieceKind::Rook,
                    4 => PieceKind::Queen,
                    5 => PieceKind::King,
                    _ => unreachable!(),
                };
                return Some(Piece { color, kind });
            }
        }
        None
    }

    fn get_all_pieces(&self) -> Vec<(Square, Piece)> {
        let mut out = Vec::new();
        for (i, &bb) in self.bitboards.iter().enumerate() {
            let color = if i < 6 { Color::White } else { Color::Black };
            let kind = match i % 6 {
                0 => PieceKind::Pawn,
                1 => PieceKind::Knight,
                2 => PieceKind::Bishop,
                3 => PieceKind::Rook,
                4 => PieceKind::Queen,
                5 => PieceKind::King,
                _ => unreachable!(),
            };
            let mut bits = bb;
            while bits != 0 {
                let sq = bits.trailing_zeros() as u8;
                out.push((Square(sq), Piece { color, kind }));
                bits &= bits - 1;
            }
        }
        out
    }

    fn generate_moves(&self) -> Vec<Move> {
        vec![]
    }
    fn make_move(&mut self, mv: &Move) {
        let from_mask = 1u64 << mv.from.0;
        let to_mask = 1u64 << mv.to.0;

        let piece_index = self.get_bitboard_index(mv.piece.color, mv.piece.kind);

        // remove from starting position
        self.bitboards[piece_index] &= !from_mask;

        // capture
        if let Some(captured) = &mv.captures {
            let capture_index = self.get_bitboard_index(captured.color, captured.kind);
            self.bitboards[capture_index] &= !to_mask;
        }

        // promotion
        if let Some(promoted_kind) = mv.promotion {
            let promo_index = self.get_bitboard_index(mv.piece.color, promoted_kind);
            self.bitboards[promo_index] |= to_mask;
        } else {
            self.bitboards[piece_index] |= to_mask;
        }
    }


    fn unmake_move(&mut self, _: &Move) {}
    fn hash(&self) -> u64 {
        0
    }
    fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn load_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 2 {
            panic!("Invalid FEN string");
        }

        let position_part = parts[0];
        let side_part = parts[1];

        self.bitboards = [0u64; 12];

        let ranks: Vec<&str> = position_part.split('/').collect();
        if ranks.len() != 8 {
            panic!("FEN must have 8 ranks");
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file_idx = 0; 

            for c in rank_str.chars() {
                match c {
                    '1'..='8' => {
                        let empty_squares = c.to_digit(10).unwrap() as u8;
                        file_idx += empty_squares;
                    }
                    _ => {
                        if let Some(bb_index) = Self::fen_char_to_bitboard_index(c) {
                            let square_index = (7 - rank_idx) * 8 + (file_idx as usize);
                            self.bitboards[bb_index] |= 1u64 << square_index;
                        }
                        file_idx += 1;
                    }
                }
            }

            if file_idx != 8 {
                panic!("Invalid FEN rank length");
            }
        }

        self.side_to_move = match side_part {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid FEN side to move"),
        };
    }
}

impl BitboardBoard {
    pub fn get_mask_for_color(&self, color: Color) -> u64 {
        match color {
            Color::White => self.bitboards[0..6].iter().fold(0, |acc,&bb| acc | bb),
            Color::Black => self.bitboards[6..12].iter().fold(0, |acc,&bb| acc | bb)
        }
    }
    pub fn get_enemy_bitboard(&self) -> u64 {
        if self.side_to_move == Color::White {
            self.get_mask_for_color(Color::Black)
        } else {
            self.get_mask_for_color(Color::White)
        }
    }

    pub fn get_empty_squares_bitboard(&self) -> u64 {
        !self.get_all_pieces_mask()
    }

    pub fn get_all_pieces_mask(&self) -> u64{
        self.bitboards.iter().fold(0u64, |acc, &bb| acc | bb)
    }

    fn get_bitboard_index(&self, color: Color, kind: PieceKind) -> usize {
        match color {
            Color::White => match kind {
                PieceKind::Pawn => 0,
                PieceKind::Knight => 1,
                PieceKind::Bishop => 2,
                PieceKind::Rook => 3,
                PieceKind::Queen => 4,
                PieceKind::King => 5,
            },
            Color::Black => match kind {
                PieceKind::Pawn => 6,
                PieceKind::Knight => 7,
                PieceKind::Bishop => 8,
                PieceKind::Rook => 9,
                PieceKind::Queen => 10,
                PieceKind::King => 11,
            },
        }
    }

    fn fen_char_to_bitboard_index(c: char) -> Option<usize> {
        match c {
            'P' => Some(0),
            'N' => Some(1),
            'B' => Some(2),
            'R' => Some(3),
            'Q' => Some(4),
            'K' => Some(5),
            'p' => Some(6),
            'n' => Some(7),
            'b' => Some(8),
            'r' => Some(9),
            'q' => Some(10),
            'k' => Some(11),
            _ => None,
        }
    }

}
