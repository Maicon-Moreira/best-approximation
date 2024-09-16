#[derive(Clone, Debug)]
pub struct Char {
    possible_next_chars: Vec<char>,
}

impl Char {
    pub fn new(c: Option<char>) -> Self {
        static INIT_CHARS: &[char] = &['-', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        static DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        static NON_ZERO_DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        static NEXT_AFTER_DIGIT: &[char] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', '*', '/', '%', '^',
        ];
        if let Some(c_some) = c {
            if c_some.is_ascii_digit() {
                return Self {
                    possible_next_chars: NEXT_AFTER_DIGIT.to_vec(),
                };
            }
            if "+-*/%^".contains(c_some) {
                return Self {
                    possible_next_chars: NON_ZERO_DIGITS.to_vec(),
                };
            }
            if c_some == '.' {
                return Self {
                    possible_next_chars: DIGITS.to_vec(),
                };
            }
            panic!("Invalid char: {c_some}");
        } else {
            return Self {
                possible_next_chars: INIT_CHARS.to_vec(),
            };
        }
    }

    pub fn get_possible_next_chars(&self) -> Vec<char> {
        return self.possible_next_chars.clone();
    }
}
