use std::io;
use std::fmt;
use std::fmt::Formatter;

enum ChessPiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None,
}
impl fmt::Display for ChessPiece{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ChessPiece::Bishop => write!(f, ""),
            ChessPiece::Pawn => write!{f, ""},
            ChessPiece::Rook => write!{f, ""},
            ChessPiece::Knight => write!{f, ""},
            ChessPiece::Queen => write!{f, ""},
            ChessPiece::King => write!{f, ""},
            ChessPiece::None => write!{f, ""},
        }
    }
}
struct ChessMove {
    from: u32,
    to: u32,
    piece: ChessPiece,
}
impl Default for ChessMove{
    fn default() -> ChessMove {
        ChessMove{
            from: 0,
            to: 0,
            piece: ChessPiece::None,
        }
    }
}
fn parse_move() -> Result<ChessMove, String> {
    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    if result.is_err() {
        return Err(String::new());
    }
    let mut chess_move = ChessMove{..Default::default()};

    for letter in buffer.chars() {
        if letter == 'a' {
            chess_move.from = 10;
            chess_move.to = 5;
            chess_move.piece = ChessPiece::Pawn;
        }
    }
    Ok(chess_move)
}

fn main() -> io::Result<()> {
    println!("What's your name");
    let chess_move = parse_move();

    match chess_move {
        Ok(new_move) => println!("New move: {}, from: {} to {}", new_move.from, new_move.to, new_move.piece),
        Err(e) => println!("{}",e),
    }
    Ok(())
}
