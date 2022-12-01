use crate::input::Input;

fn main() {
    const INPUT: &str = "3,1,5,4,4,4,5,3,4,4,1,4,2,3,1,3,3,2,3,2,5,1,1,4,4,3,2,4,2,4,1,5,3,3,2,2,2,5,5,1,3,4,5,1,5,5,1,1,1,4,3,2,3,3,3,4,4,4,5,5,1,3,3,5,4,5,5,5,1,1,2,4,3,4,5,4,5,2,2,3,5,2,1,2,4,3,5,1,3,1,4,4,1,3,2,3,2,4,5,2,4,1,4,3,1,3,1,5,1,3,5,4,3,1,5,3,3,5,4,2,3,4,1,2,1,1,4,4,4,3,1,1,1,1,1,4,2,5,1,1,2,1,5,3,4,1,5,4,1,3,3,1,4,4,5,3,1,1,3,3,3,1,1,5,4,2,5,1,1,5,5,1,4,2,2,5,3,1,1,3,3,5,3,3,2,4,3,2,5,2,5,4,5,4,3,2,4,3,5,1,2,2,4,3,1,5,5,1,3,1,3,2,2,4,5,4,2,3,2,3,4,1,3,4,2,5,4,4,2,2,1,4,1,5,1,5,4,3,3,3,3,3,5,2,1,5,5,3,5,2,1,1,4,2,2,5,1,4,3,3,4,4,2,3,2,1,3,1,5,2,1,5,1,3,1,4,2,4,5,1,4,5,5,3,5,1,5,4,1,3,4,1,1,4,5,5,2,1,3,3";
    let mut input = Input::from_str(INPUT);
    let actual = input.problem1(256);
    println!("Solution 1 : {}", actual);
}


mod input {
    pub struct Input{
        pub values: [i64; 9],
    }

    impl Input {
        pub fn problem1(& mut self, number_of_days: u32) -> i64{
            
            for _ in 0..number_of_days {
                let d = self.values[0];
                for i in 1..9{
                    self.values[i - 1] = self.values[i];
                    self.values[i] = 0;
                }
                self.values[8] = d;
                self.values[6] += d;
            }


            self.values.iter().fold(0, |a, b| a + b)
        }

        pub fn from_str(input: &str) -> Self {
            Self {
                values :input.split(",").map(|i| i.parse::<i32>().unwrap())
                    .fold([0;9], |mut fishes, days_untill_making_babies| {
                        fishes[days_untill_making_babies as usize] += 1;
                        fishes
                    }),
            }
        }
    }
}
#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 26;
        let actual = input.problem1(18);
        assert_eq!(expected, actual);
    }
}
