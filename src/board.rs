use std::fs::File;
use std::io::Read;
use std::fmt;

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
    pub rows: [[Letter; 11]; 11],
}

#[derive(Copy, Clone, Debug)]
pub struct Letter {
    letter: Option<char>, // The letter
    scored: bool,         // Letter is new and should be scored
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}.", self.letter.unwrap_or('_'))
    }
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

    // Print with special tile indicators, only useful for 11x11
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

    pub fn get_best_move(&self, letters: &Vec<char>) -> Board {
        let options = self.possible_moves(letters);

        Board::new()
    }

    // Returns a list of valid moves and their scores
    pub fn possible_moves(&self, letters: &Vec<char>) -> Vec<(Board, usize)> {
        // Iterate through each row and put the permutations in it
        // Need to do each column as well which is not a particularly
        // efficient thing to do.
        // Not sure how to track this
        
        for row in self.rows.iter() {
            // We need to choose:
            //   1. How many letters to put down
            //   2. Which letters to use
            //   3. Which order to use
            //   4. Where to start
            // With those in mind we should have every possible move?
            //
            // So lets start with one letter words which will just be putting them adjacent
            //  to letters
            
            /*
            for word_length in 1..letters.len() {
               for letter_selection in choose_n(letters, word_length) {
                   for perm in permutations(letter_selection) {
                       for start_cell in start_positions(row, word_length) {
                           println!("{}: {}", perm, start_cell);
                       }
                   }
               }
            }
            */
        }
        vec![]
    }

}

// Return every possible sublist of letters length n (maintaining order).
// i.e. [a, b, c] choose 2 will return [a, b], [a, c] and [b, c], and not [c, b] etc. 
// Recursive function
pub fn choose_n(letters: &Vec<char>, n: usize) -> Vec<Vec<char>> {
    let len = letters.len();

    // Base cases
    // Invalid call
    if n > len {
        println!("Invalid call to choose_n");
        return vec![];
    }

    // n choose n: the input is the only output
    if n == len {
        return vec![letters.clone()];
    }

    // n choose 1: each element of the input is an output
    if n == 1 {
       let mut res = vec![];
       for l in letters {
           res.push(vec![l.clone()]);
       }
       return res;
    }

    // Build the sublists recursively:
    let mut result = vec![];
    // Take each element in the list as the first element in a set of sublists
    // The last 'first' element is the one that leaves n - 1 elements after it
    for first in (0 .. len - (n - 1)) {
        let l = letters[first];
        // Create the sublist that goes from the element after 'first' to the end
        let mut sublist = letters.clone().get(first+1..len).unwrap().to_vec();
        // For each way to choose n - 1 from that sublist, put 'first' at the start and
        // add to the result
        for mut r in choose_n(&sublist, n - 1) {
            let mut res = vec![l];
            res.append(&mut r);
            result.push(res);
        }
    } 
    result
}

// Returns every permutation (ordering) of the given list
// Removes duplicate orderings i.e. [a, b, b] will return
//    [a, b, b], [b, a, b] and [b, b, a] only
pub fn permutations(list: &Vec<char>) -> Vec<Vec<char>> {
    let len = list.len();
    // Base case
    // perms length 1 or 0: input is the only output
    if len <= 1 {
        return vec![list.clone()];
    }

    // perms length 2: input and input swapped is the output
    // This isn't strictly necessary but is a small optimisation
    if len == 2 {
        return vec![list.clone(), vec![list[1], list[0]]];
    }

    // Calculate permutations recursively by taking each element of the list
    // as the first element and appending each permutation of the list without
    // that element
    let mut result = vec![];
    for first in 0..len {
        let mut sublist = list.clone();
        let l = sublist.remove(first);
        
        for mut p in permutations(&sublist) {
            let mut res = vec![l];
            res.append(&mut p);
            result.push(res);
        }
    }
    // Dedup only considers adjacent elements so sort first
    result.sort();
    result.dedup();
    result
}

// Return a list of possible start positions to place 'word_size' letters in sequence into 
// 'row'.
// Tile placement is considered 1 by 1 from left to right, skipping over tiles that are occupied,
// so word_size = 3 can still yield longer words than 3 based on tiles already in the board
pub fn start_positions(row: &[Letter], word_size: usize) -> Vec<usize> {
    let mut res = vec![];
    for start in 0 .. (row.len() - word_size)+1 {
        // Can't start on an occupied cell
        if row[start].letter.is_some() {
            continue;
        }

        // 1 letter words don't need any more checks
        if word_size <= 1 {
            res.push(start);
            continue;
        }

        // Count unoccupied cells after start cell
        let mut blanks = 1;
        for i in start + 1 .. row.len() {
            if row[i].letter.is_none() {
                blanks += 1;
                if blanks == word_size {
                    res.push(start);
                    continue;
                }
            }
        }
    }
    res
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
