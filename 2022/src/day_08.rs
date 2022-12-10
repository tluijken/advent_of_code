use Result::{Err as Break, Ok as Next};

#[aoc(day8, part1)]
fn part_1(input: &str) -> usize {
    let mut treemap = read_tree_map(input);
    check_visible_trees(&mut treemap);
    let seen_trees = treemap
        .iter()
        .fold(0, |total, r| total + r.iter().filter(|c| c.1).count());
    seen_trees
}

fn read_tree_map(input: &str) -> Vec<Vec<(u32, bool, u32)>> {
    input.lines().fold(vec![], |mut acc, l| {
        acc.push(
            l.chars()
                .map(|c| (c.to_digit(10).unwrap(), false, 0))
                .collect::<Vec<(u32, bool, u32)>>(),
        );
        acc
    })
}

fn calc_scenic_score(treemap: &mut Vec<Vec<(u32, bool, u32)>>) {
    let copy = treemap.clone();
    treemap.iter_mut().enumerate().for_each(|(i, r)| {
        r.iter_mut().enumerate().for_each(|(j, c)| {
            let mut top = 0;
            let _ = (0..i)
                .rev()
                .try_fold(false, |rust_me_daddy, p| {
                    top = top + 1;
                    match copy[p][j].0 < c.0 {
                        true => Next(rust_me_daddy),
                        false => Break(rust_me_daddy),
                    }
                })
                .unwrap_or_default();
            let mut bottom = 0;
            _ = (i..copy.len())
                .skip(1)
                .try_fold(false, |i_have_no_idea_what_i_am_doing, p| {
                    bottom = bottom + 1;
                    match copy[p][j].0 < c.0 {
                        true => Next(i_have_no_idea_what_i_am_doing),
                        false => Break(i_have_no_idea_what_i_am_doing),
                    }
                })
                .unwrap_or_default();

            let mut left = 0;
            let _ = (0..j)
                .rev()
                .try_fold(false, |whatever_gets_the_job_done, p| {
                    left = left + 1;
                    match copy[i][p].0 < c.0 {
                        true => Next(whatever_gets_the_job_done),
                        false => Break(whatever_gets_the_job_done),
                    }
                })
                .unwrap_or_default();

            let mut right = 0;
            let _ = (j..copy[0].len())
                .skip(1)
                .try_fold(false, |rust_evengalism_strike_force_incoming, p| {
                    right = right + 1;
                    match copy[i][p].0 < c.0 {
                        true => Next(rust_evengalism_strike_force_incoming),
                        false => Break(rust_evengalism_strike_force_incoming),
                    }
                })
                .unwrap_or_default();
            c.2 = top * bottom * left * right;
        });
    });
}

fn check_visible_trees(treemap: &mut Vec<Vec<(u32, bool, u32)>>) {
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

#[aoc(day8, part2)]
fn part_2(input: &str) -> u32 {
    let mut treemap = read_tree_map(input);
    calc_scenic_score(&mut treemap);

    let highest_scenic_score = treemap
        .iter()
        .map(|r| r.iter().map(|c| c.2).max().unwrap_or(0))
        .max()
        .unwrap_or(0);
    highest_scenic_score
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
        assert_eq!(8, part_2(INPUT));
    }
}
