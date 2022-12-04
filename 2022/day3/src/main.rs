fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|comp| {
            find_index(&comp.0
                .chars()
                .find(|c| comp.1.contains(*c))
                .unwrap())
        }).sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines().collect::<Vec<&str>>().chunks(3)
        .map(|group| {
            find_index(&group.iter().map(|l| l
                    .chars()
                    .find(|c| group.iter().all(|ll| ll.contains(*c)))
                    .unwrap()
            ).next().unwrap())
        }).sum()
}

fn find_index(chr: &char) -> u32 {
    let i = *chr as u32;
    match i <= 90 {
        true => i - 38,
        false => i - 96
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
