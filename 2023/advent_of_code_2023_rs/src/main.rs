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
        let mut digit_parts: Vec<char> = Vec::new();
        let mut j: i16 = 0;
        let mut j_add_factor = 1;

        while j >= 0 {
            if input[i][j as usize].is_digit(10) {
                let current_digit = input[i][j as usize];
                digit_parts.push(current_digit);
                
                if digit_parts == vec!('2', '5', '8') {
                    println!();
                }

                if check_valid_digit(i as i16, j as i16, &input) == true {
                    let mut rest_of_digit: Vec<char> = get_rest_of_digit(i, j as usize, &input);
                    j_add_factor = rest_of_digit.len() + 1;
                    
                    digit_parts.append(&mut rest_of_digit);

                    let digit_string: String = digit_parts.iter().collect();

                    println!("{}\r\n", digit_string);

                    let digit: u64 = digit_string.parse().unwrap();

                    digit_total += digit;
                }
            } else {
                digit_parts.clear();
            }

            if (j + j_add_factor as i16) < line_length as i16 {
                j += j_add_factor as i16;
                j_add_factor = 1;
            } else {
                j = -1;
            }
        }
    }

    println!("{}", digit_total);
}

fn check_valid_digit(i: i16, j: i16, input: &Vec<Vec<char>>) -> bool {
    let input_length: usize = input.len();
    let line_length: usize = input[0].len();

    let possible_symbols: [char; 10] = ['*', '#', '+', '$', '@', '/', '-', '&', '%', '='];

    if (i == 0) & (j == 0) {
        let positions_to_check: [i16; 2] = [0, 1];

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
    } else if (i == (input_length - 1) as i16) & (j == (line_length - 1) as i16) {
        let positions_to_check: [i16; 2] = [-1, 0];

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
        let line_positions_to_check: [i16; 2] = [0, 1];
        let col_positions_to_check: [i16; 3] = [-1, 0, 1];

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
    } else if j == 0 {
        let line_positions_to_check: [i16; 3] = [-1, 0, 1];
        let col_positions_to_check: [i16; 2] = [0, 1];

        for k in 0..3 {
            for l in 0..2 {
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
    } else if i == (input_length - 1) as i16 {
        let line_positions_to_check: [i16; 2] = [-1, 0];
        let col_positions_to_check: [i16; 3] = [-1, 0, 1];

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
    } else if j == (line_length - 1) as i16 {
        let line_positions_to_check: [i16; 3] = [-1, 0, 1];
        let col_positions_to_check: [i16; 2] = [-1, 0];

        for k in 0..3 {
            for l in 0..2 {
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
    }
    else {
        let positions_to_check: [i16; 3] = [-1, 0, 1];
        for k in 0..3 {
            for l in 0..3 {
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
    }
}

fn get_rest_of_digit(i: usize, j: usize, input: &Vec<Vec<char>>) -> Vec<char> {
    let mut rest_of_digit: Vec<char> = Vec::new();
    for n in 1..3 {
        if input[i][j + n].is_digit(10) {
            rest_of_digit.push(input[i][j + n]);
        } else {
            return rest_of_digit
        }
    }
    rest_of_digit
}
