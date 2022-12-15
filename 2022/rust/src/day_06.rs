#[aoc(day6, part1)]
fn part_1(input: &str) -> usize {
    get_distinct_sequence(input, 4)
}

#[aoc(day6, part2)]
fn part_2(input: &str) -> usize {
    get_distinct_sequence(input, 14)
}

fn get_distinct_sequence(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| !w.iter().any(|c| w.iter().filter(|cc| cc == &c).count() > 1))
        .map(|(i, _)| i)
        .unwrap()
        + window_size
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(19, part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part_2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
