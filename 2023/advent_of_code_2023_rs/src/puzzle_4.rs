use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let input_path = if cfg!(windows) {
        Path::new("C:/Users/James/Documents/Learning/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle3.txt")
    } else {
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle3.txt")
    };

    let input: Vec<Vec<char>> = read_lines(input_path);
    parse_input(input)
}

fn read_lines(file: &Path) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        let line_chars: Vec<char> = line.chars().collect();
        result.push(line_chars)
    }

    result
}

// Winning numbers are to the left of the |
// Your numbers are to the right of the |
// 1 point for your first match which is doubled for every subsequent match.

fn parse_input() {

}
