use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut prev: Option<i32> = None;
    let mut nr_of_increases = 0; 
    for(_index, line) in BufReader::new(File::open("input").unwrap()).lines().enumerate(){
        let curr: i32 = line.unwrap().parse::<i32>().unwrap(); // we assume all is ok for now.
        if prev.is_some() && prev < Some(curr) {
            nr_of_increases += 1;
        }
        
        prev = Some(curr);
    }

    // -1, i.d.k....
    println!("Increases = {}", nr_of_increases - 1);
}

fn part_2(){
    let mut prev: Option<i32> = None;
    let mut nr_of_increases = 0;
    let lines: Vec<i32> = BufReader::new(File::open("input").unwrap()).lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    
    for x in 0..lines.len() - 2 {
        let curr: i32 = lines[x] + lines[x + 1] + lines[x+2];

        if prev.is_some() && prev < Some(curr) {
            nr_of_increases += 1;
        }
        
        prev = Some(curr);
    }

    println!("Increases = {}", nr_of_increases);
}
