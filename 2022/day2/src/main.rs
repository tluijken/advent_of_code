fn part_1(input: &str) -> u32 {
    todo!();
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
    const INPUT: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(0, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(INPUT));
    }
}
