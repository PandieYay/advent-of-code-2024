use std::fs;

fn main() {
    let word_search = fs::read_to_string("d4.txt").expect("Something went wrong reading the file");

    let mut xmas_counter = 0;
    let word_search_bytes = word_search.as_bytes();
    let length_of_one_line = word_search.find("\n").unwrap() + 1;
    for i in 0..word_search_bytes.len() {
        if word_search_bytes[i] == b'X' {
            // Check forward
            if i < word_search.len() - 3 {
                if word_search_bytes[i + 1] == b'M'
                    && word_search_bytes[i + 2] == b'A'
                    && word_search_bytes[i + 3] == b'S'
                {
                    println!("Found forward XMAS");
                    xmas_counter += 1;
                }
            }
            // Check backward
            if i >= 3 {
                if word_search_bytes[i - 1] == b'M'
                    && word_search_bytes[i - 2] == b'A'
                    && word_search_bytes[i - 3] == b'S'
                {
                    println!("Found backward XMAS");
                    xmas_counter += 1;
                }
            }
            // Check down
            if (i + length_of_one_line * 3) <  word_search_bytes.len() {
                if word_search_bytes[i + length_of_one_line] == b'M'
                    && word_search_bytes[i + length_of_one_line * 2] == b'A'
                    && word_search_bytes[i + length_of_one_line * 3] == b'S'
                {
                    println!("Found down XMAS");
                    xmas_counter += 1;
                }
            }
            // Check up
            if (i as isize - (length_of_one_line * 3) as isize) >= 0 {
                if word_search_bytes[i - length_of_one_line] == b'M'
                    && word_search_bytes[i - length_of_one_line * 2] == b'A'
                    && word_search_bytes[i - length_of_one_line * 3] == b'S'
                {
                    println!("Found up XMAS");
                    xmas_counter += 1;
                }
            }
            // Check diagonal down left
            if (i + length_of_one_line * 3 - 3) <  word_search_bytes.len() {
                if word_search_bytes[i + length_of_one_line - 1] == b'M'
                    && word_search_bytes[i + length_of_one_line * 2 - 2] == b'A'
                    && word_search_bytes[i + length_of_one_line * 3 - 3] == b'S'
                {
                    println!("Found diagonal down left XMAS");
                    xmas_counter += 1;
                }
            }
            // Check diagonal down right
            if (i + length_of_one_line * 3 + 3) <  word_search_bytes.len() {
                if word_search_bytes[i + length_of_one_line + 1] == b'M'
                    && word_search_bytes[i + length_of_one_line * 2 + 2] == b'A'
                    && word_search_bytes[i + length_of_one_line * 3 + 3] == b'S'
                {
                    println!("Found diagonal down right XMAS");
                    xmas_counter += 1;
                }
            }
            // Check diagonal up left
            if (i as isize - (length_of_one_line * 3) as isize - 3) >= 0 {
                if word_search_bytes[i - length_of_one_line - 1] == b'M'
                    && word_search_bytes[i - length_of_one_line * 2 - 2] == b'A'
                    && word_search_bytes[i - length_of_one_line * 3 - 3] == b'S'
                {
                    println!("Found diagonal up left XMAS");
                    xmas_counter += 1;
                }
            }
            // Check diagonal up right
            if (i as isize - (length_of_one_line * 3) as isize + 3) >= 0 {
                if word_search_bytes[i - length_of_one_line + 1] == b'M'
                    && word_search_bytes[i - length_of_one_line * 2 + 2] == b'A'
                    && word_search_bytes[i - length_of_one_line * 3 + 3] == b'S'
                {
                    println!("Found diagonal up right XMAS");
                    xmas_counter += 1;
                }
            }
        }
    }
    println!("Total XMAS: {}", xmas_counter);
}
