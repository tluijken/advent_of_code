use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let mut prev: Option<i32> = None;
    let mut nr_of_increases = 0; 
    for(_index, line) in BufReader::new(File::open("input").unwrap()).lines().enumerate(){
        let curr: i32 = line.unwrap().parse::<i32>().unwrap(); // we assume all is ok for now.
        if prev.is_some() && prev <= Some(curr) {
            nr_of_increases += 1;
        }
        
        prev = Some(curr);
    }

    println!("Total number of increases is: {}", nr_of_increases);
}
