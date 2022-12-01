use std::collections::HashMap;

fn part_1(input: &str) -> u32 {
    *get_sorted_weights(input).first().unwrap_or(&u32::MIN)
}

fn part_2(input: &str) -> u32 {
    get_sorted_weights(input)[..3].iter().fold(0, |acc, i| acc + i)
}

fn get_sorted_weights(input: &str) -> Vec<u32> {
    let mut result = input
        .lines()
        .map(|l| l.trim().parse::<u32>())
        .fold(HashMap::new(), |mut acc, parsed| {
            match parsed {
                Ok(weight) => *acc.entry(std::cmp::max(0, acc.len() as isize - 1) as usize).or_insert(0) += weight,
                _ => {
                    acc.insert(acc.len(), 0);
                }
            }
            acc
        })
        .into_values()
        .collect::<Vec<u32>>();
    result.sort_by(|a, b| b.cmp(a));
    result
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
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
        assert_eq!(24000, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(INPUT));
    }
}
