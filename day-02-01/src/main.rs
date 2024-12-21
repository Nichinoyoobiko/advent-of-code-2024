mod data;

use data::INPUT as text;
use regex::Regex;

fn check_line(numbers: &Vec<i32>) -> bool {
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

fn main() {
    let mut sum = 0;
    for line in text.lines() {
        let numbers = extract_numbers(line);
        sum += if check_line(&numbers) { 1 } else { 0 };
    }

    println!("Result: {}", sum);
}
