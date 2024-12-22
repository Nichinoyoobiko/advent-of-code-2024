mod data;

use data::INPUT as text;
use regex::Regex;

fn main() {
    let expression = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = expression
        .captures_iter(text)
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
        .map(|pair| pair.0 * pair.1)
        .reduce(|acc, element| acc + element)
        .unwrap_or(0);

    println!("{}", result);
}
