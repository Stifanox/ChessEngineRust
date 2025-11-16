use crate::move_generation::BitboardMoveGenerator;
use crate::move_generation::naive_move_generator::NaiveMoveGenerator;
use super::Board;
use super::types::{Color, Move, Piece, PieceKind, Square};

pub const CASTLE_WHITE_KINGSIDE: u8 = 1;
pub const CASTLE_WHITE_QUEENSIDE: u8 = 2;
pub const CASTLE_BLACK_KINGSIDE: u8 = 4;
pub const CASTLE_BLACK_QUEENSIDE: u8 = 8;


#[derive(Clone, Copy)]
pub struct BoardState {
    pub castling_rights: u8,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
}

pub struct BitboardBoard {
    pub bitboards: [u64; 12],
    side_to_move: Color,
    move_generator: Box<dyn BitboardMoveGenerator>,
    pub castling_rights: u8,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    history: Vec<BoardState>,
}

impl BitboardBoard {
    pub fn new_empty(move_generator: Option<Box<dyn BitboardMoveGenerator>>) -> Self {
        match move_generator {
            None => {
                Self {
                    bitboards: [0; 12],
                    side_to_move: Color::White,
                    move_generator: Box::new(NaiveMoveGenerator::new()),
                    castling_rights: 0,
                    en_passant_square: None,
                    halfmove_clock: 0,
                    fullmove_number: 1,
                    history: Vec::new(),
                }
            }
            Some(generator) => {
                Self {
                    bitboards: [0; 12],
                    side_to_move: Color::White,
                    move_generator: generator,
                    castling_rights: 0,
                    en_passant_square: None,
                    halfmove_clock: 0,
                    fullmove_number: 1,
                    history: Vec::new(),
                }
            }
        }

    }

