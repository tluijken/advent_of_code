use std::collections::HashSet;

fn part_1(input: &str) -> usize {
    let path = path_head(input, 1);
    path.len()
}

fn part_2(input: &str) -> usize {
    let path = path_head(input, 9);
    path.len()
}

fn path_head(input: &str, followers: usize) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    input
        .lines()
        .map(|l| {
            let count = l[2..].parse::<u32>().unwrap();
            match l.as_bytes()[0] as char {
                'U' => ((0, 1), count),
                'D' => ((0, -1), count),
                'R' => ((1, 0), count),
                'L' => ((-1, 0), count),
                _ => unreachable!(),
            }
        })
        .fold(
            vec![(0i32, 0i32); followers + 1],
            |mut rope, ((x, y), len)| {
                (0..len).for_each(|_| {
                    rope[0] = (rope[0].0 + x, rope[0].1 + y);
                    (1..rope.len()).for_each(|i| {
                        let (x, y) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                        if x.abs() > 1 || y.abs() > 1 {
                            rope[i].0 += x.signum();
                            rope[i].1 += y.signum();
                        }
                    });
                    visited.insert(rope[followers]);
                });
                rope
            },
        );
    visited
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn test_part_1() {
        assert_eq!(13, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2(INPUT));
        assert_eq!(
            36,
            part_2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )
        );
    }
}
