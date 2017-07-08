mod board;

use board::Board;
use board::Letter;

fn main() {
    let mut board = Board::new();
    board.place_unscored(4, 5, 'c');
    board.place_unscored(10, 5, 'y');
    board.place_unscored(8, 8, 'x');
    board.print();
    
    for (b, score) in board.possible_moves(&vec!['a', 'b', 'c']) {
        b.print();
    }

}

fn print_row(row: &[Letter]) {
    for l in row {
        print!("{}", l);
    }
    println!("");
}
