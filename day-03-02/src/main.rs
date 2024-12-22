mod data;

use data::INPUT as text;
use regex::Regex;

const DONT: &str = "don't()";
const DO: &str = "do()";

fn extract_from_slice(slc: &str) -> i32 {
    let expression = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    return expression
        .captures_iter(slc)
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
            )
        })
        .map(|pair| {
            (
                pair.0.parse::<i32>().unwrap_or(0),
                pair.1.parse::<i32>().unwrap_or(0),
            )
        })
        .inspect(|pair|println!("{}x{}", pair.0, pair.1))
        .map(|pair| pair.0 * pair.1)
        .reduce(|acc, element| acc + element)
        .unwrap_or(0);
}

fn main() {
    let mut include = true;
    let mut string_start = 0;
    let mut sum = 0;
    loop {
        if include {
            let include_index = text[string_start..].find(DONT);
            let include_index = (include_index.unwrap_or(text.len()) + string_start).min(text.len());
            println!("Index from {} to {}", string_start, include_index);
            sum += extract_from_slice(&text[string_start..include_index]);
            string_start = include_index + DONT.len();
        } else {
            let exclude_index = text[string_start..].find(DO);
            let exclude_index = (exclude_index.unwrap_or(text.len()) + string_start).min(text.len());
            println!("Index from {} to {}", string_start, exclude_index);
            string_start = exclude_index + DO.len();
        }
        if string_start >= text.len() {
            break;
        }
        include = !include;
    }

    println!("{}", sum);
}
