#[aoc(day2, part1)]
fn part_1(input: &str) -> u32 {
    input.lines().fold(0, |mut acc, line| {
        let delta = (line.as_bytes()[2] as i32 - line.as_bytes()[0] as i32 - 23 + 3) % 3;
        acc += (line.as_bytes()[2] - b'X'
            + 1
            + match delta {
                1 => 6,
                0 => 3,
                _ => 0,
            }) as u32;
        acc
    })
}

#[aoc(day2, part2)]
fn part_2(input: &str) -> i32 {
    input.lines().fold(0, |mut acc, line| {
        acc += match line.as_bytes()[2] {
            b'X' => (line.as_bytes()[0] as i32 - b'A' as i32 - 1 + 3) % 3 + 1,
            b'Y' => 3 + line.as_bytes()[0] as i32 - b'A' as i32 + 1,
            _ => 6 + (line.as_bytes()[0] as i32 - b'A' as i32 + 1) % 3 + 1,
        };

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(INPUT));
    }
}