    pub fn new_startpos() -> Self {
        let mut board = Self::new_empty(None);
        board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board
    }
    pub fn switch_side(&mut self) {
        self.side_to_move = if self.side_to_move == Color::White { Color::Black } else { Color::White };
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                let square = Square::from_coords(file, rank);
                if let Some(piece) = self.piece_at(square) {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    fen.push(Self::piece_to_fen_char(piece));
                } else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(if self.side_to_move == Color::White { 'w' } else { 'b' });

        fen.push(' ');
        let mut castling_str = String::new();
        if self.castling_rights & CASTLE_WHITE_KINGSIDE != 0 { castling_str.push('K'); }
        if self.castling_rights & CASTLE_WHITE_QUEENSIDE != 0 { castling_str.push('Q'); }
        if self.castling_rights & CASTLE_BLACK_KINGSIDE != 0 { castling_str.push('k'); }
        if self.castling_rights & CASTLE_BLACK_QUEENSIDE != 0 { castling_str.push('q'); }
        if castling_str.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&castling_str);
        }

        fen.push(' ');
        if let Some(ep_square) = self.en_passant_square {
            fen.push_str(&ep_square.to_notation());
        } else {
            fen.push('-');
        }

        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    fn piece_to_fen_char(piece: Piece) -> char {
        let c = match piece.kind {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        };
        if piece.color == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
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
        let pseudo_legal_moves = self.move_generator.generate_moves(self);
        let mut legal_moves = Vec::new();

        for mv in pseudo_legal_moves {
            let mut board_clone = self.clone();
            board_clone.make_move(&mv);

            if !board_clone.is_in_check(self.side_to_move()) {
                if mv.piece.kind == PieceKind::King {
                    let diff = mv.to.0 as i8 - mv.from.0 as i8;
                    if diff.abs() == 2 {
                        let mut is_legal = true;
                        let (start, end) = if diff > 0 { (mv.from.0, mv.to.0) } else { (mv.to.0, mv.from.0) };
                        let attack_map = self.generate_attack_map(self.side_to_move().opposite());
                        for i in start..=end {
                            if (attack_map & (1u64 << i)) != 0 {
                                is_legal = false;
                                break;
                            }
                        }
                        if is_legal {
                            legal_moves.push(mv);
                        }
                    } else {
                        legal_moves.push(mv);
                    }
                } else {
                    legal_moves.push(mv);
                }
            }
        }
        legal_moves
    }
    fn make_move(&mut self, mv: &Move) {
        self.history.push(BoardState {
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
            halfmove_clock: self.halfmove_clock,
        });

        self.halfmove_clock += 1;
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        if mv.piece.kind == PieceKind::Pawn || mv.captures.is_some() {
            self.halfmove_clock = 0;
        }

        let from_mask = 1u64 << mv.from.0;
        let to_mask = 1u64 << mv.to.0;

        let piece_index = self.get_bitboard_index(mv.piece.color, mv.piece.kind);
        self.en_passant_square = None;

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

        // castling
        if mv.piece.kind == PieceKind::King {
            let diff = mv.to.0 as i8 - mv.from.0 as i8;
            if diff.abs() == 2 {
                let (rook_from, rook_to) = if diff > 0 {
                    (mv.to.0 + 1, mv.to.0 - 1)
                } else {
                    (mv.to.0 - 2, mv.to.0 + 1)
                };
                let rook_index = self.get_bitboard_index(mv.piece.color, PieceKind::Rook);
                self.bitboards[rook_index] &= !(1u64 << rook_from);
                self.bitboards[rook_index] |= 1u64 << rook_to;
            }
            if mv.piece.color == Color::White {
                self.castling_rights &= !CASTLE_WHITE_KINGSIDE;
                self.castling_rights &= !CASTLE_WHITE_QUEENSIDE;
            } else {
                self.castling_rights &= !CASTLE_BLACK_KINGSIDE;
                self.castling_rights &= !CASTLE_BLACK_QUEENSIDE;
            }
        }

        if mv.piece.kind == PieceKind::Rook {
            if mv.from.0 == 0 {
                self.castling_rights &= !CASTLE_WHITE_QUEENSIDE;
            } else if mv.from.0 == 7 {
                self.castling_rights &= !CASTLE_WHITE_KINGSIDE;
            } else if mv.from.0 == 56 {
                self.castling_rights &= !CASTLE_BLACK_QUEENSIDE;
            } else if mv.from.0 == 63 {
                self.castling_rights &= !CASTLE_BLACK_KINGSIDE;
            }
        }

        if mv.piece.kind == PieceKind::Pawn {
            let diff = mv.to.0 as i8 - mv.from.0 as i8;
            if diff.abs() == 16 {
                self.en_passant_square = Some(Square((mv.from.0 as i8 + diff / 2) as u8));
            }
        }
    }


    fn unmake_move(&mut self, mv: &Move) {
        if let Some(prev_state) = self.history.pop() {
            self.castling_rights = prev_state.castling_rights;
            self.en_passant_square = prev_state.en_passant_square;
            self.halfmove_clock = prev_state.halfmove_clock;
        }
        if self.side_to_move == Color::White {
            self.fullmove_number -= 1;
        }

        let from_mask = 1u64 << mv.from.0;
        let to_mask = 1u64 << mv.to.0;

        let piece_index = self.get_bitboard_index(mv.piece.color, mv.piece.kind);

        // promotion
        if let Some(promoted_kind) = mv.promotion {
            let promo_index = self.get_bitboard_index(mv.piece.color, promoted_kind);
            self.bitboards[promo_index] &= !to_mask;
            self.bitboards[piece_index] |= from_mask;
        } else {
            self.bitboards[piece_index] &= !to_mask;
            self.bitboards[piece_index] |= from_mask;
        }

        // capture
        if let Some(captured) = &mv.captures {
            let capture_index = self.get_bitboard_index(captured.color, captured.kind);
            self.bitboards[capture_index] |= to_mask;
        }

        // castling
        if mv.piece.kind == PieceKind::King {
            let diff = mv.to.0 as i8 - mv.from.0 as i8;
            if diff.abs() == 2 {
                let (rook_from, rook_to) = if diff > 0 {
                    (mv.to.0 + 1, mv.to.0 - 1)
                } else {
                    (mv.to.0 - 2, mv.to.0 + 1)
                };
                let rook_index = self.get_bitboard_index(mv.piece.color, PieceKind::Rook);
                self.bitboards[rook_index] |= 1u64 << rook_from;
                self.bitboards[rook_index] &= !(1u64 << rook_to);
            }
        }
    }

    fn hash(&self) -> u64 {
        0
    }
    fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    fn load_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 6 {
            panic!("Invalid FEN string: not enough parts");
        }

        self.bitboards = [0u64; 12];
        self.castling_rights = 0;
        self.en_passant_square = None;
        self.history.clear();

        let position_part = parts[0];
        let ranks: Vec<&str> = position_part.split('/').collect();
        if ranks.len() != 8 {
            panic!("FEN must have 8 ranks");
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file_idx = 0;
            for c in rank_str.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file_idx += digit as u8;
                } else {
                    if let Some(bb_index) = Self::fen_char_to_bitboard_index(c) {
                        let square_index = (7 - rank_idx) * 8 + (file_idx as usize);
                        self.bitboards[bb_index] |= 1u64 << square_index;
                    }
                    file_idx += 1;
                }
            }
        }

        self.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid FEN side to move"),
        };

        let castling_part = parts[2];
        if castling_part != "-" {
            for c in castling_part.chars() {
                match c {
                    'K' => self.castling_rights |= CASTLE_WHITE_KINGSIDE,
                    'Q' => self.castling_rights |= CASTLE_WHITE_QUEENSIDE,
                    'k' => self.castling_rights |= CASTLE_BLACK_KINGSIDE,
                    'q' => self.castling_rights |= CASTLE_BLACK_QUEENSIDE,
                    _ => {}
                }
            }
        }

        let en_passant_part = parts[3];
        if en_passant_part != "-" {
            self.en_passant_square = Square::from_notation(en_passant_part);
        }

        self.halfmove_clock = parts[4].parse().unwrap_or(0);
        self.fullmove_number = parts[5].parse().unwrap_or(1);
    }
}

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

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

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_bb = self.bitboards[self.get_bitboard_index(color, PieceKind::King)];
        if king_bb == 0 {
            return true; // King is captured, this is an illegal state
        }
        let king_square = king_bb.trailing_zeros() as u8;
        let attack_map = self.generate_attack_map(color.opposite());
        (attack_map & (1u64 << king_square)) != 0
    }

    pub fn generate_attack_map(&self, color: Color) -> u64 {
        let mut attack_map = 0u64;
        let pawns = self.bitboards[self.get_bitboard_index(color, PieceKind::Pawn)];

        if color == Color::White {
            attack_map |= (pawns << 7) & NOT_H_FILE;
            attack_map |= (pawns << 9) & NOT_A_FILE;
        } else {
            attack_map |= (pawns >> 7) & NOT_A_FILE;
            attack_map |= (pawns >> 9) & NOT_H_FILE;
        }

        let mut knights = self.bitboards[self.get_bitboard_index(color, PieceKind::Knight)];
        while knights != 0 {
            let sq = knights.trailing_zeros() as u8;
            knights &= knights - 1;
            
            let knight_offsets = [15, 17, 10, 6, -15, -17, -10, -6];
            for &offset in knight_offsets.iter() {
                let target = sq as i8 + offset;
                if target < 0 || target >= 64 { continue; }

                let from_file = sq % 8;
                let to_file = target as u8 % 8;
                if (from_file as i8 - to_file as i8).abs() > 2 { continue; }

                attack_map |= 1u64 << target;
            }
        }

        let mut kings = self.bitboards[self.get_bitboard_index(color, PieceKind::King)];
        while kings != 0 {
            let sq = kings.trailing_zeros() as u8;
            kings &= kings - 1;
            
            let king_offsets = [8, -8, 1, -1, 9, 7, -9, -7];
            for &offset in king_offsets.iter() {
                let target = sq as i8 + offset;
                if target < 0 || target >= 64 { continue; }

                let from_file = sq % 8;
                let to_file = target as u8 % 8;
                if (from_file as i8 - to_file as i8).abs() > 1 { continue; }

                attack_map |= 1u64 << target;
            }
        }

        let mut bishops = self.bitboards[self.get_bitboard_index(color, PieceKind::Bishop)];
        while bishops != 0 {
            let sq = bishops.trailing_zeros() as u8;
            bishops &= bishops - 1;
            attack_map |= self.get_bishop_attacks(Square(sq));
        }

        let mut rooks = self.bitboards[self.get_bitboard_index(color, PieceKind::Rook)];
        while rooks != 0 {
            let sq = rooks.trailing_zeros() as u8;
            rooks &= rooks - 1;
            attack_map |= self.get_rook_attacks(Square(sq));
        }

        let mut queens = self.bitboards[self.get_bitboard_index(color, PieceKind::Queen)];
        while queens != 0 {
            let sq = queens.trailing_zeros() as u8;
            queens &= queens - 1;
            attack_map |= self.get_bishop_attacks(Square(sq));
            attack_map |= self.get_rook_attacks(Square(sq));
        }

        attack_map
    }

    fn get_bishop_attacks(&self, square: Square) -> u64 {
        let mut attacks = 0u64;
        let (r, f) = (square.0 / 8, square.0 % 8);
        let occupied = self.get_all_pieces_mask();

        for i in 1..8 {
            if r + i > 7 || f + i > 7 { break; }
            let mask = 1u64 << ((r + i) * 8 + (f + i));
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in 1..8 {
            if r + i > 7 || f < i { break; }
            let mask = 1u64 << ((r + i) * 8 + (f - i));
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in 1..8 {
            if r < i || f + i > 7 { break; }
            let mask = 1u64 << ((r - i) * 8 + (f + i));
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in 1..8 {
            if r < i || f < i { break; }
            let mask = 1u64 << ((r - i) * 8 + (f - i));
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }

        attacks
    }

    fn get_rook_attacks(&self, square: Square) -> u64 {
        let mut attacks = 0u64;
        let (r, f) = (square.0 / 8, square.0 % 8);
        let occupied = self.get_all_pieces_mask();

        for i in (r + 1)..8 {
            let mask = 1u64 << (i * 8 + f);
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in (0..r).rev() {
            let mask = 1u64 << (i * 8 + f);
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in (f + 1)..8 {
            let mask = 1u64 << (r * 8 + i);
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }
        for i in (0..f).rev() {
            let mask = 1u64 << (r * 8 + i);
            attacks |= mask;
            if occupied & mask != 0 { break; }
        }

        attacks
    }
}

impl Clone for BitboardBoard {
    fn clone(&self) -> Self {
        Self {
            bitboards: self.bitboards.clone(),
            side_to_move: self.side_to_move,
            move_generator: Box::new(NaiveMoveGenerator::new()),
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
            history: self.history.clone(),
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
        }
    }
}

impl Color {
    pub fn opposite(&self) -> Color {
        if *self == Color::White { Color::Black } else { Color::White }
    }
}
