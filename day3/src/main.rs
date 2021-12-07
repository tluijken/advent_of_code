#![feature(drain_filter)]

const WIDTH: usize = 12;

fn main() {
    let numbers = include_str!("../input")
        .lines() // read all lines
        .map(|l| usize::from_str_radix(l, 2).unwrap()) // parse earch line to a binary value containing only ones and zero's (radix = 2)
        .collect::<Vec<usize>>(); // collect

    let count = numbers.len(); // get the total number of rows.
    
    // Step 1 : significats per column   
    let significants = numbers.iter()
        .fold(vec![0; WIDTH], |count, bits| { // iterate over all the colunms (0..WIDTH), and use the lambda taking the result (count) and the current row (bits)
            count // count the number of records in this column that has the bit set to one
                .iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i ))
            .collect()
    });

    let gamma: u32 = significants.iter()
        .enumerate()
        .map(|(i, b)| ((b > &(count / 2)) as u32) << i) // for each significants (per column), check if the significant-count > the total count / 2
        .sum();
    let epsilon = !gamma & ((1 << WIDTH) - 1); // inverse the bit-value capping this to the width (no preceding 1's)
    println!("Result step 1 = {}", gamma * epsilon);

    // Step 2 : retrieve the value that indicated the Co2 and O2 values
    let oxy = (0..WIDTH).rev().scan(numbers.clone(), |oxy, i| { // iterate from left to right, using an own copy of the numbers iterator, Scan acts as a fold
        let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
        oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
        return oxy.first().copied()
        
    }).last().unwrap();
    let co2 = (0..WIDTH).rev().scan(numbers.clone(), |co2, i| { // iterate from left to right, using an own copy of the numbers iterator, Scan acts as a fold
        let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
        co2.drain_filter(|n| (*n & 1 << i > 0) == one);
        return co2.first().copied()
        
    }).last().unwrap();
    println!("Result step 2 = {}", oxy * co2);
}
