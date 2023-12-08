use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let input_path = if cfg!(windows) {
        Path::new("SUPER_AMAZING_WINDOWS_PATH!")
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

fn parse_input(input: Vec<Vec<char>>) {
    let line_length = input[0].len();

    for i in 0..input.len() {
        for j in 0..line_length {
            if input[i][j].is_digit(10) {
                check_valid_digit(i as i8, j as i8, &input);
            }
        }
    }
}

fn check_valid_digit(i: i8, j: i8, input: &Vec<Vec<char>>) {
    let input_length = input.len();
    let line_length = input[0].len();
    let positions_to_check: [i8; 3] = [-1, 0, 1];

    if (i > 0) & (j > 0) {
        for k in 0..3 {
            for l in 0..3 {
                let char_to_check = input[(i + positions_to_check[k]) as usize]
                    [(i + positions_to_check[l]) as usize];
                println!("{}", char_to_check);
            }
        }
    }
}
