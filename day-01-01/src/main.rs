mod data;

use data::INPUT as text;
use regex::Regex;

fn debug_print_vec(vector : &Vec<(usize, i32)>) {
    for element in vector {
        println!("{}", element.1);
    }
    println!("----------------");
}

fn main() {
    let expression = Regex::new(r"\d+").unwrap();

    let (mut left, mut right): (Vec<_>, Vec<_>) = expression
        .find_iter(text)
        .map(|matching_string| matching_string.as_str().parse::<i32>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);

    left.sort_by(|(_, number_a), (_, number_b)| number_a.cmp(number_b));
    // debug_print_vec(&left);

    right.sort_by(|(_, number_a), (_, number_b)| number_a.cmp(number_b));
    // debug_print_vec(&right);

    let result : u32 = std::iter::zip(left, right)
        .map(|((_, number_a), (_, number_b))| number_a.abs_diff(number_b))
        .sum();

    println!("Result: {}", result);
}
