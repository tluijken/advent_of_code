
const GRID_LENGTH:usize = 10;

fn main() {
    let lines= include_str!("../input-sample").lines()
        .map(Line::parse)
        .collect::<Vec<Line>>();
    
    let straight_lines = lines.iter().filter(|l| l.is_straight()).collect::<Vec<&Line>>();

    let mut array = vec![vec![0; GRID_LENGTH]; GRID_LENGTH];


    let mut nr_of_danger_positions = 0;
    for x in 0..GRID_LENGTH {
        for y in 0..GRID_LENGTH {
            for line in &straight_lines {
                if line.on_position(&x, &y){
                    array[y][x] += 1;
                }
            }
            
            if array[y][x] >= 2
            {
                nr_of_danger_positions += 1;
            }
        }
    }

    for line in array {
        println!("{:?}", line);
    }

    println!("Answer step 1 : {}", nr_of_danger_positions);
}


struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            start,
            end,
       }
    }

    fn parse(line: &str) -> Line {
        let points = line.split("->").collect::<Vec<&str>>();
        Line::new(Point::parse(points[0]), Point::parse(points[1]))
    }

    fn is_straight( &self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_45_degrees(&self) -> bool {
        let left_x = if self.start.x < self.end.x { self.start.x} else { self.end.x };
        let right_x = if self.start.x > self.end.x { self.start.x} else { self.end.x };
        let bottom_y = if self.start.y > self.end.y { self.start.y} else { self.end.y };
        let top_y = if self.start.y < self.end.y { self.start.y} else { self.end.y };

        let x_width = right_x - left_x;
        
        let result = top_y + x_width == bottom_y;
        result
    }

    fn on_position(&self, x: &usize, y: &usize) -> bool {
        let left_x = if self.start.x < self.end.x { self.start.x} else { self.end.x };
        let right_x = if self.start.x > self.end.x { self.start.x} else { self.end.x };
        let bottom_y = if self.start.y > self.end.y { self.start.y} else { self.end.y };
        let top_y = if self.start.y < self.end.y { self.start.y} else { self.end.y };
        x >= &left_x && x <= &right_x && y >= &top_y && y <= &bottom_y
    }
}


impl Point {
    fn parse(line: &str) -> Point {
        let points = line.split(',').map(|i| i.trim()).collect::<Vec<&str>>();
        Point {
            x: points[0].parse::<usize>().unwrap(), 
            y: points[1].parse::<usize>().unwrap(),
        }
    }
}
