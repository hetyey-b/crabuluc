use puluc::GameBoard;

mod puluc;

fn main() {
    let board = GameBoard::new();

    print!("{}", board.to_string());
}
