use std::io;

fn parse_move() -> Result<String, String> {
    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    if result.is_err() {
        return Err(String::new());
    }
    Ok(buffer)
}

fn main() -> io::Result<()> {
    println!("What's your name");
    let chess_move = parse_move();

    match chess_move {
        Ok(new_move) => println!("{}", new_move),
        Err(e) => println!("{}",e),
    }
    Ok(())
}
