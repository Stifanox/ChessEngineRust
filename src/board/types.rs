#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub u8); // 0..63 (A1 = 0, H8 = 63)

impl Square {
    pub fn from_coords(file: u8, rank: u8) -> Self {
        Self(rank * 8 + file)
    }

    pub fn to_coords(&self) -> (u8, u8) {
        (self.0 / 8, self.0 % 8)
    }

    pub fn to_notation(&self) -> String {
        let file = (self.0 % 8) as u8 + b'a';
        let rank = (self.0 / 8) as u8 + b'1';
        format!("{}{}", file as char, rank as char)
    }

    pub fn from_notation(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }
        let file = s.chars().nth(0)? as u8 - b'a';
        let rank = s.chars().nth(1)? as u8 - b'1';
        if file > 7 || rank > 7 {
            return None;
        }
        Some(Self::from_coords(file, rank))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub promotion: Option<PieceKind>,
    pub captures: Option<Piece>,
}

impl Move {
    pub fn new(from: Square, to: Square, piece: Piece, promotion:Option<PieceKind>, captures: Option<Piece>) -> Self {
        Self {
            from,
            to,
            piece,
            promotion,
            captures,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Checkmate,
    Stalemate,
}
