#[aoc(day13, part1)]
fn part_1(input: &str) -> u32 {
    0
}
#[aoc(day13, part2)]
fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    #[test]
    fn test_part_1() {
        assert_eq!(24000, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(INPUT));
    }
}
