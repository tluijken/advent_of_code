use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {     
    let data: Vec<Vec<u32>> = BufReader::new(File::open("input").unwrap()).lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().chars().map(|bit| bit.to_digit(10).unwrap()).collect())
        .collect();

    let record_count: u32 = data.iter().len() as u32;
    let mut gamma: Vec<u16> = [].to_vec();
    let mut epsilon: Vec<u16> = [].to_vec();
    for index in 0..12 {
        let column_count:u32 = data.iter().map(|item| item[index]).filter(|item| item.gt(&0)).sum();
        if column_count > record_count / 2 { gamma.push(1);epsilon.push(0) } else { gamma.push(0);epsilon.push(1); }
    }

    println!("Result step 1 = {}", convert(&gamma) as u32 * convert(&epsilon) as u32);
}


fn convert(bits: &[u16]) -> u16 {
    bits.iter().fold(0, |result, &bit| {
        (result << 1) ^ bit
    })
}

