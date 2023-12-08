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
    struct NumLocation {
        index: usize,
        num: u64,
    }

    fn tuple_to_struct(tuple: (usize, &str)) -> NumLocation {
        let nums_as_strings: HashMap<String, u64> = HashMap::from([
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
            (String::from("four"), 4),
            (String::from("five"), 5),
            (String::from("six"), 6),
            (String::from("seven"), 7),
            (String::from("eight"), 8),
            (String::from("nine"), 9),
        ]);
        let mut num = 0;

        if tuple.1.len() > 1 {
            num = nums_as_strings[tuple.1];
        } else {
            num = tuple.1.chars().nth(0).unwrap().to_digit(10).unwrap() as u64;
        }

        let matched_num_string = NumLocation {
            index: tuple.0,
            num: num,
        };
        matched_num_string
    }

    fn get_string_digits(a_string: &String) -> u64 {
        let nums_as_strings: HashMap<String, char> = HashMap::from([
            (String::from("one"), '1'),
            (String::from("two"), '2'),
            (String::from("three"), '3'),
            (String::from("four"), '4'),
            (String::from("five"), '5'),
            (String::from("six"), '6'),
            (String::from("seven"), '7'),
            (String::from("eight"), '8'),
            (String::from("nine"), '9'),
        ]);

        let mut nested_num_to_itertions_in_string: Vec<Vec<NumLocation>> = Vec::new();

        for (num_string, num) in nums_as_strings.iter() {
            let mut all_number_in_string = Vec::new();

            let mut num_string_iterations_in_string: Vec<NumLocation> = a_string
                .match_indices(num_string)
                .into_iter()
                .map(|x| tuple_to_struct(x))
                .collect();

            all_number_in_string.append(&mut num_string_iterations_in_string);

            let mut num_iterations_in_string: Vec<NumLocation> = a_string
                .match_indices(*num)
                .into_iter()
                .map(|x| tuple_to_struct(x))
                .collect();

            all_number_in_string.append(&mut num_iterations_in_string);

            if !all_number_in_string.is_empty() {
                nested_num_to_itertions_in_string.push(all_number_in_string);
            }
        }

        let mut num_to_iterations_in_string: Vec<NumLocation> = nested_num_to_itertions_in_string
            .into_iter()
            .flatten()
            .collect();
        num_to_iterations_in_string.sort_by_key(|x| x.index);

        let mut num_loc_iter = num_to_iterations_in_string.iter();
        let first_loc = num_loc_iter.next().unwrap();
        let first_digit = first_loc.num;

        let last_loc = num_to_iterations_in_string.pop().unwrap();
        let last_digit = last_loc.num;

        let two_digits_together = format!("{first_digit}{last_digit}");
        let num: u64 = two_digits_together.parse().unwrap();

        println!("{:#?} - {}", num_to_iterations_in_string, num);
        num
    }

    pub fn parse_callibration_values(calibration_strings: Vec<String>) -> u64 {
        // let mut collection_digits_total: u64 = 0;

        let collection_digits_total: u64 = calibration_strings
            .iter()
            .map(|x| get_string_digits(x))
            .sum();

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
    let input_path = Path::new("/Users/james/Documents/GitHub/Advent_of_Code/2023/advent_of_code_2023_rs/inputs/puzzle1.txt");
    let calibration_strings = read_lines(input_path);
    let outputs = part_2::parse_callibration_values(calibration_strings);
    println!("{:#?}", outputs);
}
