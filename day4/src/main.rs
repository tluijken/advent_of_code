#![feature(entry_insert)]

use std::collections::HashMap;
fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<_>>();

    let numbers_drawn = lines.first()
                             .unwrap()
                             .split(',')
                             .map(|s| s.parse::<u32>().unwrap())
                             .collect::<Vec<_>>();

    let count = lines.len();

    // Construct the boards
    let mut boards: Vec<Board> = Vec::new();
    for x in (2..count).step_by(6) {
        let mut numbers: HashMap<u32, Vec<(u32, bool)>> = HashMap::new();
        for y in 0..5{
            let values = lines[x+y].split(' ').filter(|n| !n.is_empty()).map(|l| (l.parse::<u32>().unwrap(), false)).collect();
            numbers.entry(y as u32).insert(values);
        }
        boards.push(Board{
            numbers,
            has_won: false
        });
    }


    let mut winner = false;   
    let mut last_drawn_number: &u32 = &0;
    let mut winner_score: u32 = 0;
    for number in &numbers_drawn {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.check_winner() {
                winner_score = board.sum_unmarked();
                winner = true;
                break;
            }
        }
        if winner {
            last_drawn_number = number;
            break;
        }
    }
    
    println!("Result for step 1 : {}", winner_score * last_drawn_number);
    
    winner = false;   
    winner_score = 0;
    let number_of_cards:usize = boards.len();
    let mut winning_cards:usize = 0;
    for number in &numbers_drawn {
        for board in boards.iter_mut() {
            if board.has_won {
                continue;
            }
            board.mark_number(number);
            if board.check_winner() {
                winning_cards += 1;
                if winning_cards == number_of_cards - 1 {
                    winner_score = board.sum_unmarked();
                    winner = true;
                    break;
                }
            }
        }
        if winner {
            last_drawn_number = number;
            break;
        }
    }
    
    println!("Result for step 2 : {}", winner_score * last_drawn_number);
}

struct Board {
    numbers: HashMap<u32, Vec<(u32, bool)>>,
    has_won: bool,
}


impl Board{
    fn check_winner(&mut self) -> bool {
        // Check horizontal
        for values in self.numbers.values() {
            if values.iter().all(|v| v.1) {
                self.has_won = true;
                return true;
            }
        }

        // check vertical
        for x in 0..5 {
            if self.numbers.iter().all(|(_, values)| values[x].1) {
                self.has_won = true;
                return true;
            }
        }
        false
    }

    fn mark_number(&mut self, drawn_number: &u32) {
        for (_, value) in self.numbers.iter_mut()
        {
            for mut number in value {
                if number.0 == *drawn_number {
                    number.1 = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers.iter().map(|(_, values)| values.iter().filter(|v| !v.1).map(|v| v.0).sum::<u32>()).sum::<u32>()
    }
}

