#![allow(needless_range_loop)]
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;
use std::fmt;
use std::clone::Clone;

use super::letter;
use letter::Letter;

extern crate hunspell;
use self::hunspell::Hunspell;

const DL : [(usize, usize); 12] = [(2, 2), (2, 4), (2, 6), (2, 8),
                                  (4, 2), (4, 8), (6, 2), (6, 8),
                                  (8, 2), (8, 4), (8, 6), (8, 8)];

const DW : [(usize, usize); 8] = [(1, 1), (1, 5), (1, 9), (5, 1),
                                  (5, 9), (9, 1), (9, 5), (9, 9)];

const TL : [(usize, usize); 8] = [(0, 0), (0, 10), (3, 3), (3, 7),
                                  (7, 3), (7, 7), (10, 0), (10, 10)];

const TW : [(usize, usize); 8] = [(0, 2), (0, 8), (2, 0), (2, 10),
                                  (8, 0), (8, 10), (10, 2), (10, 8)];

#[derive(Copy, Clone)]
pub struct Board {
    pub rows: [[Letter; 11]; 11],
}


impl Board {

    pub fn new() -> Board {
        Board {
            rows: [[Letter::blank(); 11]; 11]
        }
    }

    pub fn new_from_file(file: &str) -> Board {
        let mut file = File::open(file).unwrap();

        let mut string = String::new();
        file.read_to_string(&mut string);

        let mut res = Board::new();

        for (i, line) in string.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c != '_' {
                    res.place_unscored(i, j, c);
                }
            }
        }
        res
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
                    /*
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
                    */
                    print!("_{}_", '_');
                } else {
                    if letter.scored {
                        print!("*{}*", c);
                    } else {
                        print!("_{}_", c);
                    }
                }
            }
            println!("");
        }
    }
    
    pub fn place(&mut self, i: usize, j: usize, letter: char) {
        let l = Letter::new(letter);
        if self.rows[i][j].letter.is_some() {
            println!("WARNING: Placing tile over existing tile");
        }
        self.rows[i][j] = l;
    }

    pub fn place_unscored(&mut self, i: usize, j: usize, letter: char) {
        let l = Letter::new_unscored(letter);
        if self.rows[i][j].letter.is_some() {
            println!("WARNING: Placing tile over existing tile");
        }
        self.rows[i][j] = l;
    }

    // Returns a list of valid moves and their scores
    pub fn possible_moves(&self, letters: &Vec<char>) -> Vec<(Board, usize)> {
        // We need to choose:
        //   1. How many letters to put down
        //   2. Which letters to use
        //   3. Which order to use
        //   4. Where to start
        // With those in mind we should have every possible move?
        //
        // So lets start with one letter words which will just be putting them adjacent
        //  to letters
        let mut result = vec![];
        let spell = Hunspell::new("en_GB.aff", "en_GB.dic");
        let mut checks = 0;
        for word_length in 1..letters.len() + 1 {
            println!("Word Length: {}", word_length);
            for letter_selection in choose_n(letters, word_length) {
                //println!("Letters: {:?}", letter_selection);
                for perm in permutations(&letter_selection) {
                    //println!("Perm: {:?}", perm);
                    // Here is where we want to look at both rows and columns
                    for (row_num, row) in self.rows.iter().enumerate() {
                        //println!("{}", row_num);
                        //println!("----");
                        for start_cell in start_positions(row, word_length) {
                            //println!("{},{}", row_num, start_cell);
                            let (mut opt, legal) = self.put_word(&perm, row_num, start_cell);
                            if legal {
                                checks += 1; 
                                let score = opt.score(&spell);
                                result.push((opt, score));
                            }
                        }
                    }

                    if word_length == 1 {
                        continue;
                    }

                    for (col_num, col) in cols_to_rows(&self.rows).iter().enumerate() {
                        //println!("----");
                        for start_cell in start_positions(col, word_length) {
                            let (mut opt, legal) = self.put_word_v(&perm, col_num, start_cell);
                            if legal {
                                let score = opt.score(&spell);
                                result.push((opt, score));
                            }
                        }
                    }
                }
                print!("\rChecks: {}", checks);
                io::stdout().flush();
            }
            checks = 0;
            println!("");
        }
        result
    }

    pub fn score(&mut self, spellcheck: &Hunspell) -> usize {

        let mut score = 0;
        let mut word_score = 0;
        let mut doubled = 1;
        let mut tripled = 1;

        let mut word = String::new();

        for row in &mut self.rows {
            for letter in row {
                if letter.is_blank() {
                    // Double out
                    while doubled > 1 {
                        word_score *= 2;
                        doubled    -= 1;
                    }
                    // Triple out
                    while tripled > 1 {
                        word_score *= 3;
                        tripled    -= 1;
                    }
                    // Add to total
                    score += word_score;
                    word_score = 0;

                    // Spellcheck
                    if word.len() > 1 {
                        let valid = spellcheck.check(&word);
                        if valid {println!("Spellchecking '{}': {}", word, valid);}
                        if !valid {
                            return 0;
                        }
                        word = String::new();
                    }

                } else if letter.scored {
                    letter.scored = false;
                    let mut letter_score = letter.score();
                    word.push(letter.letter.unwrap());
                    if letter.double { letter_score *= 2;}
                    if letter.triple { letter_score *= 3;}
                    if letter.dw { 
                        letter.dw = false;
                        doubled += 1;
                    }
                    if letter.tw {
                        letter.tw = false;
                        tripled += 1;
                    }
                    
                    word_score += letter_score;
                }
            }
            // Cash out words that hit the edge
            // Double out
            while doubled > 1 {
                word_score *= 2;
                doubled    -= 1;
            }
            // Triple out
            while tripled > 1 {
                word_score *= 3;
                tripled    -= 1;
            }
            // Add to total
            score += word_score;
            word_score = 0;

            if word.len() > 1 {
                let valid = spellcheck.check(&word);
                        if valid {println!("Spellchecking '{}': {}", word, valid);}
                if !valid {
                    return 0;
                }
                word = String::new();
            }
        }

        // Do the same thing for columns
        for row in &mut cols_to_rows(&self.rows) {
            for letter in row {
                if letter.is_blank() {
                    // Double out
                    while doubled > 1 {
                        word_score *= 2;
                        doubled    -= 1;
                    }
                    // Triple out
                    while tripled > 1 {
                        word_score *= 3;
                        tripled    -= 1;
                    }
                    // Add to total
                    score += word_score;
                    word_score = 0;
                } else if letter.scored {
                    letter.scored = false;
                    let mut letter_score = letter.score();
                    if letter.double { letter_score *= 2;}
                    if letter.triple { letter_score *= 3;}
                    if letter.dw { 
                        letter.dw = false;
                        doubled += 1;
                    }
                    if letter.tw {
                        letter.tw = false;
                        tripled += 1;
                    }
                    
                    word_score += letter_score;
                }
            }
            // Cash out words that hit the edge
            // Double out
            while doubled > 1 {
                word_score *= 2;
                doubled    -= 1;
            }
            // Triple out
            while tripled > 1 {
                word_score *= 3;
                tripled    -= 1;
            }
            // Add to total
            score += word_score;
            word_score = 0;
        }
        score
    }

    fn put_word_v(&self, letters: &[char], row: usize, start_cell: usize) -> (Board, bool) {
        let mut board = *self;
        board.rows = cols_to_rows(&board.rows);
        let (mut res, legal) = board.put_word(letters, row, start_cell);
        res.rows = cols_to_rows(&res.rows);
        (res, legal)
    }

    fn put_word(&self, letters: &[char], row: usize, start_cell: usize) -> (Board, bool) {
        let mut legal = false;
        let mut board = *self;
        let mut i = start_cell;
        // Check for a word left of the word beginning
        if i > 0 {
            let mut j = 1;
            loop {
                if board.rows[row][i - j].letter.is_some() {
                    //if !legal {println!("Legalising on pre-word")}
                    legal = true;
                    board.rows[row][i - j].scored = true;
                    j += 1;
                    if j >= i {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        for letter in letters {
            // Skip letters in the row (but mark to be scored)
            while board.rows[row][i].letter.is_some() {
                //if !legal {println!("Legalising on mid-word")}
                legal = true;
                board.rows[row][i].scored = true;
                i += 1;
            }
            // Place the letter
            let mut input = board.rows[row][i];
            input.letter = Some(*letter);
            input.scored = true;

            // Check for multipliers
            if is_dl(row, i) { input.double = true };
            if is_dw(row, i) { input.dw     = true };
            if is_tl(row, i) { input.triple = true };
            if is_tw(row, i) { input.tw     = true };

            board.rows[row][i] = input;

            // Check for abutting words up and down
            // Checking up is uglier to avoid usize subtraction overflow
            if row > 0 {
                let mut j = 1;
                loop {
                    if board.rows[row-j][i].letter.is_some() {
                        //if !legal {println!("Legalising on up-word")}
                        legal = true;
                        board.rows[row-j][i].scored = true;
                        j += 1;
                        if j > row {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            let mut j = 1;
            while row+j < board.rows.len() && board.rows[row+j][i].letter.is_some() {
                //if !legal {println!("Legalising on down-word")}
                legal = true;
                board.rows[row+j][i].scored = true;
                j += 1;
            }

            i += 1;
        }

        // Check for a word right of the end
        let mut j = 0;
        while i + j < board.rows[row].len() && board.rows[row][i + j].letter.is_some() {
            //if !legal {println!("Legalising on post-word")}
            legal = true;
            board.rows[row][i+j].scored = true;
            j += 1;
        }
        (board, legal)
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
           res.push(vec![*l]);
       }
       return res;
    }

    // Build the sublists recursively:
    let mut result = vec![];
    // Take each element in the list as the first element in a set of sublists
    // The last 'first' element is the one that leaves n - 1 elements after it
    for first in 0 .. len - (n - 1) {
        let l = letters[first];
        // Create the sublist that goes from the element after 'first' to the end
        let sublist = letters.clone()[first+1..len].to_vec();
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
// Removes duplicate orderings i.e. [a, b, b] will r]eturn
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

pub fn cols_to_rows(cols: &[[Letter; 11]; 11]) -> [[Letter; 11]; 11] {
    let mut rows = *cols;

    for i in 0 .. cols.len(){
        for j in 0 .. cols[i].len() {
            rows[j][i] = cols[i][j];
        }
    }
    rows
}

fn is_dl(i: usize, j: usize) -> bool {
    for tup in &DL {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    false
}

fn is_dw(i: usize, j: usize) -> bool {
    for tup in &DW {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    false
}

fn is_tl(i: usize, j: usize) -> bool {
    for tup in &TL {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    false
}

fn is_tw(i: usize, j: usize) -> bool {
    for tup in &TW {
        let &(row, col) = tup;
        if i == row && j == col {
            return true;
        }
    }
    false
}

fn dot() {
    print!(".");
    io::stdout().flush();
}
