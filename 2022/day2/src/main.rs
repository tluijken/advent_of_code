use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
enum PlayerMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Ord for PlayerMove {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other)  {
            (PlayerMove::Rock, PlayerMove::Paper) => Ordering::Less,
            (PlayerMove::Paper, PlayerMove::Scissors) => Ordering::Less,
            (PlayerMove::Scissors, PlayerMove::Rock) => Ordering::Less,
            (PlayerMove::Paper, PlayerMove::Rock) => Ordering::Greater,
            (PlayerMove::Scissors, PlayerMove::Paper) => Ordering::Greater,
            (PlayerMove::Rock, PlayerMove::Scissors) => Ordering::Greater,
            _ => Ordering::Equal
        }
    }
}

impl FromStr for PlayerMove {
    type Err = ();
    fn from_str(input: &str) -> Result<PlayerMove, Self::Err> {
        match input {
            "X"  => Ok(PlayerMove::Rock),
            "Y"  => Ok(PlayerMove::Paper),
            "Z"  => Ok(PlayerMove::Scissors),
            "A"  => Ok(PlayerMove::Rock),
            "B"  => Ok(PlayerMove::Paper),
            "C"  => Ok(PlayerMove::Scissors),
            _      => Err(()),
        }
    }
}

fn part_1(input: &str) -> u32 {
    input.lines()
        .map(|l|l.split(" ").collect::<Vec<&str>>())
        .map(|s|  (PlayerMove::from_str(s[0]).unwrap(), PlayerMove::from_str(s[1]).unwrap()))
        .fold(0, |acc, i|
            match i.1.cmp(&i.0) {
                Ordering::Less => acc + i.1 as u32,
                Ordering::Equal => acc + 3 + i.1 as u32,
                Ordering::Greater => acc + 6 + i.1 as u32
        })
}

fn part_2(input: &str) -> u32 {
    input.lines()
        .map(|l|l.split(" ").collect::<Vec<&str>>())
        .map(|s|  (PlayerMove::from_str(s[0]).unwrap(), PlayerMove::from_str(s[1]).unwrap()))
        .fold(0, |acc, i|
            match i.1 {
                // Lose
                PlayerMove::Rock => match i.0 {
                    PlayerMove::Rock => acc + PlayerMove::Scissors as u32,
                    PlayerMove::Paper => acc + PlayerMove::Rock as u32,
                    PlayerMove::Scissors => acc + PlayerMove::Paper as u32
                },
                // Draw
                PlayerMove::Paper => acc + 3 + i.0 as u32,
                // Win
                PlayerMove::Scissors =>  match i.0 {
                    PlayerMove::Rock => acc + 6 + PlayerMove::Paper as u32,
                    PlayerMove::Paper => acc + 6 + PlayerMove::Scissors as u32,
                    PlayerMove::Scissors => acc + 6 + PlayerMove::Rock as u32
                }
            })

}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(INPUT));
    }
}
