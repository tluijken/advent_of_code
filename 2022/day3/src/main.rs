fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|comp| {
            comp.0
                .chars()
                .find(|c| comp.1.chars().any(|cc| &cc == c))
                .unwrap()
        })
        .map(|common| find_index(&common))
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines().collect::<Vec<&str>>().chunks(3)
        .map(|group| {
             group.iter().map(|l| l
                    .chars()
                    .find(|c| group.iter().all(|ll| ll.chars().any(|cc| &cc == c)))
                    .unwrap()
            ).collect::<Vec<char>>().first().unwrap().to_owned()
        })
        .map(|common| find_index(&common))
        .sum()
}

fn find_index(chr: &char) -> usize {
    match ('a'..='z').position(|c| &c == chr) {
        Some(lower_index) => lower_index + 1,
        None => match ('A'..='Z').position(|c| &c == chr) {
            Some(upper_index) => upper_index + 27,
            None => 0,
        },
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_part_1() {
        assert_eq!(157, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(70, part_2(INPUT));
    }
}
