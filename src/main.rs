mod board;

use board::Board;

fn main() {
    let letters = vec!['a', 'c', 'c'];
    let mut list = board::permutations(&letters);
    for l in &list {
        println!("{:?}", l);
    }
    println!("\n");
    list.sort();
    for l in &list {
        println!("{:?}", l);
    }
    println!("\n");
    list.dedup();
    for l in &list {
        println!("{:?}", l);
    }
}
