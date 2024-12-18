// This format has been sponsored by copilot refactoring main.rs, just using this for reference.
use std::fs;

struct WordSearch {
    bytes: Vec<u8>,
    length_of_one_line: usize,
}

impl WordSearch {
    fn new(content: &str) -> Self {
        let bytes = content.as_bytes().to_vec();
        let length_of_one_line = content.find('\n').unwrap() + 1;
        WordSearch {
            bytes,
            length_of_one_line,
        }
    }

    fn check_diagonal_down_left(&self, index: usize) -> bool {
        if (index + self.length_of_one_line * 2 - 2) < self.bytes.len() {
            if self.bytes[index + self.length_of_one_line - 1] == b'A'
                && self.bytes[index + self.length_of_one_line * 2 - 2] == b'S'
            {
                return true;
            }
        }
        false
    }

    fn check_diagonal_down_right(&self, index: usize) -> bool {
        if (index + self.length_of_one_line * 2 + 2) < self.bytes.len() {
            if self.bytes[index + self.length_of_one_line + 1] == b'A'
                && self.bytes[index + self.length_of_one_line * 2 + 2] == b'S'
            {
                return true;
            }
        }
        false
    }

    fn check_diagonal_up_left(&self, index: usize) -> bool {
        if (index as isize - (self.length_of_one_line * 2) as isize - 2) >= 0 {
            if self.bytes[index - self.length_of_one_line - 1] == b'A'
                && self.bytes[index - self.length_of_one_line * 2 - 2] == b'S'
            {
                return true;
            }
        }
        false
    }

    fn check_diagonal_up_right(&self, index: usize) -> bool {
        if (index as isize - (self.length_of_one_line * 2) as isize + 2) >= 0 {
            if self.bytes[index - self.length_of_one_line + 1] == b'A'
                && self.bytes[index - self.length_of_one_line * 2 + 2] == b'S'
            {
                return true;
            }
        }
        false
    }
}

fn main() {
    let word_search_content = fs::read_to_string("d4.txt").expect("Something went wrong reading the file");
    let word_search = WordSearch::new(&word_search_content);

    let mut xmas_counter = 0;
    for i in 0..word_search.bytes.len() {
        if word_search.bytes[i] == b'M' {
            // Check left and right Ms
            if i < word_search.bytes.len() - 2 {
                if word_search.bytes[i + 2] == b'M' {
                    if word_search.check_diagonal_down_left(i + 2)
                        && word_search.check_diagonal_down_right(i)
                    {
                        xmas_counter += 1;
                    }
                    if word_search.check_diagonal_up_left(i + 2)
                        && word_search.check_diagonal_up_right(i)
                    {
                        xmas_counter += 1;
                    }
                }
            }
            // Check up and down Ms
            if (i + word_search.length_of_one_line * 2) < word_search.bytes.len() {
                if word_search.bytes[i + word_search.length_of_one_line * 2] == b'M' {
                    if word_search.check_diagonal_down_right(i)
                        && word_search.check_diagonal_up_right(i + word_search.length_of_one_line * 2)
                    {
                        xmas_counter += 1;
                    }
                    if word_search.check_diagonal_down_left(i)
                        && word_search.check_diagonal_up_left(i + word_search.length_of_one_line * 2)
                    {
                        xmas_counter += 1;
                    }
                }
            }
        }
    }
    println!("Total XMAS: {}", xmas_counter);
}