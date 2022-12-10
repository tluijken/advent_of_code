use std::cmp::Ordering;
use std::str::FromStr;

const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
enum PlayerMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Ord for PlayerMove {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PlayerMove::Rock, PlayerMove::Paper)
            | (PlayerMove::Paper, PlayerMove::Scissors)
            | (PlayerMove::Scissors, PlayerMove::Rock) => Ordering::Less,

            (PlayerMove::Paper, PlayerMove::Rock)
            | (PlayerMove::Scissors, PlayerMove::Paper)
            | (PlayerMove::Rock, PlayerMove::Scissors) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl FromStr for PlayerMove {
    type Err = ();
    fn from_str(input: &str) -> Result<PlayerMove, Self::Err> {
        match input {
            "A" | "X" => Ok(PlayerMove::Rock),
            "B" | "Y" => Ok(PlayerMove::Paper),
            "C" | "Z" => Ok(PlayerMove::Scissors),
            _ => Err(()),
        }
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|s| {
            (
                PlayerMove::from_str(s[0]).unwrap(),
                PlayerMove::from_str(s[1]).unwrap(),
            )
        })
        .fold(0, |acc, (opponent_move, player_move)| {
            match player_move.cmp(&opponent_move) {
                Ordering::Less => acc + player_move as u32,
                Ordering::Equal => acc + DRAW_SCORE + player_move as u32,
                Ordering::Greater => acc + WIN_SCORE + player_move as u32,
            }
        })
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|s| {
            (
                PlayerMove::from_str(s[0]).unwrap(),
                PlayerMove::from_str(s[1]).unwrap(),
            )
        })
        .fold(0, |acc, (opponent_move, player_move)| match player_move {
            // Lose
            PlayerMove::Rock => match opponent_move {
                PlayerMove::Rock => acc + PlayerMove::Scissors as u32,
                PlayerMove::Paper => acc + PlayerMove::Rock as u32,
                PlayerMove::Scissors => acc + PlayerMove::Paper as u32,
            },
            // Draw
            PlayerMove::Paper => acc + DRAW_SCORE + opponent_move as u32,
            // Win
            PlayerMove::Scissors => match opponent_move {
                PlayerMove::Rock => acc + WIN_SCORE + PlayerMove::Paper as u32,
                PlayerMove::Paper => acc + WIN_SCORE + PlayerMove::Scissors as u32,
                PlayerMove::Scissors => acc + WIN_SCORE + PlayerMove::Rock as u32,
            },
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
