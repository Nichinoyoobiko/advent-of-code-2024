mod data;

use data::INPUT as print;
use regex::Regex;

// Return indices sorted by either the left or right side of the rules.
fn sort_by(rules: &Vec<(i32, i32)>) -> Vec<usize> {
    let mut rules = rules.into_iter().enumerate().collect::<Vec<_>>();

    rules.sort_by(|(_, a), (_, b)| {
        return a.1.cmp(&b.1);
    });

    return rules
        .into_iter()
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
}

fn number_check(rules: &Vec<(i32, i32)>, numbers: &Vec<i32>, right_indices: &Vec<usize>) -> i32 {
    for num_index in 0..numbers.len() {
        let number = numbers[num_index];
        // Good case for premature optimization wasted. We operate on sorted indices but then don't exit early. ...
        for check_index in right_indices {
            let check_pair = rules[*check_index];
            if number == check_pair.1 {
                for check_num_index in num_index + 1..numbers.len() {
                    let check_num = numbers[check_num_index];
                    if check_num == check_pair.0 {
                        return 0;
                    }
                }
            }
        }
    }

    return numbers[numbers.len() / 2];
}

fn main() {
    let rule_expression = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let parts = print.split("\n\n").collect::<Vec<_>>();

    let rules = parts[0]
        .lines()
        .map(|line| rule_expression.captures(line).unwrap())
        .map(|captures| (captures.get(1).unwrap(), captures.get(2).unwrap()))
        .map(|(a, b)| {
            (
                a.as_str().parse::<i32>().unwrap_or(-1),
                b.as_str().parse::<i32>().unwrap_or(-1),
            )
        })
        .collect::<Vec<_>>();

    let right_indices = sort_by(&rules);

    // dbg!(left_indices);
    // dbg!(right_indices);

    let mut sum = 0;

    let content = parts[1];
    let lines = content.lines().collect::<Vec<_>>();
    for line in lines {
        let numbers = line
            .split(',')
            .map(|element| element.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<_>>();

        sum += number_check(&rules, &numbers, &right_indices);
    }

    println!("Sum: {}", sum);
}
