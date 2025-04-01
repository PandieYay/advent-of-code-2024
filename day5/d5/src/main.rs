use regex::Regex;
use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("d5.txt").expect("Something went wrong reading the file");
    let regex = Regex::new(r"\d{1,2}\|\d{1,2}").unwrap();
    let matches: Vec<_> = regex
        .find_iter(&puzzle_input)
        .map(|mat| mat.as_str())
        .collect();

    let mut rules_array = Vec::new();
    for mat in matches {
        let mut rules= mat.split("|").collect::<Vec<&str>>();
        rules_array.push([rules[0].parse::<i32>().unwrap(), rules[1].parse::<i32>().unwrap()]);
    }

    // Split the actual puzzle
    let puzzles = puzzle_input.split("\r\n\r\n").collect::<Vec<&str>>()[1];
    let mut total = 0;
    for puzzle in puzzles.split("\r\n") {
        let mut puzzle = puzzle.split(",").collect::<Vec<&str>>();
        let mut valid = true;
        for i in 0..puzzle.len() {
            let num = puzzle[i].parse::<i32>().unwrap();

            // Check that the number does not violate any of the rules backwards and forwards
            // First check if any number on the right is going before the left number
            let matching_rules: Vec<&[i32; 2]> = rules_array.iter().filter(|&rule| rule[0] == num).collect();
            if !matching_rules.is_empty() {
                for i2 in 0..i {
                    let num2 = puzzle[i2].parse::<i32>().unwrap();
                    let matching_rules2= matching_rules.iter().any(|&rule| rule[1] == num2);
                    if matching_rules2 {
                        valid = false;
                        break;
                    }
                }
            }
        }
        if valid {
            println!("Yippee {:?}", puzzle);
            total += puzzle[(puzzle.len() + 1) / 2 - 1].parse::<i32>().unwrap();
        }
    }
    println!("Total: {:?}", total);
}
