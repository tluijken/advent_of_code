fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| match l.as_bytes()[0] as char {
            'n' => (0, 1),
            _ => (l[4..].trim().parse::<i32>().unwrap_or(0), 2),
        })
        .fold(
            (1i32, 0i32, 0i32, 20i32),
            |mut curr, (signal, cycle_size)| {
                (0..cycle_size).for_each(|_| {
                    curr.2 += 1;
                    if curr.3 == curr.2 {
                        curr.1 += curr.2 * curr.0;
                        curr.3 += 40;
                    }
                });
                curr.0 += signal;
                curr
            },
        )
        .1
}

fn part_2(input: &str) -> u32 {
    todo!();
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn test_part_1() {
        assert_eq!(13140, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(INPUT));
    }
}
