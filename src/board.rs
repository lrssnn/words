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
    rows: [[char; 11]; 11],
}

impl Board {

    pub fn new() -> Board {
        Board {
            rows: [['_'; 11]; 11]
        }
    }

    pub fn print(&self) {
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
