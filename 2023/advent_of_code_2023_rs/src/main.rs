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
    let _extract_digits: Vec<u64>;

    let digits_in_calibration_strings: Vec<String> = calibration_strings
        .iter()
        .map(|x| extract_nums_from_string(x))
        .collect();

    for digit_set in digits_in_calibration_strings {
        let digit_set_len = digit_set.len();
        if digit_set_len > 1 {
            // extract_digits.push(&digit_set[0..1] as u64 + &digit_set[digit_set_len - 1..digit_set_len] as u64)
            println!(
                "{}, {}",
                &digit_set[0..1],
                &digit_set[digit_set_len - 1..digit_set_len]
            );
        } else {
            // extract_digits.push(&digit_set[0..1] as u64);
            println!("{}", &digit_set[0..1]);
        }
    }
    100
}

fn main() {
    let input_path =
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/inputs/puzzle1.txt");
    let calibration_strings = read_lines(input_path);
    let _outputs = parse_callibration_values(calibration_strings);
    println!("{:#?}", _outputs);
}
