#[derive(Debug, Clone, Copy)]
enum Operation {
    MonkeyDance,
    Add(u64),
    Multiply(u64),
}

#[aoc_generator(day11)]
fn parse_monkeys(input: &str) -> Vec<(Vec<u64>, Operation, u64, usize, usize, usize)> {
    input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|lines| {
            (
                lines[1][16..]
                    .split(", ")
                    .map(|i| i.parse().unwrap_or(0))
                    .collect(),
                lines[2][23..]
                    .parse()
                    .map(|n| match lines[2].contains("+") {
                        true => Operation::Add(n),
                        false => Operation::Multiply(n),
                    })
                    .unwrap_or(Operation::MonkeyDance),
                lines[3][19..].parse().unwrap_or(0),
                lines[4][25..].parse().unwrap_or(0),
                lines[5][26..].parse().unwrap_or(0),
                0,
            )
        })
        .collect()
}

fn play(
    monkeys: &mut Vec<(Vec<u64>, Operation, u64, usize, usize, usize)>,
    rounds: usize,
    f: impl Fn(u64) -> u64,
) -> usize {
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|m| {
            let (items, operation, test, m1, m2, _) = monkeys[m].clone();
            items.iter().for_each(|item| {
                let new = match operation {
                    Operation::Add(n) => f(item + n),
                    Operation::Multiply(n) => f(item * n),
                    Operation::MonkeyDance => f(item * item),
                };
                match new % test == 0 {
                    true => monkeys[m1].0.push(new),
                    false => monkeys[m2].0.push(new),
                }
            });
            monkeys[m].5 += monkeys[m].0.len();
            monkeys[m].0.clear();
        });
    });
    monkeys.sort_by(|a, b| b.5.cmp(&a.5));
    monkeys.iter().map(|m| m.5).take(2).product()
}

#[aoc(day11, part1)]
fn part_1(monkeys: &Vec<(Vec<u64>, Operation, u64, usize, usize, usize)>) -> usize {
    play(&mut monkeys.clone(), 20, |x| x / 3)
}

#[aoc(day11, part2)]
fn part_2(monkeys: &Vec<(Vec<u64>, Operation, u64, usize, usize, usize)>) -> usize {
    play(&mut monkeys.clone(), 10000, |x| {
        x % monkeys.iter().map(|m| m.2).product::<u64>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn test_part_1() {
        assert_eq!(10605, part_1(&parse_monkeys(INPUT)));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(2713310158, part_2(&parse_monkeys(INPUT)));
    }
}
