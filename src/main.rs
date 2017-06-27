mod board;

use board::Board;

fn main() {
   let board = Board::new_from_file("board.dat");
   board.print();
   println!("Size: {}", board.size());
}
