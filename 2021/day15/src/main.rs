use crate::input::Input;

fn main() {
    const INPUT: &str = include_str!("../input");

    let mut input = Input::from_str(INPUT);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);

    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}

mod input {
    use std::collections::BinaryHeap;
    pub struct Input {
        risk_map: Vec<Vec<i32>>,
    }

    impl Input {
        pub fn problem1(&mut self) -> i32 {
            self.find_safest_route()
        }

        pub fn problem2(&mut self) -> i32 {
            // expand the riskmap
            self.risk_map = (0..(5 * self.risk_map.len()))
                .map(|x| {
                    (0..(5 * self.risk_map[0].len()))
                        .map(|y| {
                            let cost = self.risk_map[x % self.risk_map.len()]
                                [y % self.risk_map[0].len()]
                                + (x / self.risk_map.len()) as i32
                                + (y / self.risk_map[0].len()) as i32;
                            if cost < 10 {
                                cost
                            } else {
                                cost - 9
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            self.find_safest_route()
        }

        pub fn from_str(input: &str) -> Self {
            let risk_map = input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|r| r.to_digit(10).unwrap() as i32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Self { risk_map }
        }

        pub fn find_safest_route(&self) -> i32 {
            let goal = (&self.risk_map.len() - 1, &self.risk_map[0].len() - 1);
            let mut visited = vec![vec![i32::MAX; self.risk_map[0].len()]; self.risk_map.len()];
            let mut current = BinaryHeap::new();
            current.push((0, 0, 0));
            while let Some((cost, x, y)) = current.pop() {
                if (x, y) == goal {
                    return -cost;
                }
                if -cost > visited[x][y] {
                    continue;
                }
                let mut neighbors = [(x + 1, y), (x, y + 1)].to_vec();
                if x > 0 {
                    neighbors.push((x - 1, y));
                }
                if y > 0 {
                    neighbors.push((x, y - 1));
                }

                for (x1, y1) in neighbors {
                    let next_cost = match self.risk_map.get(x1).and_then(|row| row.get(y1)) {
                        Some(c) => -cost + c,
                        None => continue,
                    };
                    if next_cost < visited[x1][y1] {
                        current.push((-next_cost, x1, y1));
                        visited[x1][y1] = next_cost;
                    }
                }
            }
            i32::MAX
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 40;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 315;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
