#[aoc_generator(day1)]
fn get_sorted_weights(input: &str) -> Vec<u32> {
    let mut result =
        input
            .lines()
            .map(|l| l.trim().parse::<u32>())
            .fold(vec![0], |mut acc, parsed| {
                match parsed {
                    Ok(weight) => *acc.iter_mut().last().unwrap_or(&mut 0) += weight,
                    _ => {
                        acc.push(0);
                    }
                }
                acc
            });
    result.sort_by(|a, b| b.cmp(a));
    result
}

#[aoc(day1, part1)]
fn part_1(input: &[u32]) -> u32 {
    input[0]
}
#[aoc(day1, part2)]
fn part_2(input: &[u32]) -> u32 {
    input.iter().take(3).sum()
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
        assert_eq!(24000, part_1(&get_sorted_weights(INPUT)));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(&get_sorted_weights(INPUT)));
    }
}
