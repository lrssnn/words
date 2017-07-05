mod board;

use board::Board;

fn main() {
    let letters1 = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let list = board::choose_n(&letters1, 2);
    for l in list {
        println!("{:?}", l);
    }
}
