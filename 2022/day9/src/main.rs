fn part_1(input: &str) -> usize {
    let path = path_head(input, 2);
    path.len()
}

fn part_2(input: &str) -> usize {
    let path = path_head(input, 9);
    path.len()
}

fn path_head(input: &str, path_length: usize) -> Vec<(i32, i32)> {
    let mut tail_path = vec![(0, 0)];
    input.lines().fold(
        {
            let mut path_pos = vec![];
            (0..path_length).for_each(|_| path_pos.push((0, 0)));
            path_pos
        },
        |mut path: Vec<(i32, i32)>, m| {
            match (m.chars().nth(0), m[2..].parse::<u32>().unwrap()) {
                (Some('U'), c) => (0..c).for_each(|_| {
                    path[0].1 = path[0].1 + 1;
                    (1..path_length).for_each(|i| {
                        if path[i].1 < path[i - 1].1 - 1 {
                            path[i].1 = path[i - 1].1 - 1;
                            path[i].0 = path[i - 1].0;
                        }
                    });
                    if !tail_path
                        .iter()
                        .any(|y| y.0 == path.last().unwrap().0 && y.1 == path.last().unwrap().1)
                    {
                        tail_path.push((path.last().unwrap().0, path.last().unwrap().1));
                    }
                }),
                (Some('L'), c) => (0..c).for_each(|_| {
                    path[0].0 = path[0].0 - 1;
                    (1..path_length).for_each(|i| {
                        if path[i].0 > path[i - 1].0 + 1 {
                            path[i].0 = path[i - 1].0 + 1;
                            path[i].1 = path[i - 1].1;
                        }
                    });
                    if !tail_path
                        .iter()
                        .any(|y| y.0 == path.last().unwrap().0 && y.1 == path.last().unwrap().1)
                    {
                        tail_path.push((path.last().unwrap().0, path.last().unwrap().1));
                    }
                }),
                (Some('R'), c) => (0..c).for_each(|_| {
                    path[0].0 = path[0].0 + 1;
                    (1..path_length).for_each(|i| {
                        if path[i].0 < path[i - 1].0 - 1 {
                            path[i].0 = path[i - 1].0 - 1;
                            path[i].1 = path[i - 1].1;
                        }
                    });
                    if !tail_path
                        .iter()
                        .any(|y| y.0 == path.last().unwrap().0 && y.1 == path.last().unwrap().1)
                    {
                        tail_path.push((path.last().unwrap().0, path.last().unwrap().1));
                    }
                }),
                (Some('D'), c) => (0..c).for_each(|_| {
                    path[0].1 = path[0].1 - 1;
                    (1..path_length).for_each(|i| {
                        if path[i].1 > path[i - 1].1 + 1 {
                            path[i].1 = path[i - 1].1 + 1;
                            path[i].0 = path[i - 1].0;
                        }
                    });
                    if !tail_path
                        .iter()
                        .any(|y| y.0 == path.last().unwrap().0 && y.1 == path.last().unwrap().1)
                    {
                        tail_path.push((path.last().unwrap().0, path.last().unwrap().1));
                    }
                }),
                _ => println!("Unexpected move"),
            }
            path
        },
    );
    tail_path
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
        // assert_eq!(13, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        //   assert_eq!(1, part_2(INPUT));
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
