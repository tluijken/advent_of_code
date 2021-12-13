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
    use std::collections::VecDeque;
    use std::collections::HashMap;

    pub struct Input{
        lines: Vec<String>,
        invalid_scores: HashMap<char, u32>,
        autocomplete_scores: HashMap<char, u64>
    }

    impl Input {
        pub fn problem1(& mut self) -> u32 {
            let mut result = 0;
            for l in self.lines.iter() {
                let mut stack: VecDeque<char> = VecDeque::new();
                for c in l.chars() {
                    if c == '(' || c == '[' || c == '{' || c == '<' { 
                        stack.push_back(c);
                    }
                    else if !stack.is_empty() {
                        let stack_peek = stack.get(stack.len() - 1);

                        match stack_peek {
                            Some('(') => { if c != ')' { result += self.invalid_scores[&c]; break; }},
                            Some('[') => { if c != ']' { result += self.invalid_scores[&c]; break; }},
                            Some('{') => { if c != '}' { result += self.invalid_scores[&c]; break; }},
                            Some('<') => { if c != '>' { result += self.invalid_scores[&c]; break; }},
                            _ => {}
                        }
                        stack.pop_back(); 
                    }
                }
            }
            result       
        }


        pub fn problem2(& mut self) -> u64 {
            let mut results: Vec<u64> = Vec::new();
            for l in self.lines.iter() {
                let mut stack: VecDeque<char> = VecDeque::new();
                let mut line_valid = true;
                for c in l.chars() {
                    if c == '(' || c == '[' || c == '{' || c == '<' { 
                        stack.push_back(c);
                    }
                    else if !stack.is_empty() {
                        let stack_peek = stack.get(stack.len() - 1);

                        match stack_peek {
                            // ignore this line
                            Some('(') => { if c != ')' { line_valid = false; break; }},
                            Some('[') => { if c != ']' { line_valid = false; break; }},
                            Some('{') => { if c != '}' { line_valid = false; break; }},
                            Some('<') => { if c != '>' { line_valid = false; break; }},
                            _ => {}
                        }
                        stack.pop_back(); 
                    }
                }
                if !line_valid { continue; }
                let mut total_score: u64 = 0;
                while let Some(chr) = stack.pop_back() {
                    total_score *= 5;
                    if chr == '(' {total_score += self.autocomplete_scores[&')']}
                    if chr == '{' {total_score += self.autocomplete_scores[&'}']}
                    if chr == '[' {total_score += self.autocomplete_scores[&']']}
                    if chr == '<' {total_score += self.autocomplete_scores[&'>']}
                }
                results.push(total_score);
            }

            results.sort_unstable();
            results[(results.len() / 2)]
        }

        pub fn from_str(input: &str) -> Self {
            Self {
                lines: input.lines().map(|l| l.to_string()).collect(),
                invalid_scores: [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect(),
                autocomplete_scores: [(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter().collect(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 26397;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 288957;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
