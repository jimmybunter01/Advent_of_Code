
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
    use std::{collections::HashMap};

    #[derive(Debug)]
    struct MatchingNumString<'a> {
        index: usize,
        num_string: &'a str
    }

    fn tuple_to_struct(tuple: (usize, &str)) -> MatchingNumString {
        let matched_num_string = MatchingNumString {
            index: tuple.0,
            num_string: tuple.1
        };
        matched_num_string
    }

    fn strings_to_numbers(a_string: &String) -> String {
        let nums_as_strings: HashMap<String, String> = HashMap::from([
            (String::from("one"), String::from("1"))
            , (String::from("two"), String::from("2"))
            , (String::from("three"), String::from("3"))
            , (String::from("four"), String::from("4"))
            , (String::from("five"), String::from("5"))
            , (String::from("six"), String::from("6"))
            , (String::from("seven"), String::from("7"))
            , (String::from("eight"), String::from("8"))
            , (String::from("nine"), String::from("9"))
        ]);

        let new_string: String = String::from(a_string);
        let mut nested_num_to_itertions_in_string : Vec<Vec<MatchingNumString>> = Vec::new();

        for num_string in nums_as_strings.keys() {
            let iterations_in_string: Vec<MatchingNumString> = new_string.match_indices(num_string).map(|x| tuple_to_struct(x)).collect();
            if !iterations_in_string.is_empty() {
                nested_num_to_itertions_in_string.push(iterations_in_string);
            }
        }
        
        let mut num_to_iterations_in_string : Vec<MatchingNumString> = nested_num_to_itertions_in_string.into_iter().flatten().collect();
        num_to_iterations_in_string.sort_by(|b, a| b.index.cmp(&a.index));

        println!("{:#?}\r\n---", num_to_iterations_in_string);
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
    
        for digit_set in digits_in_calibration_strings {
            let digit_set_len = digit_set.len();
    
            let first_digit = &digit_set[0..1].to_string();
            let second_digit = &digit_set[digit_set_len - 1..digit_set_len].to_string();
            let two_digits_together = format!("{first_digit}{second_digit}");
            let num: u64 = two_digits_together.parse().unwrap();
    
            collection_digits_total += num;
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
    let input_path = Path::new("C:/Users/James/Documents/Learning/Advent_of_Code/2023/inputs/puzzle1.txt");
    let calibration_strings = read_lines(input_path);
    let outputs = part_2::parse_callibration_values(calibration_strings);
    println!("{:#?}", outputs);
}
