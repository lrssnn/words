use std::fs::File;
use std::io::Read;

const DL : [(usize, usize); 12] = [(2, 2), (2, 4), (2, 6), (2, 8),
                                  (4, 2), (4, 8), (6, 2), (6, 8),
                                  (8, 2), (8, 4), (8, 6), (8, 8)];

const DW : [(usize, usize); 8] = [(1, 1), (1, 5), (1, 9), (5, 1),
                                  (5, 9), (9, 1), (9, 5), (9, 9)];

const TL : [(usize, usize); 8] = [(0, 0), (0, 10), (3, 3), (3, 7),
                                  (7, 3), (7, 7), (10, 0), (10, 10)];

const TW : [(usize, usize); 8] = [(0, 2), (0, 8), (2, 0), (2, 10),
                                  (8, 0), (8, 10), (10, 2), (10, 8)];

pub struct Board {
    rows: Vec<Vec<char>>,
    size: usize,
}


impl Board {

    pub fn new() -> Board {
        Board {
            rows: vec![
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
                vec!['_','_','_','_','_','_','_','_','_','_','_'],
            ],
            size: 11,
        }
    }

    pub fn new_from_file(file: &str) -> Board {
        let mut file = File::open(file).expect(&format!("File '{}' not found", file)); 

        let mut rows = vec![];

        let mut string = String::new();
        file.read_to_string(&mut string).expect("File read error");
        let strings = string.split_whitespace();
        for row in strings {
            rows.push(row.chars().collect());
        }

        Board {
           size: rows.len(),
           rows: rows,
        }
    }

    pub fn print(&self) {
        if self.size == 11 {
            self.print_full();
        } else {
            for row in &self.rows {
                for c in row {
                    print!("_{}_", c);
                }
                println!();
            }
        }
    }

    // Print with special tile indicators, only useful for 11x11
    fn print_full(&self) {
        for (i, row) in self.rows.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '_' {
                    // Check for multiplier tiles
                    if i == 5 && j == 5 {
                        print!("_*_");
                    } else if is_dl(i, j) {
                        print!("_d_");
                    } else if is_dw(i, j) {
                        print!("_D_");
                    } else if is_tl(i, j) {
                        print!("_t_");
                    } else if is_tw(i, j) {
                        print!("_T_");
                    } else {
                        print!("_{}_", ' ');
                    }
                } else {
                    print!("_{}_", c);
                }
            }
            println!("");
        }
    }

    pub fn possible_moves(&self) {
        // Iterate through each row and put the permutations in it
        // Need to do each column as well which is not a particularly
        // efficient thing to do.
        //
        // Not sure how to track this
    }

    pub fn size(&self) -> usize {
        self.size
    }

}

fn is_dl(i: usize, j: usize) -> bool {
    for tup in DL.iter() {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    return false;
}

fn is_dw(i: usize, j: usize) -> bool {
    for tup in DW.iter() {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    return false;
}

fn is_tl(i: usize, j: usize) -> bool {
    for tup in TL.iter() {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    return false;
}

fn is_tw(i: usize, j: usize) -> bool {
    for tup in TW.iter() {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    return false;
}
