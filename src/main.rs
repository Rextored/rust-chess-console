use crate::ChessPieceType::Bishop;
use crate::Color::{Black, White};
use std::fmt;
use std::fmt::Formatter;
use std::io;

enum ChessPieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None,
}
enum Color {
    White,
    Black,
    Undefined,
}

struct ChessPiece {
    piece_type: ChessPieceType,
    color: Color,
}

impl Default for ChessPiece {
    fn default() -> ChessPiece {
        ChessPiece {
            piece_type: ChessPieceType::King,
            color: Color::White,
        }
    }
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if matches!(self.color, Color::White) {
            match self.piece_type {
                ChessPieceType::Bishop => write!(f, "♗"),
                ChessPieceType::Pawn => write! {f, "♙"},
                ChessPieceType::Rook => write! {f, "♖"},
                ChessPieceType::Knight => write! {f, "♘"},
                ChessPieceType::Queen => write! {f, "♕"},
                ChessPieceType::King => write! {f, "♔"},
                ChessPieceType::None => write! {f, ""},
            }
        } else if matches!(self.color, Color::Black) {
            match self.piece_type {
                ChessPieceType::Bishop => write!(f, "♝"),
                ChessPieceType::Pawn => write!(f, "P"),
                ChessPieceType::Rook => write! {f, "♜"},
                ChessPieceType::Knight => write! {f, "♞"},
                ChessPieceType::Queen => write! {f, "♛"},
                ChessPieceType::King => write! {f, "♚"},
                ChessPieceType::None => write! {f, ""},
            }
        } else {
            write!(f, "")
        }
    }
}

impl fmt::Display for ChessPieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ChessPieceType::Bishop => write!(f, "♗"),
            ChessPieceType::Pawn => write! {f, "♙"},
            ChessPieceType::Rook => write! {f, "♖"},
            ChessPieceType::Knight => write! {f, "♘"},
            ChessPieceType::Queen => write! {f, "♕"},
            ChessPieceType::King => write! {f, "♔"},
            ChessPieceType::None => write! {f, ""},
        }
    }
}

struct ChessMove {
    from: u32,
    to: u32,
    piece: ChessPiece,
}

impl Default for ChessMove {
    fn default() -> ChessMove {
        ChessMove {
            from: 0,
            to: 0,
            piece: ChessPiece {
                ..Default::default()
            },
        }
    }
}

fn parse_move() -> Result<ChessMove, String> {
    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    if result.is_err() {
        return Err(String::new());
    }
    let mut chess_move = ChessMove {
        ..Default::default()
    };

    for letter in buffer.chars() {
        if letter == 'a' {
            chess_move.from = 10;
            chess_move.to = 5;
            chess_move.piece.piece_type = Bishop;
            chess_move.piece.color = White;
        }
    }
    Ok(chess_move)
}

fn main() -> io::Result<()> {
    println!("What's your name");
    let chess_move = parse_move();
    let mut board: [[ChessPiece; 8]; 3] = [
        [
            ChessPiece {
                piece_type: ChessPieceType::Rook,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Knight,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Bishop,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Queen,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::King,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Bishop,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Knight,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Rook,
                color: Color::White,
            },
        ],
        [
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::White,
            },
        ],
        [
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
            ChessPiece {
                piece_type: ChessPieceType::Pawn,
                color: Color::Black,
            },
        ]
    ];
    match chess_move {
        Ok(new_move) => println!(
            "New move: {}, from: {} to {}",
            new_move.from, new_move.to, new_move.piece
        ),
        Err(e) => println!("{}", e),
    }

    for row in board {
        for piece in row {
            print!("{}", piece);
        }
        print!("\n");
    }
    Ok(())
}
