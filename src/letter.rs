use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Letter {
    pub letter: Option<char>, // The letter
    pub scored: bool,         // Letter is new and should be scored
    pub double: bool,         // Letter should be scored double
    pub triple: bool,         // Letter should be scored triple
    pub dw:     bool,         // Letter placed on double word tile
    pub tw:     bool,         // Letter placed on triple word tile
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
            double: false,
            triple: false,
            dw:     false,
            tw:     false,
        }
    }

    pub fn new_unscored(letter: char) -> Letter {
        Letter {
            letter: Some(letter),
            scored: false,
            double: false,
            triple: false,
            dw:     false,
            tw:     false,
        }
    }

    pub fn blank() -> Letter {
        Letter {
            letter: None,
            scored: false,
            double: false,
            triple: false,
            dw:     false,
            tw:     false,
        }
    }

    pub fn is_blank(&self) -> bool {
        self.letter.is_none()
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


