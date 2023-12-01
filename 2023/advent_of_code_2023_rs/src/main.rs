use std::fs::read_to_string;
use std::path::Path;

fn read_lines(file: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn extract_nums_from_string(a_string: &String) -> String {
    a_string.chars().filter(|x| x.is_digit(10)).collect()
}

fn parse_callibration_values(calibration_strings: Vec<String>) -> u64 {
    let mut collection_digits_total: u64 = 0;

    let digits_in_calibration_strings: Vec<String> = calibration_strings
        .iter()
        .map(|x| extract_nums_from_string(x))
        .collect();

    for digit_set in digits_in_calibration_strings {
        let digit_set_len = digit_set.len();

        let first_num = &digit_set[0..1].to_string();
        let second_num = &digit_set[digit_set_len - 1..digit_set_len].to_string();

        let digit_one: u64 = first_num.parse().unwrap();
        let digit_two: u64 = second_num.parse().unwrap();
        collection_digits_total += digit_one + digit_two;
        println!("{}+{}={}", digit_one, digit_two, collection_digits_total);
    }
    collection_digits_total
}

fn main() {
    let input_path =
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/inputs/puzzle1.txt");
    let calibration_strings = read_lines(input_path);
    let _outputs = parse_callibration_values(calibration_strings);
    println!("{:#?}", _outputs);
}
