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
    pub struct Input{
        heightmap: Vec<Vec<u32>>,
    }

    impl Input {
        pub fn problem1(& mut self) -> u32 {
            let length = self.heightmap[0].len();
            let mut result = 0;
            for y in 0..self.heightmap.len() {
                for x in 0..length {
                    let left_greater = x == 0 || self.heightmap[y][x - 1] > self.heightmap[y][x];
                    let right_greater = x == length - 1 || self.heightmap[y][x + 1] > self.heightmap[y][x];
                    let top_greater = y == 0 || self.heightmap[y -1][x] > self.heightmap[y][x];
                    let bottom_greater = y == (self.heightmap.len() - 1) || self.heightmap[y + 1][x] > self.heightmap[y][x];
                    if left_greater && right_greater && top_greater && bottom_greater {
                        result += 1 + self.heightmap[y][x];
                    }
                }
            }
            result
        }


        pub fn problem2(& mut self) -> i32 {
            0
        }

        pub fn from_str(input: &str) -> Self {

            let heightmap: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
            Self {
                heightmap
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 15;
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
