use std::fs::read_to_string;
use std::path::Path;

// Notes:
// Game ID does not matter as I can just count up the line numbers.
// Initial assumption for part 1 is that any impossible game will have a value assigned to a colour which wil exceed a threshold.

#[derive(Debug)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

fn extract_nums_from_string(a_string: String) -> u64 {
    let trimmed_string = a_string.trim();
    trimmed_string.get(0..2).unwrap().trim().parse().unwrap()
}

fn get_colour_from_string(a_string: String) -> CubeColour {
    let trimmed_string = a_string.trim();
    let colour = trimmed_string.get(2..).unwrap().trim();
    match colour {
        "red" => CubeColour::Red,
        "green" => CubeColour::Green,
        "blue" => CubeColour::Blue,
        _ => {
            panic!("Not a valid colour!");
        }
    }
}

mod part1 {
    use super::{extract_nums_from_string, get_colour_from_string, CubeColour};

    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    fn parse_game_string(game_string: String) -> bool {
        let split_game_string: Vec<&str> = game_string.split(":").collect();
        let number_of_cube_colour_drawn: Vec<&str> = split_game_string[1]
            .trim()
            .split(|c| c == ',' || c == ';')
            .collect();

        let num_of_cubes: Vec<u64> = number_of_cube_colour_drawn
            .iter()
            .map(|x| extract_nums_from_string(x.to_string()))
            .collect();

        let cube_colours: Vec<CubeColour> = number_of_cube_colour_drawn
            .iter()
            .map(|x| get_colour_from_string(x.to_string()))
            .collect();

        for i in 0..num_of_cubes.len() {
            match cube_colours[i] {
                CubeColour::Red => {
                    if !cube_count_compare(num_of_cubes[i], MAX_RED) {
                        return false;
                    }
                }
                CubeColour::Green => {
                    if !cube_count_compare(num_of_cubes[i], MAX_GREEN) {
                        return false;
                    }
                }
                CubeColour::Blue => {
                    if !cube_count_compare(num_of_cubes[i], MAX_BLUE) {
                        return false;
                    }
                }
                _ => {
                    panic!("Not a valid colour!");
                }
            }
        }

        true
    }

    fn cube_count_compare(count: u64, max: u64) -> bool {
        println!("{} - {}", count, max);
        if count > max {
            false
        } else {
            true
        }
    }

    pub fn possible_game_id_sum(game_strings: Vec<String>) {
        let mut game_id_sum = 0;

        for (i, game_string) in game_strings.iter().enumerate() {
            if parse_game_string(game_string.to_string()) {
                game_id_sum += i + 1;
            }
        }

        println!("{}", game_id_sum)
    }
}

mod part2 {
    use super::{extract_nums_from_string, get_colour_from_string, CubeColour};

    fn parse_game_string(game_string: String) -> u64 {
        let mut max_red: u64 = 0;
        let mut max_green: u64 = 0;
        let mut max_blue: u64 = 0;

        let split_game_string: Vec<&str> = game_string.split(":").collect();
        let number_of_cube_colour_drawn: Vec<&str> = split_game_string[1]
            .trim()
            .split(|c| c == ',' || c == ';')
            .collect();

        let num_of_cubes: Vec<u64> = number_of_cube_colour_drawn
            .iter()
            .map(|x| extract_nums_from_string(x.to_string()))
            .collect();

        let cube_colours: Vec<CubeColour> = number_of_cube_colour_drawn
            .iter()
            .map(|x| get_colour_from_string(x.to_string()))
            .collect();

        for i in 0..num_of_cubes.len() {
            match cube_colours[i] {
                CubeColour::Red => {
                    max_red = if num_of_cubes[i] > max_red {
                        num_of_cubes[i]
                    } else {
                        max_red
                    }
                }
                CubeColour::Green => {
                    max_green = if num_of_cubes[i] > max_green {
                        num_of_cubes[i]
                    } else {
                        max_green
                    }
                }
                CubeColour::Blue => {
                    max_blue = if num_of_cubes[i] > max_blue {
                        num_of_cubes[i]
                    } else {
                        max_blue
                    }
                }
                _ => {
                    panic!("Not a valid colour!");
                }
            }
        }
        max_red * max_green * max_blue
    }

    pub fn game_power_sum(input_strings: Vec<String>) {
        let game_powers_summed: u64 = input_strings
            .iter()
            .map(|x| parse_game_string(x.to_string()))
            .sum();
        println!("{}", game_powers_summed);
    }
}

fn main() {
    let input_path = if cfg!(windows) {
        Path::new("SUPER_AMAZING_WINDOWS_PATH!")
    } else {
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle2.txt")
    };

    let input_strings: Vec<String> = read_lines(input_path);
    // part1::possible_game_id_sum(input_strings);
    part2::game_power_sum(input_strings);
}

fn read_lines(file: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}
