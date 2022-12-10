#[aoc_generator(day4)]
fn get_teams(input: &str) -> Vec<((u8, u8), (u8, u8))> {
    input
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .fold(vec![], |mut teams, pair| {
            teams.push((
                parse_section(pair.first().unwrap()),
                parse_section(pair.last().unwrap()),
            ));
            teams
        })
}
fn parse_section(pair_item: &str) -> (u8, u8) {
    let range = pair_item
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|r| r.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    (
        range.first().unwrap().to_owned(),
        range.last().unwrap().to_owned(),
    )
}

#[aoc(day4, part1)]
fn part_1(input: &[((u8, u8), (u8, u8))]) -> usize {
    input
        .iter()
        .filter(|team| {
            (team.0 .0 >= team.1 .0 && team.0 .1 <= team.1 .1)
                || (team.1 .0 >= team.0 .0 && team.1 .1 <= team.0 .1)
        })
        .count()
}
#[aoc(day4, part2)]
fn part_2(input: &[((u8, u8), (u8, u8))]) -> usize {
    input
        .iter()
        .filter(|team| {
            (team.0 .0..=team.0 .1).contains(&team.1 .0)
                || (team.0 .0..=team.0 .1).contains(&team.1 .1)
                || (team.1 .0..=team.1 .1).contains(&team.0 .0)
                || (team.1 .0..=team.1 .1).contains(&team.0 .1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(&get_teams(INPUT)));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2(&get_teams(INPUT)));
    }
}
