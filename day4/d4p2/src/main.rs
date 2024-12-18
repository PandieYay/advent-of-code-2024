use std::fs;

fn check_diagonal_down_left(word_search_bytes: &[u8], i: usize, length_of_one_line: usize) -> bool {
    if (i + length_of_one_line * 2 - 2) < word_search_bytes.len() {
        if word_search_bytes[i + length_of_one_line - 1] == b'A'
            && word_search_bytes[i + length_of_one_line * 2 - 2] == b'S'
        {
            return true;
        }
    }
    return false;
}

fn check_diagonal_down_right(
    word_search_bytes: &[u8],
    i: usize,
    length_of_one_line: usize,
) -> bool {
    if (i + length_of_one_line * 2 + 2) < word_search_bytes.len() {
        if word_search_bytes[i + length_of_one_line + 1] == b'A'
            && word_search_bytes[i + length_of_one_line * 2 + 2] == b'S'
        {
            return true;
        }
    }
    return false;
}

fn check_diagonal_up_left(word_search_bytes: &[u8], i: usize, length_of_one_line: usize) -> bool {
    if (i as isize - (length_of_one_line * 2) as isize - 2) >= 0 {
        if word_search_bytes[i - length_of_one_line - 1] == b'A'
            && word_search_bytes[i - length_of_one_line * 2 - 2] == b'S'
        {
            return true;
        }
    }
    return false;
}

fn check_diagonal_up_right(word_search_bytes: &[u8], i: usize, length_of_one_line: usize) -> bool {
    if (i as isize - (length_of_one_line * 2) as isize + 2) >= 0 {
        if word_search_bytes[i - length_of_one_line + 1] == b'A'
            && word_search_bytes[i - length_of_one_line * 2 + 2] == b'S'
        {
            return true;
        }
    }
    return false;
}

fn main() {
    let word_search = fs::read_to_string("d4.txt").expect("Something went wrong reading the file");

    let mut xmas_counter = 0;
    let word_search_bytes = word_search.as_bytes();
    let length_of_one_line = word_search.find("\n").unwrap() + 1;
    for i in 0..word_search_bytes.len() {
        if word_search_bytes[i] == b'M' {
            // Check left and right Ms
            if i < word_search.len() - 2 {
                if word_search_bytes[i + 2] == b'M' {
                    if check_diagonal_down_left(word_search_bytes, i + 2, length_of_one_line)
                        && check_diagonal_down_right(word_search_bytes, i, length_of_one_line)
                    {
                        xmas_counter += 1;
                    }
                    if check_diagonal_up_left(word_search_bytes, i + 2, length_of_one_line)
                        && check_diagonal_up_right(word_search_bytes, i, length_of_one_line)
                    {
                        xmas_counter += 1;
                    }
                }
            }
            // Check up and down Ms
            if (i + length_of_one_line * 2) < word_search_bytes.len() {
                if word_search_bytes[i + length_of_one_line * 2] == b'M' {
                    if check_diagonal_down_right(word_search_bytes, i, length_of_one_line)
                        && check_diagonal_up_right(
                            word_search_bytes,
                            i + length_of_one_line * 2,
                            length_of_one_line,
                        )
                    {
                        xmas_counter += 1;
                    }
                    if check_diagonal_down_left(word_search_bytes, i, length_of_one_line)
                        && check_diagonal_up_left(
                            word_search_bytes,
                            i + length_of_one_line * 2,
                            length_of_one_line,
                        )
                    {
                        xmas_counter += 1;
                    }
                }
            }
        }
    }
    println!("Total XMAS: {}", xmas_counter);
}
