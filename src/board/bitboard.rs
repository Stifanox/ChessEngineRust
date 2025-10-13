use super::Board;
use super::types::{Color, Move, Piece, PieceKind, Square};

pub struct BitboardBoard {
    pub bitboards: [u64; 12],
    side_to_move: Color,
}

impl BitboardBoard {
    pub fn new_empty() -> Self {
        Self {
            bitboards: [0; 12],
            side_to_move: Color::White,
        }
    }

    pub fn new_startpos() -> Self {
        let mut board = Self::new_empty();

        // białe piony
        board.bitboards[0] = 0x000000000000FF00;
        // białe figury
        board.bitboards[1] = 0x0000000000000042;
        board.bitboards[2] = 0x0000000000000024;
        board.bitboards[3] = 0x0000000000000081;
        board.bitboards[4] = 0x0000000000000008;
        board.bitboards[5] = 0x0000000000000010;

        // czarne piony
        board.bitboards[6] = 0x00FF000000000000;
        // czarne figury
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

    fn all_pieces(&self) -> Vec<(Square, Piece)> {
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
    fn make_move(&mut self, _: &Move) {}
    fn unmake_move(&mut self, _: &Move) {}
    fn hash(&self) -> u64 {
        0
    }
    fn side_to_move(&self) -> Color {
        self.side_to_move
    }
}

impl BitboardBoard {
    pub fn color_mask(&self, color: Color) -> u64 {
        match color {
            Color::White => self.bitboards[0..6].iter().fold(0, |acc,&bb| acc | bb),
            Color::Black => self.bitboards[6..12].iter().fold(0, |acc,&bb| acc | bb)
        }
    }

    pub fn all_pieces_mask(&self) -> u64{
        self.bitboards.iter().fold(0u64, |acc, &bb| acc | bb)
    }

}
