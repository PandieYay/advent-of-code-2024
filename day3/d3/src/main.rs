use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("d3.txt").expect("Something went wrong reading the file");
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = regex.find_iter(&contents).map(|mat| mat.as_str()).collect();

    let mut total = 0;
    for mat in matches {
        let numbers: Vec<i32> = Regex::new(r"\d+")
            .unwrap()
            .find_iter(mat)
            .map(|number| number.as_str().parse().unwrap())
            .collect();
        total += numbers[0] * numbers[1];
    }
    println!("Total: {}", total);
}
