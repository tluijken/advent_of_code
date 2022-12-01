use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    let result = get_sorted_weights(input);
    match result.first() {
        Some(max) => max.to_owned(),
        None => 0,
    }
}

pub fn part_2(input: &str) -> u32 {
    let result = get_sorted_weights(input);
    let last_weight: &u32 = &result[..3].iter().fold(0, |total, item| item + total);
    last_weight.to_owned()
}

fn get_sorted_weights(input: &str) -> Vec<u32> {
    let mut result = input
        .lines()
        .map(|l| l.trim())
        .fold(HashMap::new(), |mut acc, line| match line.is_empty() {
            true => {
                acc.insert(acc.keys().len(), 0);
                acc
            }
            false => {
                let index = match acc.is_empty() {
                    true => 0,
                    false => acc.len() - 1,
                };
                let entry = acc.entry(index).or_insert(0);
                *entry += line.parse::<u32>().unwrap_or(0);
                acc
            }
        })
        .values()
        .map(|v| v.to_owned())
        .collect::<Vec<u32>>();
    result.sort();
    result.reverse();
    result
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let result = part_1(&input);
    println!("{}", result);

    let result = part_2(&input);
    println!("{}", result);
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";
    #[test]
    fn test_part_1() {
        let output = part_1(INPUT);
        assert_eq!(24000, output);
    }
    #[test]
    fn test_part_2() {
        let output = part_2(INPUT);
        assert_eq!(45000, output);
    }
}
