use crate::input::Input;

fn main() {
    const INPUT: &str = include_str!("../input");
    

    let mut input = Input::from_str(INPUT);
    let actual = input.problem1(10);
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem1(40);
    println!("Solution 2 : {}", actual2);
}


mod input {
    use std::collections::HashMap;
    pub struct Input<'a>{
        value: &'a str,
        insertions: Vec<(&'a str, &'a str)>
        
    }

    impl<'a> Input<'a> {
        pub fn problem1(& mut self, runtime: usize) -> u64 {
            let mut initial: HashMap<String, u64> = HashMap::new();
            for c in 0..self.value.len() -1 {
                let count = initial.entry(self.value[c..c+2].to_string()).or_default();
                *count += 1;
            }
            
            let result = (0..runtime).fold(initial, |counts, _| {
                let mut next_counts = HashMap::new();
                for (pair, count) in &counts {
                    let insertion = self.insertions.iter().find(|c| c.0 == pair);
                    if let Some(ins) = insertion {
                        let pair_1 = pair[0..1].to_owned() + ins.1;
                        let pair_2 = ins.1.to_string() + &pair[1..2];
                        *next_counts.entry(pair_1).or_default() += count;
                        *next_counts.entry(pair_2).or_default() += count;
                    }
                }
                next_counts        
            });

            let mut count: HashMap<String, u64> = HashMap::new();
            for (pair, c) in result {
                *count.entry(pair[0..1].to_string()).or_default() += c;
            }
            *count.entry(self.value[self.value.len() -1..self.value.len()].to_string()).or_default() += 1;
            let top_char = count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
            let least_char = count.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
            top_char.1 - least_char.1
        }


        pub fn from_str(input: &'a str) -> Self {
            
            let lines = input.lines().collect::<Vec<&'a str>>();
            let insertions = lines.iter().skip(2).map(|line| {
                let (pair, insertion) = line.split_once("->").unwrap();
                (pair, insertion)
            }).fold(Vec::new(), |mut insertions: Vec<(&'a str, &'a str)>, (mut pair, mut insertion)| {
                pair = pair.trim();
                insertion = insertion.trim();
                insertions.push((pair, insertion));
                insertions
            });
            

            Self {
                value: lines[0],
                insertions
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 1588;
        let actual = input.problem1(10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected:u64 = 2188189693529;
        let actual = input.problem1(40);
        assert_eq!(expected, actual);
    }
}
