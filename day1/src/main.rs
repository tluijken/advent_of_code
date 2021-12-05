use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let depts: Vec<i32> = BufReader::new(File::open("input").unwrap()).lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().parse::<i32>())
        .filter(|number| number.is_ok())
        .map(|item| item.unwrap())
        .collect();

    part_1(&depts);
    part_2(&depts);
}

fn part_1(depts: &[i32]) {
    let mut nr_of_increases = 0; 
 
    for x in 1..depts.len() {
        if depts[x] > depts[x -1] {
            nr_of_increases += 1;
        }
    }

    println!("Result part 1 = {}", nr_of_increases);
}

fn part_2(depts: &[i32]) {
    let mut prev: Option<i32> = None;
    let mut nr_of_increases = 0;
    
    for x in 0..depts.len() - 2 {
        let curr = Some(depts[x] + depts[x+ 1] + depts[x+2]);
        if prev.is_some() && prev < curr {
            nr_of_increases += 1;
        }
        prev = curr;
    }

    println!("Result part 2 = {}", nr_of_increases);
}
