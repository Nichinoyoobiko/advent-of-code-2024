mod data;

use data::INPUT as text;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let expression = Regex::new(r"\d+").unwrap();

    let (left, right): (Vec<_>, Vec<_>) = expression
        .find_iter(text)
        .map(|matching_string| matching_string.as_str().parse::<i32>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);

    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for right_element in right {
        let value = count_map.get(&right_element.1).unwrap_or(&0) + 1;
        count_map.insert(right_element.1, value);
    }

    // let keys: Vec<_> = count_map.keys().collect();
    // println!("Keys");
    // for key in keys {
    //     println!("{}", *key);
    // }
    // let values: Vec<_> = count_map.values().collect();
    // println!("Values");
    // for value in values {
    //     println!("{}", *value);
    // }

    let mut sum = 0;
    for left_element in left {
        let key = left_element.1;
        let value = count_map.get(&key).unwrap_or(&0);
        sum += key * (*value);
    }

    println!("Result: {}", sum);
}
