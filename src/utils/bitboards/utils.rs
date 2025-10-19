pub mod utils{
    use crate::board::{BitboardBoard, Board};
    use crate::board::types::{Color, Piece, PieceKind, Square};

    pub fn print_board_state(board: &BitboardBoard) {
        let mut board_string = String::new();

        for rank in (0..8).rev() {
            board_string.push('|');
            for file in 0..8 {
                let position = rank * 8 + file;
                let piece_at = board.piece_at(Square(position));

                match piece_at {
                    None => board_string.push_str(" *"),
                    Some(piece) => board_string.push_str(&format!(" {}", map_piece_to_char(&piece))),
                }
            }
            board_string.push_str(" |\n");
        }

        print!("{}", board_string);
    }



    pub fn map_piece_to_char(piece: &Piece) -> &str {
        match (piece.kind, piece.color) {
            (PieceKind::Pawn,   Color::White) => "P",
            (PieceKind::Pawn,   Color::Black) => "p",
            (PieceKind::Knight, Color::White) => "N",
            (PieceKind::Knight, Color::Black) => "n",
            (PieceKind::Bishop, Color::White) => "B",
            (PieceKind::Bishop, Color::Black) => "b",
            (PieceKind::Rook,   Color::White) => "R",
            (PieceKind::Rook,   Color::Black) => "r",
            (PieceKind::Queen,  Color::White) => "Q",
            (PieceKind::Queen,  Color::Black) => "q",
            (PieceKind::King,   Color::White) => "K",
            (PieceKind::King,   Color::Black) => "k",
        }
    }
}
