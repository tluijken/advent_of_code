use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


struct Vec2{
    x: i32,
    y: i32,
}

impl Vec2 {
    fn mutate(&mut self, x_mod: i32, y_mod: i32){
        self.x += x_mod;
        self.y += y_mod;
    }
}

fn main() {
    let movements: Vec<Vec2> = BufReader::new(File::open("input").unwrap()).lines()
        .filter(|line| line.is_ok())
        .map(|command_line| {
            let command = command_line.unwrap();
            let cmd: Vec<&str> = command.split(' ').collect();
            let val = cmd[1].parse::<i32>().unwrap();
            if cmd[0] == "forward" {
                return Vec2{x: val, y: 0};
            }
            if cmd[0] == "down" {
                return Vec2 {x: 0, y: val};
            }

            Vec2{ x: 0, y: -val}    
        })
        .collect();

    let mut aim = 0;
    let mut pos = Vec2{x: 0, y: 0};

    for cmd in movements {
        aim += cmd.y;
        pos.mutate(cmd.x, cmd.x * aim);
    }

    println!("Result = {}", pos.x * pos.y);
}
