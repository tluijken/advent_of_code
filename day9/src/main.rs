use crate::input::Input;

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    const INPUT: &str = include_str!("../input");
    

    let mut input = Input::from_str(INPUT);
    let actual = input.problem1();
    println!("Solution 1 : {}", actual);
    
    let actual2 = input.problem2();
    println!("Solution 2 : {}", actual2);
}


mod input {
    use crate::NEIGHBORS;

    pub struct Input{
        heightmap: Vec<Vec<u32>>,
    }

    impl Input {
        pub fn problem1(& mut self) -> u32 {
            let mut result = 0;
            for (y, line) in self.heightmap.iter().enumerate() {
                for (x, cell) in line.iter().enumerate() {
                    if NEIGHBORS.iter().all(|&(nx, ny)| {
                        self.heightmap.get((y as isize + ny) as usize)
                            .and_then(|l| l.get((x as isize + nx) as usize))
                            .map(|n| cell < n)
                            .unwrap_or(true)
                    }) { result += cell + 1; }
                }
            }
            result
        }

        pub fn problem2(&mut self) -> usize {
            let mut basins: Vec<usize> = vec![];
            for y in 0..self.heightmap.len() {
                for x in 0..self.heightmap.len() {
                    if self.heightmap[y][x] < 9 {
                        basins.push(get_basin(&mut self.heightmap, x, y));
                    }
                }
            }

            basins.sort_unstable();
            basins.iter().rev().take(3).product()
        }


        pub fn from_str(input: &str) -> Self {

            let heightmap: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
            Self {
                heightmap
            }
        }
    }

    /// Scans all NEIGHBORS for x and y resursively and marks handled items for this basin with the
    /// high-number to it doesn't get used again
    /// Returns the count of all cells within the basin
    fn get_basin(heightmap: &mut Vec<Vec<u32>>, x: usize, y: usize) -> usize {
        heightmap[y][x]= 9; // mark as used
        NEIGHBORS.iter().map(|(nx, ny)| ((x as isize + nx) as usize, (y as isize + ny) as usize))
            .fold(1, |acc, (x, y)| {
                match heightmap.get(y).and_then(|l| l.get(x)).map(|&n| n < 9){
                    Some(true) => acc + get_basin(heightmap, x, y),
                    _ => acc
                }
            })
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
        let expected = 1134;
        let actual = input.problem2();
        assert_eq!(expected, actual);
    }
}
