use crate::input::Input;

fn main() {
    const INPUT: &str = "hl-WP
vl-fo
vl-WW
WP-start
vl-QW
fo-wy
WW-dz
dz-hl
fo-end
VH-fo
ps-vl
FN-dz
WP-ps
ps-start
WW-hl
end-QW
start-vl
WP-fo
end-FN
hl-QW
WP-dz
QW-fo
QW-dz
ps-dz";
    
    let mut input = Input::from_str(INPUT);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}


mod input {
    use std::collections::{HashMap, VecDeque};
    pub struct Input<'a> {
        graph: HashMap<&'a str, Vec<&'a str>>,
    }

    impl<'a> Input<'a> {
        pub fn problem1(& mut self) -> usize {
            self.get_routes(false)
        }


        pub fn problem2(& mut self) -> usize {
            self.get_routes(true)
        }

        fn get_routes(&mut self, duplicate_allowed: bool) -> usize {
            // start at the end, and continue upward
            let mut q = VecDeque::from([(vec!["end"], None)]);
            let mut result = 0;
            while let Some((path, dup)) = q.pop_front() {
                for &cave in &self.graph[path.last().unwrap()] {
                    match cave {
                        "end" => continue,
                        "start" => result += 1,
                        cave => {
                            let mut dup = dup;
                            if cave.chars().all(char::is_lowercase) && path.contains(&cave) {
                                if !duplicate_allowed || dup.is_some() {
                                    continue;
                                }
                                dup = Some(cave);
                            };
                            let next = path.iter().cloned().chain([cave]).collect();
                            q.push_back((next, dup));
                        }
                    }
                }
            }

            result
        }

        pub fn from_str(input: &'a str) -> Self {
            let graph = input.lines().flat_map(|line| {
                let (from, to) = line.split_once('-').unwrap();
                [(from, to), (to, from)]
            }).fold(HashMap::new(), |mut graph: HashMap<&'a str, Vec<&'a str>>, (from, to)| {
                graph.entry(from).or_default().push(to);
                graph
            });
            Self {
                graph
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 226;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 3509;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
