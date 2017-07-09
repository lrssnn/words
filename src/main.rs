mod board;
mod letter;

use std::cmp::Ordering;

use board::Board;

fn main() {
    let mut board = Board::new_from_file("board.dat");
    board.print();
    println!("{},{}", board.rows.len(), board.rows[0].len());
    /*
    board.place_unscored(4, 5, 'c');
    board.place_unscored(10, 5, 'y');
    board.place_unscored(8, 8, 'x');
    */
    
    let letters = vec!['b', 't', 'q', 'i', 'o', 'e', 'x'];
    let mut moves = board.possible_moves(&letters);
    moves.sort_by(sort_moves);
    //moves.reverse();

    for &(board, score) in moves.iter().take(5){
        board.print();
        println!("Score: {}\n", score);
    }
}

fn sort_moves(a: &(Board, usize), b: &(Board, usize)) -> Ordering {
    a.1.cmp(&b.1).reverse()
}
    

