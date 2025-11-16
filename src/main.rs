use std::io;
use crate::board::Board;
use crate::board::types::{Color, GameState, PieceKind, Square};
use crate::game::GameManager;
use crate::utils::bitboards::utils::utils::print_board_state;

mod board;
mod utils;
mod move_generation;
mod tests;
mod evaluation;
mod search;
mod game;

fn main() {
    let mut game_manager = GameManager::new();
    let mut input_buffer = String::new();

    loop {
        print_board_state(game_manager.get_board());
        println!("Current evaluation: {}", game_manager.get_evaluation());

        match game_manager.get_game_state() {
            GameState::Checkmate => {
                println!("Checkmate!");
                break;
            }
            GameState::Stalemate => {
                println!("Stalemate!");
                break;
            }
            GameState::InProgress => {}
        }

        if game_manager.get_board().side_to_move() == Color::White {
            println!("Enter command (e.g., 'move', 'getfen', 'loadfen <fen>', 'q'):");
            input_buffer.clear();
            io::stdin().read_line(&mut input_buffer).unwrap();
            let input_line = input_buffer.trim();
            let parts: Vec<&str> = input_line.splitn(2, ' ').collect();
            let command = parts[0];

            match command {
                "q" => break,
                "getfen" => {
                    println!("FEN: {}", game_manager.get_fen());
                    continue;
                }
                "loadfen" => {
                    if parts.len() > 1 {
                        game_manager.load_from_fen(parts[1]);
                    } else {
                        println!("Please provide a FEN string.");
                    }
                    continue;
                }
                "move" => {
                    let moves_by_piece = game_manager.get_legal_moves_grouped();
                    if moves_by_piece.is_empty() {
                        println!("No legal moves available.");
                        continue;
                    }

                    println!("Available pieces to move:");
                    let mut pieces: Vec<_> = moves_by_piece.keys().collect();
                    pieces.sort_by_key(|(p, s)| (p.kind as u8, s.0));
                    for (i, (piece, square)) in pieces.iter().enumerate() {
                        println!("{}. {}{}", i + 1, piece_kind_to_char(piece.kind), square.to_notation());
                    }

                    println!("Enter piece to move (e.g., Pe2):");
                    input_buffer.clear();
                    io::stdin().read_line(&mut input_buffer).unwrap();
                    let piece_input = input_buffer.trim();

                    let (piece_kind, from_square) = match parse_piece_and_square(piece_input) {
                        Some(ps) => ps,
                        None => {
                            println!("Invalid input format.");
                            continue;
                        }
                    };

                    let selected_moves = match moves_by_piece.iter().find(|((p, s), _)| p.kind == piece_kind && *s == from_square) {
                        Some((_, mvs)) => mvs,
                        None => {
                            println!("No moves for that piece.");
                            continue;
                        }
                    };

                    println!("Available moves for {}{}:", piece_kind_to_char(piece_kind), from_square.to_notation());
                    for (i, mv) in selected_moves.iter().enumerate() {
                        println!("{}. {}", i + 1, mv.to.to_notation());
                    }

                    println!("Enter destination square (e.g., e4):");
                    input_buffer.clear();
                    io::stdin().read_line(&mut input_buffer).unwrap();
                    let dest_input = input_buffer.trim();

                    let dest_square = match Square::from_notation(dest_input) {
                        Some(s) => s,
                        None => {
                            println!("Invalid square notation.");
                            continue;
                        }
                    };

                    let final_move = match selected_moves.iter().find(|m| m.to == dest_square) {
                        Some(m) => m.clone(),
                        None => {
                            println!("Invalid destination for that piece.");
                            continue;
                        }
                    };

                    game_manager.apply_move(&final_move);
                }
                _ => println!("Unknown command."),
            }
        } else {
            println!("Black is thinking...");
            if let Some(mv) = game_manager.find_best_move() {
                println!("Black plays {}{}", mv.from.to_notation(), mv.to.to_notation());
                game_manager.apply_move(&mv);
            } else {
                println!("Black has no moves!");
            }
        }
    }
}

fn piece_kind_to_char(kind: PieceKind) -> char {
    match kind {
        PieceKind::Pawn => 'P',
        PieceKind::Knight => 'N',
        PieceKind::Bishop => 'B',
        PieceKind::Rook => 'R',
        PieceKind::Queen => 'Q',
        PieceKind::King => 'K',
    }
}

fn char_to_piece_kind(c: char) -> Option<PieceKind> {
    match c.to_ascii_uppercase() {
        'P' => Some(PieceKind::Pawn),
        'N' => Some(PieceKind::Knight),
        'B' => Some(PieceKind::Bishop),
        'R' => Some(PieceKind::Rook),
        'Q' => Some(PieceKind::Queen),
        'K' => Some(PieceKind::King),
        _ => None,
    }
}

fn parse_piece_and_square(s: &str) -> Option<(PieceKind, Square)> {
    if s.len() != 3 {
        return None;
    }
    let piece_kind = char_to_piece_kind(s.chars().nth(0)?)?;
    let square = Square::from_notation(&s[1..3])?;
    Some((piece_kind, square))
}
