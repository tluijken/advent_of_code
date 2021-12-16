use crate::input::Input;

fn main() {
    const INPUT: &str = include_str!("../input");
    

    let mut input = Input::from_str(INPUT);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}


mod input {

    pub struct Input<'a>{
        value: &'a str,
        insertions: Vec<(&'a str, &'a str)>
        
    }

    impl<'a> Input<'a> {
        pub fn problem1(& mut self) -> usize {
            let mut value = self.value.to_string();
            for _ in 0..10 {
                let mut new_line = "".to_string();

                for i in 0..value.len() {
                    if i == value.len() - 1 {
                        new_line += &value[i..i+1]
                    }
                    else {
                        let substr = &value[i..i+2];
                        match self.insertions.iter().find(|i| i.0 == substr){
                            Some(insertion) => {
                                let replacement = format!("{}{}", &insertion.0[0..1], insertion.1);
                                new_line += &replacement;
                            }
                            _ => new_line += &value[i..i+1]
                        }
                    }
                }
                value = new_line;
            }


            let mut v: Vec<char> = value.chars().collect();
            v.sort_unstable();
            v.dedup();
            
            let mut occurences: Vec<usize> = Vec::new();
            
            for c in v {
                occurences.push(value.matches(c).count());
            }

            occurences.sort_unstable();
            occurences.last().unwrap() - occurences.first().unwrap()
        }


        pub fn problem2(& mut self) -> i32 {
            0
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
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 0;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
