#[aoc(day12, part1)]
fn part_1(input: &str) -> u32 {
    0
}

#[aoc(day12, part2)]
fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    #[test]
    fn test_part_1() {
        assert_eq!(31, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(70, part_2(INPUT));
    }
}
