mod board;

use board::Board;
use board::Letter;

fn main() {
    let board = Board::new();
    let mut row = board.rows[0];
    print_row(&row);
    row[7] = Letter::new('s');
    print_row(&row);
    println!("{:?}", board::start_positions(&row, 3));
}

fn print_row(row: &[Letter]) {
    for l in row {
        print!("{}", l);
    }
    println!("");
}
