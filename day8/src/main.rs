use crate::input::Input;

fn main() {
    let input = include_str!("../input");

    let mut input = Input::from_str(input);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}

mod input {
    
    pub struct Sample {
        pub inputs: Vec<String>,
        pub outputs: Vec<String>,
    }

    pub struct Input {       
        samples: Vec<Sample>
    }

    impl Sample{
        
        fn get_patterns(&self) -> Vec<&str> {
            // easy numbers...
            let one = self.inputs.iter().find(|n| n.len() == 2).unwrap();
            let four = self.inputs.iter().find(|n| n.len() == 4).unwrap();
            let seven = self.inputs.iter().find(|n| n.len() == 3).unwrap();
            let eight = self.inputs.iter().find(|n| n.len() == 7).unwrap();

            // 3 has 5 chars and contains both chars from one
            let three = self.inputs.iter().find(|n| n.len() == 5 && scrambles_contains(n, one)).unwrap();
            
            // 6 contains 6 digits and fits in eight, but doesn't contain all from one, unlike 9.
            let six = self.inputs.iter().find(|n| n.len() == 6 && scrambles_contains(eight, n) && !scrambles_contains(n, one)).unwrap();

            // 9 has 6 chars, all 5 of 3 + 1d
            let nine = self.inputs.iter().find(|n| n.len() == 6 && scrambles_contains(n, three)).unwrap();
            
            // 5 completey fits in number nine and had a length of 5
            let five = self.inputs.iter().find(|n| n.len() == 5 && scrambles_contains(nine, n) && !scrambles_contains(n, one)).unwrap();
            
            // 0 had 6 chars and is not 9  and contains all from one :P
            let zero = self.inputs.iter().find(|n| n.len() == 6 && n != &nine && scrambles_contains(n, one)).unwrap();
            
            
            let two = self.inputs.iter().find(|n| n.len() == 5 && n != &three && n != &five).unwrap();

            let numbers: [&str;10] = [
                zero, // 0
                one,       // 1
                two,    // 2
                three,    // 3
                four,       // 4
                five,    // 5
                six,   // 6
                seven,       // 7
                eight,       // 8
                nine    // 9
            ];

            numbers.to_vec()
        }

    }

    fn scrambles_contains(a: &str, b: &str) -> bool {
        b.chars().filter(|bc| a.chars().any(|ac| ac == bc.clone())).count() == b.len()
    }

    impl Input {
        pub fn problem1(& mut self) -> usize {
            let mut count: usize = 0;
            for sample in self.samples.iter() {
                count += sample.outputs
                    .iter()
                    .filter(|o|
                            o.len() == 2 ||  // 1
                            o.len() == 3 ||  // 7
                            o.len() == 4 ||  // 4 
                            o.len() == 7 // 8
                            ).count();
            }

            count
        }

        pub fn problem2(& mut self) -> usize {
            let mut result = 0;

            for sample in self.samples.iter() {
                let mut output:String = "".to_string();
                for o in sample.outputs.iter() {
                    if o.is_empty()
                    {
                        continue;
                    }

                    let sorted_number_values:Vec<&str> = sample.get_patterns().into_iter().collect();

                    let corresponsing_num = sorted_number_values.clone().into_iter().position(|n| 
                        scrambles_contains(n, o) && o.len() == n.len()
                    );
                    if let Some(i) = corresponsing_num { output.push_str(&i.to_string()) }
                }
                println!("{}", output);
                let parsed_output: usize = output.parse::<usize>().unwrap();
                result += parsed_output;
            }

            result
        }

        pub fn from_str(input: &str) -> Self {
            let lines: Vec<&str> = input.lines().collect();
            let samples = lines.iter()
                .map(|l| l.split("|").collect::<Vec<&str>>())
                .into_iter()
                .map(|spl|
                Sample {
                    inputs: spl[0].split(" ").map(|ll| ll.to_string()).collect(),
                    outputs: spl[1].split(" ").map(|ll| ll.to_string()).collect()
                }).collect();
            Self {
                samples
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
 
    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);

        let expected = 26;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 61229;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
