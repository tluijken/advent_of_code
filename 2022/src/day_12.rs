use std::collections::HashMap;

#[aoc_generator(day12)]
fn get_heightmap(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let result = input.lines().enumerate().fold(vec![], |mut acc, (i, row)| {
        acc.push(row.chars().enumerate().fold(vec![], |mut acc2, (j, c)| {
            match c {
                'S' => {
                    start = (i, j);
                    acc2.push(b'a');
                }
                'E' => {
                    end = (i, j);
                    acc2.push(b'z');
                }
                _ => acc2.push(c as u8),
            }
            acc2
        }));
        acc
    });
    (result, start, end)
}

fn get_neighbors((x, y): (usize, usize), height_map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    vec![(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)]
        .iter()
        .map(|(hx, vy)| (x as i32 + hx, y as i32 + vy))
        .into_iter()
        .filter(|(x, y)| x >= &0 && y >= &0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|(x, y)| match height_map.get(*x) {
            Some(x) => x.get(*y).is_some(),
            _ => false,
        })
        .filter(|(nx, ny)| height_map[*nx][*ny] as i32 - height_map[x][y] as i32 <= 1)
        .collect()
}

#[aoc(day12, part1)]
fn part_1(input: &(Vec<Vec<u8>>, (usize, usize), (usize, usize))) -> u64 {
    let cost = calculate_route_costs(input);
    cost.get(&input.2).unwrap_or(&0).clone()
}

fn calculate_route_costs(
    (heighmap, start, _): &(Vec<Vec<u8>>, (usize, usize), (usize, usize)),
) -> HashMap<(usize, usize), u64> {
    let mut q = vec![start.clone()];
    let mut path: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut cost: HashMap<(usize, usize), u64> = HashMap::new();
    path.insert(start.clone(), None);
    cost.insert(start.clone(), 0);
    while !q.is_empty() {
        let curr = q.pop().unwrap();
        for n in get_neighbors(curr, heighmap) {
            let new_cost = cost.get(&curr).unwrap() + 1;
            if cost.get(&n).unwrap_or(&u64::MAX) > &new_cost {
                cost.insert(n, new_cost);
                q.push(n);
                path.insert(n, Some(curr));
            }
        }
    }
    cost
}

#[aoc(day12, part2)]
fn part_2(input: &(Vec<Vec<u8>>, (usize, usize), (usize, usize))) -> u64 {
    let cost = calculate_route_costs(input);
    let mut shortest_path = u64::MAX;
    for (x, y) in cost.keys() {
        if input.0[*x][*y] == b'a' {
            let path_length = cost.get(&(*x, *y)).unwrap();
            if path_length < &shortest_path {
                shortest_path = *path_length;
            }
        }
    }
    shortest_path
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
        assert_eq!(31, part_1(&get_heightmap(INPUT)));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(29, part_2(&get_heightmap(INPUT)));
    }
}
