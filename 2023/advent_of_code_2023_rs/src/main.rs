use std::fs::read_to_string;
use std::path::Path;

mod part_1 {
    pub fn extract_nums_from_string(a_string: &String) -> String {
        a_string.chars().filter(|x| x.is_digit(10)).collect()
    }

    pub fn parse_callibration_values(calibration_strings: Vec<String>) -> u64 {
        let mut collection_digits_total: u64 = 0;

        let digits_in_calibration_strings: Vec<String> = calibration_strings
            .iter()
            .map(|x| extract_nums_from_string(x))
            .collect();

        for digit_set in digits_in_calibration_strings {
            let digit_set_len = digit_set.len();

            let first_digit = &digit_set[0..1].to_string();
            let second_digit = &digit_set[digit_set_len - 1..digit_set_len].to_string();
            let two_digits_together = format!("{first_digit}{second_digit}");
            let num: u64 = two_digits_together.parse().unwrap();

            collection_digits_total += num;
            println!("{}, {}", num, collection_digits_total);
        }
        collection_digits_total
    }
}

mod part_2 {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct MatchingNumString<'a> {
        index: usize,
        num_string: &'a str,
    }

    fn tuple_to_struct(tuple: (usize, &str)) -> MatchingNumString {
        let matched_num_string = MatchingNumString {
            index: tuple.0,
            num_string: tuple.1,
        };
        matched_num_string
    }

    fn strings_to_numbers(a_string: &String) -> String {
        let nums_as_strings: HashMap<String, &str> = HashMap::from([
            (String::from("one"), "1"),
            (String::from("two"), "2"),
            (String::from("three"), "3"),
            (String::from("four"), "4"),
            (String::from("five"), "5"),
            (String::from("six"), "6"),
            (String::from("seven"), "7"),
            (String::from("eight"), "8"),
            (String::from("nine"), "9"),
        ]);

        let mut new_string: String = String::new();
        let mut nested_num_to_itertions_in_string: Vec<Vec<MatchingNumString>> = Vec::new();

        for num_string in nums_as_strings.keys() {
            let iterations_in_string: Vec<MatchingNumString> = a_string
                .match_indices(num_string)
                .map(|x| tuple_to_struct(x))
                .collect();
            if !iterations_in_string.is_empty() {
                nested_num_to_itertions_in_string.push(iterations_in_string);
            }
        }

        let mut num_to_iterations_in_string: Vec<MatchingNumString> =
            nested_num_to_itertions_in_string
                .into_iter()
                .flatten()
                .collect();

        if !num_to_iterations_in_string.is_empty() {
            num_to_iterations_in_string.sort_by(|b, a| b.index.cmp(&a.index));

            let first_matched_num_string = &num_to_iterations_in_string[0];
            let num_string = first_matched_num_string.num_string;
            let num = nums_as_strings[num_string];
            let replaced_string = a_string.replace(num_string, num);

            if num_to_iterations_in_string.len() > 1 {
                new_string = strings_to_numbers(&replaced_string);
            } else {
                new_string = replaced_string;
            }
        } else {
            return a_string.to_string();
        }

        new_string
    }

    fn extract_nums_from_string(a_string: String) -> String {
        a_string.chars().filter(|x| x.is_digit(10)).collect()
    }

    pub fn parse_callibration_values(calibration_strings: Vec<String>) -> u64 {
        let mut collection_digits_total: u64 = 0;

        let digits_in_calibration_strings: Vec<String> = calibration_strings
            .iter()
            .map(|x| strings_to_numbers(x))
            .map(|x| extract_nums_from_string(x))
            .collect();

        let mut i = 1;
        for digit_set in digits_in_calibration_strings {
            let digit_set_len = digit_set.len();

            let first_digit = &digit_set[0..1].to_string();
            let second_digit = &digit_set[digit_set_len - 1..digit_set_len].to_string();
            let two_digits_together = format!("{first_digit}{second_digit}");
            let num: u64 = two_digits_together.parse().unwrap();

            collection_digits_total += num;
            println!("{} - {}, {}", i, num, collection_digits_total);
            i += 1;
        }
        collection_digits_total
    }
}

fn read_lines(file: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn main() {
    let input_path = if cfg!(windows) {
        Path::new("C:/Users/James/Documents/Learning/Advent_of_Code/2023/inputs/puzzle1.txt")
    } else {
        Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/inputs/puzzle1.txt")
    };
    let calibration_strings = read_lines(input_path);
    let outputs = part_2::parse_callibration_values(calibration_strings);
    println!("{:#?}", outputs);
}
