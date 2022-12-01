#![feature(array_chunks)]

use crate::input::Input;

fn main() {
    let input = input::hex_to_binary("620080001611562C8802118E34");
    let mut input1 = Input::from_str(&input);
    let sum = input1.problem1();

    println!("Sum {} ", sum);
    //const INPUT: &str = "11101110000000001101010000001100100000100011000001100000"; //include_str!("../input");

    //let mut input = Input::from_str(INPUT);
    //let actual = input.problem1();
    // println!("Solution 1 : {}", actual);

    //let mut input2 = Input::from_str("00111000000000000110111101000101001010010001001000000000");
    //let actual2 = input2.problem1();
    //println!("Solution 2 : {}", actual2);
}

mod input {

    use std::str::FromStr;

    const MIN_LENGTH: usize = 6 + 1 + 4; // minimum length of packet is a literal with 4 bits

    #[derive(Debug)]
    struct Packet {
        version: isize,
        type_id: PacketType,
        length_type_id: isize,
        sub_packet_length: isize,
        sub_packets: Vec<Packet>,
        literal_value: String,
    }

    #[derive(Debug)]
    enum PacketType {
        Literal,
        Operator,
    }

    fn sum_versions(packet: &Packet) -> isize {
        let sum = match &packet.type_id {
            PacketType::Literal => 0,
            PacketType::Operator => packet.sub_packets.iter().map(sum_versions).sum(),
        };

        packet.version + sum
    }
    pub fn hex_to_binary(hex: &str) -> String {
        hex.chars()
            .map(|x| {
                let raw = u64::from_str_radix(&x.to_string(), 16).unwrap(); // parse hex digit
                let bits = format!("{:04b}", raw); // convert it to a binary string
                bits
            })
            .collect()
    }

    impl FromStr for Packet {
        type Err = std::string::ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let version = isize::from_str_radix(&s[0..3], 2).unwrap();
            let type_id = if isize::from_str_radix(&s[3..6], 2).unwrap() == 4 {
                PacketType::Literal
            } else {
                PacketType::Operator
            };

            let length_type_id = isize::from_str_radix(&s[6..7], 2).unwrap();

            let mut sub_packet_length_string = "";

            println!("{}", s);
            if length_type_id == 1 && s.len() >= 18 {
                sub_packet_length_string = &s[7..18];
            } else if s.len() >= 22 {
                sub_packet_length_string = &s[7..22];
            }

            let sub_packet_length = usize::from_str_radix(sub_packet_length_string, 2).unwrap_or(0);
            let mut sub_packets: Vec<Packet> = Vec::new();
            let mut literal_value: String = "".to_string();

            match type_id {
                PacketType::Literal => {
                    let substr = &s[6..];
                    let mut literal_string_binary_val = "".to_string();
                    for index in (0..substr.len()).step_by(5) {
                        literal_string_binary_val += &substr[index + 1..index + 5];
                        let last_package = &substr[index..index + 1] == "0";
                        if last_package {
                            break;
                        }
                    }
                    literal_value = isize::from_str_radix(&literal_string_binary_val, 2)
                        .unwrap()
                        .to_string();
                }
                PacketType::Operator => {
                    if length_type_id == 1 {
                        for i in 0..sub_packet_length {
                            let index = 18 + (i * 15);
                            let sub_packet: Packet = str::parse(&s[index..index + 15]).unwrap();
                            sub_packets.push(sub_packet);
                        }
                    } else {
                        let mut index: usize = 0;
                        while index < sub_packet_length {
                            let mut end_index: usize = index + 11;
                            if end_index + 11 > sub_packet_length {
                                // no other full subpackage can be contained...so, take the remainder
                                end_index += (sub_packet_length - index) % 11;
                            }
                            let sub_packet: Packet = str::parse(&s[index..index + 11]).unwrap();
                            sub_packets.push(sub_packet);

                            index = end_index;
                        }
                    }
                }
            }

            Ok(Self {
                version,
                type_id,
                length_type_id,
                sub_packet_length: sub_packet_length.try_into().unwrap(),
                sub_packets,
                literal_value,
            })
        }
    }

    pub struct Input {
        packets: Vec<Packet>,
    }

    impl Input {
        pub fn problem1(&mut self) -> isize {
            println!("Packets {:?}", self.packets);

            sum_versions(&self.packets[0])
        }

        pub fn problem2(&mut self) -> isize {
            0
        }

        pub fn from_str(input: &str) -> Self {
            let packets: Vec<Packet> = input.lines().map(str::parse).map(Result::unwrap).collect();
            Self { packets }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Input;
    const INPUT: &str = "00111000000000000110111101000101001010010001001000000000";

    #[test]
    fn problem1() {
        let mut input = Input::from_str(INPUT);
        let expected = 0;
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
