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

fn parse_input(input: Vec<Vec<char>>) {
    let line_length = input[0].len();
    let mut digit_total = 0;

    for i in 0..input.len() {

        let mut digit_parts_so_far: Vec<u16> = Vec::new();
        
        for j in 0..line_length {
            if input[i][j].is_digit(10) {
                let current_digit = input[i][j].to_digit(10).unwrap() as u16;
                digit_parts_so_far.push(current_digit);

                let valid_digit = check_valid_digit(i as i8, j as i8, &input);

            } else {
                current_digit_parts.clear();
            }
        }
    }

    println!("{}", digit_total);
}

fn check_valid_digit(i: i8, j: i8, input: &Vec<Vec<char>>) -> bool {
    let input_length: usize = input.len();
    let line_length: usize = input[0].len();
    
    let possible_symbols: [char; 4] = ['*', '#', '+', '$'];

    if (i == 0) & (j == 0) {
        let positions_to_check: [i8; 2] = [0, 1];
        
        for k in 0..2 {
            for l in 0..2 {
                let line_to_check: usize = (i + positions_to_check[k]) as usize;    
                let col_to_check: usize = (j + positions_to_check[l]) as usize;    

                let char_to_check = input[line_to_check][col_to_check];

                for symbol in possible_symbols.iter() {
                    if char_to_check == *symbol {
                        return true;
                    } 
                }

            }
        }
        false
    } else if (i == (input_length - 1) as i8) & (j == (line_length - 1) as i8) {
        let positions_to_check: [i8; 2] = [-1, 0];
        
        for k in 0..2 {
            for l in 0..2 {
                let line_to_check: usize = (i + positions_to_check[k]) as usize;    
                let col_to_check: usize = (j + positions_to_check[l]) as usize;    

                let char_to_check = input[line_to_check][col_to_check];
                
                for symbol in possible_symbols.iter() {
                    if char_to_check == *symbol {
                        return true;
                    } 
                }
            }
        }
        false
    } else if i == 0 {
        let line_positions_to_check: [i8; 2] = [0, 1];
        let col_positions_to_check: [i8; 3] = [-1, 0, 1];        

        for k in 0..2 {
            for l in 0..3 {
                let line_to_check: usize = (i + line_positions_to_check[k]) as usize;    
                let col_to_check: usize = (j + col_positions_to_check[l]) as usize;    

                let char_to_check = input[line_to_check][col_to_check];
                
                for symbol in possible_symbols.iter() {
                    if char_to_check == *symbol {
                        return true;
                    } 
                }

            }
        }

        false
    } else {
        let positions_to_check: [i8; 3] = [-1, 0, 1];
        for k in 0..3 {
            for l in 0..3 {
                
                let line_to_check: usize = (i + positions_to_check[k]) as usize;    
                let col_to_check: usize = (j + positions_to_check[l]) as usize;    
                
                if (line_to_check < input_length) & (col_to_check < line_length) {
                    let char_to_check = input[line_to_check][col_to_check];
                    for symbol in possible_symbols.iter() {
                        if char_to_check == *symbol {
                            return true;
                        } else {
                            return false;
                        }
                    }
                } 
                else {
                    return false;
                }
            }
        }
        false
    }
}

fn get_digit(i: usize, j: usize, input: &Vec<Vec<char>>) -> u16 {
    let extra_digit_parts = 
    for i in 1..3 {
        if input[i][j+i].is_digit(10) {

        }
    }

    
}