#[derive(Debug, Clone, Copy)]
enum Operation {
    Other,
    Add(u64),
    Multiply(u64),
}

#[aoc_generator(day11)]
fn parse_monkeys(input: &str) -> Vec<(Vec<u64>, Operation, u64, usize, usize)> {
    input
        .split("\n\n")
        .map(|m| {
            let lines = m.lines().skip(1).map(|l| l.trim()).collect::<Vec<&str>>();
            (
                lines[0][16..]
                    .split(", ")
                    .map(|i| i.parse().unwrap_or(0))
                    .collect(),
                lines[1][23..]
                    .parse()
                    .map(|n| match lines[1].contains("+") {
                        true => Operation::Add(n),
                        false => Operation::Multiply(n),
                    })
                    .unwrap_or(Operation::Other),
                lines[2][19..].parse().unwrap_or(0),
                lines[3][25..].parse().unwrap_or(0),
                lines[4][26..].parse().unwrap_or(0),
            )
        })
        .collect()
}

fn play(
    monkeys: &mut Vec<(Vec<u64>, Operation, u64, usize, usize)>,
    rounds: usize,
    f: impl Fn(u64) -> u64,
) -> usize {
    let mut inspections = vec![0; monkeys.len()];
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|m| {
            let (items, operation, test, throw_true, throw_false) = monkeys[m].clone();
            items.iter().for_each(|item| {
                let new = match operation {
                    Operation::Add(n) => f(item + n),
                    Operation::Multiply(n) => f(item * n),
                    Operation::Other => f(item * item),
                };
                match new % test == 0 {
                    true => monkeys[throw_true].0.push(new),
                    false => monkeys[throw_false].0.push(new),
                }
            });
            inspections[m] += monkeys[m].0.len();
            monkeys[m].0.clear();
        });
    });
    inspections.sort_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

#[aoc(day11, part1)]
fn part_1(monkeys: &Vec<(Vec<u64>, Operation, u64, usize, usize)>) -> usize {
    play(&mut monkeys.clone(), 20, |x| x / 3)
}

#[aoc(day11, part2)]
fn part_2(monkeys: &Vec<(Vec<u64>, Operation, u64, usize, usize)>) -> usize {
    let modulus = monkeys.iter().map(|m| m.2).product::<u64>();
    play(&mut monkeys.clone(), 10000, |x| x % modulus)
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
