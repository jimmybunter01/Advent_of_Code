use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let input_path = if cfg!(windows) {
        Path::new("C:/Users/James/Documents/Learning/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle3.txt")
    } else {
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle3.txt")
    };

    let input: Vec<Vec<char>> = read_lines(input_path);
    // part1::parse_input(input)
    part2::parse_input(input)
}

fn read_lines(file: &Path) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        let line_chars: Vec<char> = line.chars().collect();
        result.push(line_chars)
    }

    result
}

mod part1 {
    pub fn parse_input(input: Vec<Vec<char>>) {
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
        } else {
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
                return rest_of_digit;
            }
        }
        rest_of_digit
    }
}

mod part2 {
    // Find the star
    pub fn parse_input(input: Vec<Vec<char>>) {
        let line_length = input[0].len();
        let input_length = input.len();

        let mut gear_ratio_total: u64 = 0;

        for i in 0..input_length {
            let mut j: i16 = 0;

            while j >= 0 {
                if input[i][j as usize] == '*' {
                    let digits = check_if_valid_gear(i, j as usize, &input, line_length);
                    if !digits.0.is_empty() & !digits.1.is_empty() {
                        let digit_1: String = digits.0.iter().collect();
                        let digit_2: String = digits.1.iter().collect();

                        let digit_1 : u64 = digit_1.parse().unwrap();
                        let digit_2 : u64 = digit_2.parse().unwrap();

                        gear_ratio_total += digit_1 * digit_2;
                        println!("{} * {} = {} - At location [{},{}]", digit_1, digit_2, gear_ratio_total, i,j);
                    }
                }

                if j as usize + 1 < line_length {
                    j += 1;
                } else {
                    j = - 1;
                }
            }
        }
        println!("{}", gear_ratio_total)
    }

    fn check_if_valid_gear(i: usize, j: usize, input: &Vec<Vec<char>>, line_length: usize) -> (Vec<char>, Vec<char>) {
        
        let input_len = input.len();
        let mut value_of_digits_found: (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());
        
        let positions_to_check: [i16; 3] = [-1, 0, 1];
        
        let mut k = 0;
        let mut l = 0;

        let mut digits_around_gear = 0;

        while k < 3 {
            while l < 3 {
                let _line_to_check: i16 = i as i16 + positions_to_check[k];
                let _col_to_check: i16 = j as i16 + positions_to_check[l];

                let line_to_check = _line_to_check as usize;
                let col_to_check = _col_to_check as usize;
                let mut l_add_factor = 1;

                if (line_to_check >= 0 as usize) & (line_to_check < input_len) & (col_to_check >= 0 as usize) & (col_to_check < line_length) {    
                    let mut digit: Vec<char> = Vec::new();
                    let char_to_check = input[line_to_check][col_to_check];

                    if char_to_check.is_digit(10) {
                        digits_around_gear += 1;

                        if digits_around_gear > 2 {
                            value_of_digits_found.0 = Vec::new();
                            value_of_digits_found.1 = Vec::new();
                            break;
                        }

                        digit = check_left(line_to_check, col_to_check, &input);
                        let mut digits_to_right: Vec<char> = check_right(line_to_check, col_to_check, &input, line_length);
                        l_add_factor = digits_to_right.len() + 1;
                        digit.push(char_to_check);
                        digit.append(&mut digits_to_right);
                        

                        if value_of_digits_found.0.is_empty() {
                            value_of_digits_found.0 = digit;    
                        } else {
                            value_of_digits_found.1 = digit;
                        }
                    }
                }
                l += l_add_factor;
            }
            k+=1;
            l=0;
        }
        value_of_digits_found
    }
        

    fn check_left(start_line: usize, start_col: usize, input: &Vec<Vec<char>>) -> Vec<char> {
        let mut digits_to_the_left: Vec<char> = Vec::new();
        let mut n = 1;

        while n > 0 {
            let col_to_check: i16 = start_col as i16 - n;

            if col_to_check >= 0 {
                let char_to_check = input[start_line][col_to_check as usize];

                if char_to_check.is_digit(10) {
                    digits_to_the_left.insert(0, char_to_check);
                    n += 1;
                } else {
                    n = 0;
                }
            }
            else {
                n = 0;
            } 
        }

        digits_to_the_left
    }

    fn check_right(start_line: usize, start_col: usize, input: &Vec<Vec<char>>, line_length: usize) -> Vec<char> {
        let mut digits_to_the_right: Vec<char> = Vec::new();
        let mut n = 1;

        while n > 0 {
            let col_to_check: usize = start_col + n;

            if col_to_check < line_length {
                let char_to_check = input[start_line][col_to_check];

                if char_to_check.is_digit(10) {
                    digits_to_the_right.push(char_to_check);
                    n += 1;
                } else {
                    n = 0;
                }
            } else {
                n = 0;
            }
        }
        digits_to_the_right
    }
}
