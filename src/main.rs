mod board;
mod letter;

use std::cmp::Ordering;

use board::Board;

fn main() {
    let (mut board, letters) = Board::new_from_file("board.dat");
    /*
    board.print();
    for l in letters {
        print!("{} ", l);
    }
    println!("");
    println!("{},{}", board.rows.len(), board.rows[0].len());
    */
    let mut moves = board.possible_moves(&letters);
    moves.sort_by(sort_moves);
    let len = moves.len();
    let mut split = len;
        println!("{}", len);
    if len > 5 {
        println!("{}", len);
        split = 5;
    }
        
    let mut moves = moves.split_off(len - split);

    for &(board, score) in moves.iter() {
        board.print();
        println!("Score: {}\n", score);
    }
}

fn sort_moves(a: &(Board, usize), b: &(Board, usize)) -> Ordering {
    a.1.cmp(&b.1)
}
    

