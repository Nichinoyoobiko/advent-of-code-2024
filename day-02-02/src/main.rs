mod data;

use data::INPUT as text;
use regex::Regex;
// use std::fs;


fn check_line(numbers: &Vec<i32>/* , debug_string: &mut String */) -> bool {
    let mut previous = numbers[0];
    let mut prev_sign = 0;
    for index in 1..numbers.len() {
        let number = numbers[index];
        let diff = previous - number;
        if diff.abs() < 1 || diff.abs() > 3 || prev_sign * diff.signum() < 0 {
            return false;
        }
        previous = number;
        prev_sign = diff.signum();
    }

    return true;
}

fn extract_numbers(line: &str) -> Vec<i32> {
    let expression = Regex::new(r"\d+").unwrap();

    return expression
        .find_iter(line)
        .map(|matching_string| matching_string.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
}

// fn write_to_file(data: &str) {
//     // let data = "Some data!";
//     fs::write("result.txt", data).expect("Unable to write file");
// }

fn main() {
    // let mut debug_string = String::new();
    let mut sum = 0;
    for line in text.lines() {
        let mut skip_index = 0;
        let numbers = &extract_numbers(line);
        let number_count = numbers.len();
        // Brute force line check with a single number missing in case the check was unsuccessful.
        while skip_index < number_count {
            let clean_numbers = numbers
                .into_iter()
                .enumerate()
                .filter(|indexed| indexed.0 != skip_index)
                .map(|indexed| *indexed.1)
                .collect::<Vec<_>>();

            if check_line(&clean_numbers/* , &mut debug_string */) {
                sum += 1;
                break;
            }
            skip_index += 1;
        }
    }

    // write_to_file(debug_string.as_str());
    println!("Sum: {}", sum);
}
