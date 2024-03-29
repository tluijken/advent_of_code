#[aoc(day3, part1)]
fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first_half, second_half)| {
            get_prio(
                &first_half
                    .chars()
                    .find(|c| second_half.contains(*c))
                    .unwrap(),
            )
        })
        .sum()
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            get_prio(
                &group
                    .iter()
                    .map(|l| {
                        l.chars()
                            .find(|c| group.iter().all(|ll| ll.contains(*c)))
                            .unwrap()
                    })
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

fn get_prio(chr: &char) -> u32 {
    let i = *chr as u32;
    match i <= 90 {
        true => i - 38,
        false => i - 96,
    }
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
