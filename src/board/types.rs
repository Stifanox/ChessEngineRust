#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub promotion: Option<PieceKind>,
}

impl Move {
    pub fn new(from: Square, to: Square, piece: Piece) -> Self {
        Self {
            from,
            to,
            piece,
            promotion: None,
        }
    }
}
