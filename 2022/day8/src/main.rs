fn part_1(input: &str) -> usize {
    let mut treemap = read_tree_map(input);
    check_visible_trees(&mut treemap);
    let seen_trees = treemap
        .iter()
        .fold(0, |total, r| total + r.iter().filter(|c| c.1).count());
    seen_trees
}

fn read_tree_map(input: &str) -> Vec<Vec<(u32, bool)>> {
    input.lines().fold(vec![], |mut acc, l| {
        acc.push(
            l.chars()
                .map(|c| (c.to_digit(10).unwrap(), false))
                .collect::<Vec<(u32, bool)>>(),
        );
        acc
    })
}

fn check_visible_trees(treemap: &mut Vec<Vec<(u32, bool)>>) {
    treemap.iter_mut().for_each(|r| {
        r.iter_mut().fold(None, |mut max, c| {
            if max.is_none() || max.unwrap_or(0) < c.0 {
                max = Some(c.0);
                c.1 = true;
            }
            max
        });
        r.iter_mut().rev().fold(None, |mut max, c| {
            if max.is_none() || max.unwrap_or(0) < c.0 {
                max = Some(c.0);
                c.1 = true;
            }
            max
        });
    });
    for i in 0..treemap.first().unwrap().len() {
        treemap.iter_mut().fold(None, |mut max, r| {
            if max.is_none() || max.unwrap_or(0) < r[i].0 {
                max = Some(r[i].0);
                r[i].1 = true;
            }
            max
        });
        treemap.iter_mut().rev().fold(None, |mut max, r| {
            if max.is_none() || max.unwrap_or(0) < r[i].0 {
                max = Some(r[i].0);
                r[i].1 = true;
            }
            max
        });
    }
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
    const INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn test_part_1() {
        assert_eq!(21, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(INPUT));
    }
}
