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
    rows: [[Letter; 11]; 11],
}

#[derive(Copy, Clone)]
pub struct Letter {
    letter: Option<char>, // The letter
    scored: bool,         // Letter is new and should be scored
}

impl Letter {
    
    pub fn new(letter: char) -> Letter {
        Letter {
            letter: Some(letter),
            scored: true,
        }
    }

    pub fn new_unscored(letter: char) -> Letter {
        Letter {
            letter: Some(letter),
            scored: false,
        }
    }

    pub fn blank() -> Letter {
        Letter {
            letter: None,
            scored: false,
        }
    }

    pub fn score(&self) -> usize {
        let letter = match self.letter {
            Some(c) => c,
            None    => return 0,
        };

        match letter {
            'a' => 1,
            'b' => 4,
            'c' => 4,
            'd' => 2,
            'e' => 1,
            'f' => 4,
            'g' => 3,
            'h' => 3,
            'i' => 1,
            'j' => 10,
            'k' => 5,
            'l' => 2,
            'm' => 4,
            'n' => 2,
            'o' => 1,
            'p' => 4,
            'q' => 10,
            'r' => 1,
            's' => 1,
            't' => 1,
            'u' => 2,
            'v' => 5,
            'w' => 4,
            'x' => 8,
            'y' => 3,
            'z' => 10,
            _  => 0,
        }
    }
}

impl Board {

    pub fn new() -> Board {
        Board {
            rows: [[Letter::blank(); 11]; 11]
        }
    }

    pub fn print(&self) {
        for (i, row) in self.rows.iter().enumerate() {
            for (j, &letter) in row.iter().enumerate() {
                let c = match letter.letter {
                    Some(c) => c,
                    None    => '_',
                };
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
        // Easiest way to do this is probably to just take each letter and 
        // iterate through its possible placements, including a non-placement
        // to get a huge set of nonsense moves and then check rules after
        // (as opposed to the smarted way of building valid moves out
        // by choosing a direction)
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
