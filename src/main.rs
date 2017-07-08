mod board;

use std::cmp::Ordering;

use board::Board;

fn main() {
    let mut board = Board::new();
    board.place_unscored(4, 5, 'c');
    board.place_unscored(10, 5, 'y');
    board.place_unscored(8, 8, 'x');
    board.print();
    
    let mut moves = board.possible_moves(&vec!['a', 'b', 'c']);
    moves.sort_by(sort_moves);

    for &(board, score) in moves.iter().rev().take(5) {
        board.print();
        println!("Score: {}\n", score);
    }

}

fn sort_moves(a: &(Board, usize), b: &(Board, usize)) -> Ordering {
    a.1.cmp(&b.1)
}
    

