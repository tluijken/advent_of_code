use crate::input::Input;

fn main() {
    const INPUT: &str = "5251578181
6158452313
1818578571
3844615143
6857251244
2375817613
8883514435
2321265735
2857275182
4821156644";
    

    let mut input = Input::from_str(INPUT);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}


mod input {

    const NEIGHBORS: [(isize, isize); 8] = [
        (0, -1), (1, -1), (-1, -1), //lefts
        (0, 1), (1, 1), (-1, 1), // rights
        (-1, 0), (1, 0), // bottom and top
    ];

    pub struct Input{
        heightmap: Vec<Vec<u32>>,
    }

    impl Input {
        pub fn problem1(& mut self) -> i32 {
            let mut result = 0;
            for _ in 0..100 {
                for y in 0..self.heightmap.len() {
                    for x in 0..self.heightmap[0].len() {
                        check_flash(&mut self.heightmap, x, y);
                    }
                }
                for y in 0..self.heightmap.len() {
                    for x in 0..self.heightmap[0].len() {
                        if self.heightmap[y][x] > 9 {
                            result += 1;
                            self.heightmap[y][x] = 0;
                        }
                    }
                }

            }

            println!("{:?}", self.heightmap);
            result
        }

        pub fn problem2(& mut self) -> i32 {
            for i in 0..1000000000 {
                let mut result = 0;
                for y in 0..self.heightmap.len() {
                    for x in 0..self.heightmap[0].len() {
                        check_flash(&mut self.heightmap, x, y);
                    }
                }
                for y in 0..self.heightmap.len() {
                    for x in 0..self.heightmap[0].len() {
                        if self.heightmap[y][x] > 9 {
                            result += 1;
                            self.heightmap[y][x] = 0;
                        }
                    }
                }

                if result == self.heightmap.len() * self.heightmap[0].len() {
                    return i + 1;
                }
            }
            0
        }

        pub fn from_str(input: &str) -> Self {
            let heightmap: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
            Self {
                heightmap
            }
        }
    }

    fn check_flash(heightmap: &mut Vec<Vec<u32>>, x: usize, y: usize) -> usize {
        if heightmap[y][x] > 9 {
            return 0
        }
        heightmap[y][x] += 1; // increase by one
        if heightmap[y][x] > 9 {
            // we flashed..increment neighbors
            return NEIGHBORS.iter().map(|(nx, ny)| ((x as isize + nx) as usize, (y as isize + ny) as usize))
                .fold(1, |acc, (x, y)| {
                    match heightmap.get(y).and_then(|l| l.get(x)).map(|&n| n < 10) {
                        Some(true) => acc + {
                            check_flash(heightmap, x, y)
                        },
                        _ => acc
                    }
                });
        }
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Input;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 1656;
        let actual = input.problem1();
        assert_eq!(expected, actual);
    }

    #[test]
    fn problem2() {
        let mut input = Input::from_str(INPUT);
        let expected = 195;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
