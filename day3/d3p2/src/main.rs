use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("d3.txt").expect("Something went wrong reading the file");

    let mul_pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    let mut total = 0;
    let mut mul_enabled = true;
    let mut index = 0;

    // Think there is a faster way to this using find, TODO
    while index < contents.len() {
        if let Some(mat) = do_pattern.find(&contents[index..]) {
            if mat.start() == 0 {
                mul_enabled = true;
                index += mat.end();
                continue;
            }
        }

        if let Some(mat) = dont_pattern.find(&contents[index..]) {
            if mat.start() == 0 {
                mul_enabled = false;
                index += mat.end();
                continue;
            }
        }

        if let Some(mat) = mul_pattern.find(&contents[index..]) {
            if mat.start() == 0 && mul_enabled {
                let numbers: Vec<i32> = Regex::new(r"\d+")
                    .unwrap()
                    .find_iter(mat.as_str())
                    .map(|number| number.as_str().parse().unwrap())
                    .collect();
                total += numbers[0] * numbers[1];
                index += mat.end();
            } else {
                index += 1;
            }
        } else {
            index += 1;
        }
    }

    println!("Total: {}", total);
}