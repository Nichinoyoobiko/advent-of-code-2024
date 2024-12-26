mod data;

use data::INPUT as print;
use multimap::MultiMap;
use regex::Regex;
use std::num::ParseIntError;

// TODO: Learn if all the "unwraps here are really necessary. It feels like the way null propagates in Optionals in Java should be possible here as well?!

// Map with right-rule-values as keys.
fn map_right_values(rules: &Vec<(i32, i32)>) -> MultiMap<i32, i32> {
    let mut map = MultiMap::new();

    for rule in rules {
        map.insert(rule.1, rule.0);
    }

    return map;
}

fn number_check(right_map: &MultiMap<i32, i32>, numbers: &Vec<i32>) -> bool {
    for num_index in 0..numbers.len() {
        let number = numbers[num_index];
        for check_index in num_index..numbers.len() {
            let check = numbers[check_index];
            if right_map
                .get_vec(&number)
                .unwrap_or(&Vec::<i32>::new())
                .into_iter()
                .any(|next| check == *next)
            {
                return false;
            }
        }
    }

    return true;
}

fn number_sort(right_map: &MultiMap<i32, i32>, numbers: &mut Vec<i32>) -> i32 {
    // Be aware that we get reverse order here - from what is expected from the example - but we anyway only care about the "middle value".
    numbers.sort_by(|a, b| {
        if right_map
            .get_vec(a)
            .unwrap_or(&Vec::<i32>::new())
            .into_iter()
            .any(|next| *b == *next)
        {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    });

    dbg!(&numbers);

    return numbers[numbers.len() / 2];
}

fn main() -> Result<(), ParseIntError> {
    let rule_expression = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let parts = print.split("\n\n").collect::<Vec<_>>();

    let mut rules: Vec<(i32, i32)> = vec![];
    for (_, [left, right]) in rule_expression
        .captures_iter(parts[0])
        .map(|captures| captures.extract())
    {
        rules.push((left.parse()?, right.parse()?));
    }

    let right_map = map_right_values(&rules);

    // dbg!(right_map);

    let mut wrong_lines = Vec::new();

    let content = parts[1];
    let lines = content.lines().collect::<Vec<_>>();
    for line in lines {
        let numbers = line
            .split(',')
            .filter_map(|element| element.parse::<i32>().ok())
            .collect::<Vec<_>>();

        if !number_check(&right_map, &numbers) {
            wrong_lines.push(numbers);
        }
    }

    // dbg!(&wrong_lines);

    let mut sum = 0;
    for mut line in wrong_lines {
        sum += number_sort(&right_map, &mut line);
    }

    println!("{}", sum);

    Ok(())
}
