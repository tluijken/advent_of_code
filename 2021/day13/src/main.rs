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
        dots: Vec<Point>,
        folds: Vec<Fold>,
    }
#[derive(Debug)]
    struct Point{
        x: u32,
        y: u32,
    }

    struct Fold{
        axis: String,
        pos: u32
    }

    impl Input {
        pub fn problem1(& mut self) -> usize {
            let f = &self.folds[0];
            if f.axis == "y"{
                for mut point in &mut self.dots {
                    if point.y < f.pos{
                        continue;
                    }
                    

                    point.y -= (point.y - f.pos) * 2
                }
            }
            else {
                for mut point in &mut self.dots {
                    if point.x < f.pos {
                        continue
                    }
                    point.x -= (point.x - f.pos) * 2
                }
            }

            let mut result:Vec<&Point> = Vec::new();
            for dot in &self.dots {
                if !result.iter().any(|r| r.x == dot.x && r.y == dot.y) {
                    result.push(dot);
                }
            }
            result.iter().len()
        }


        pub fn problem2(& mut self) -> usize {
            for f in &self.folds {
                if f.axis == "y"{
                    for mut point in &mut self.dots {
                        if point.y < f.pos{
                            continue;
                        }


                        point.y -= (point.y - f.pos) * 2
                    }
                }
                else {
                    for mut point in &mut self.dots {
                        if point.x < f.pos {
                            continue
                        }
                        point.x -= (point.x - f.pos) * 2
                    }
                }
            }
            let mut result:Vec<&Point> = Vec::new();
            for dot in &self.dots {
                if !result.iter().any(|r| r.x == dot.x && r.y == dot.y) {
                    result.push(dot);
                }
            }
            
            
            for y in 0..6 {
                for x in 0..40 {
                    if result.iter().any(|r| r.x == x && r.y == y) {
                        print!("\x1b[0;92m#\x1b[0m"); //prints in green
                        //print!("#");
                    }
                    else {
                        print!(".");
                    }
                }
                println!();
            }

            0
        }

        pub fn from_str(input: &str) -> Self {

            let dots = input.lines()
                .filter(|l| !l.is_empty() && !l.starts_with("fold"))
                .map(|l| l.split_once(",").unwrap())
                .fold(Vec::new(), |mut dots: Vec<Point>, (x, y)| {
                    dots.push(Point{x: x.parse::<u32>().unwrap(), y : y.parse::<u32>().unwrap()});
                    dots
                });
            let folds = input.lines()
                .filter(|l| l.starts_with("fold"))
                .map(|l| l.split_once("=").unwrap())
                .fold(Vec::new(), |mut folds: Vec<Fold>, (axis, pos)| {
                    folds.push(Fold{axis: axis.replace("fold along ", ""), pos : pos.parse::<u32>().unwrap()});
                    folds
                });

            Self {
                dots,
                folds
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 17;
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
