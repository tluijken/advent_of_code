fn part_1(input: &str) -> String {
    let mut stacks = fill_stacks(input, get_number_of_stacks(input));
    for movement in get_movements(input) {
        let mut items = pick_crates_from_stack(&mut stacks, movement[1], movement[0]);
        stacks[movement[2] as usize - 1].append(&mut items);
    }
    get_cargo_top_crates(&stacks)
}

fn part_2(input: &str) -> String {
    let mut stacks = fill_stacks(input, get_number_of_stacks(input));
    for movement in get_movements(input) {
        let mut items = pick_crates_from_stack(&mut stacks, movement[1], movement[0]);
        items.reverse();
        stacks[movement[2] as usize - 1].append(&mut items);
    }
    get_cargo_top_crates(&stacks)
}

fn get_cargo_top_crates(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().fold(String::new(), |mut result, stack| {
        result.push(*stack.last().unwrap());
        result
    })
}

fn get_movements(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| l.contains("move"))
        .map(|l| {
            l.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(" ")
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .into_iter()
        .collect::<Vec<Vec<u8>>>()
}

fn get_number_of_stacks(input: &str) -> usize {
    (input
        .lines()
        .filter(|l| l.contains("]"))
        .last()
        .unwrap()
        .len()
        + 1)
        / 4
}
fn fill_stacks(input: &str, number_of_stacks: usize) -> Vec<Vec<char>> {
    input
        .lines()
        .rev()
        .filter(|l| l.contains("]"))
        .fold(vec![], |mut result, line| {
            for i in 0..number_of_stacks {
                if result.len() <= i {
                    result.push(vec![]);
                }
                if let Some(c) = line[1 + (4 * i)..].chars().next() {
                    if c != ' ' {
                        result[i].push(c);
                    }
                }
            }
            result
        })
}

fn pick_crates_from_stack(
    stacks: &mut Vec<Vec<char>>,
    stack_number: u8,
    crate_count: u8,
) -> Vec<char> {
    let mut items = vec![];
    for _ in 0..crate_count {
        items.push(stacks[stack_number as usize - 1].pop().unwrap());
    }
    items
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn test_part_1() {
        assert_eq!("CMZ", part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("MCD", part_2(INPUT));
    }
}
