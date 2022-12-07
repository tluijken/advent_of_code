use std::collections::HashMap;

fn part_1(input: &str) -> u32 {
    get_folder_sizes(input)
        .iter()
        .filter(|kv| kv.1 <= &100000)
        .map(|kv| kv.1)
        .sum()
}

fn get_folder_sizes(input: &str) -> HashMap<String, u32> {
    let mut current = String::from("/");
    let folders = input.lines().skip(1).fold(HashMap::new(), |mut str, cmd| {
        if cmd.starts_with("$ cd") {
            if cmd.ends_with("..") {
                if let Some(pos) = current.rfind('/') {
                    current = current[..pos].to_string();
                }
            } else {
                current = format!(
                    "{}{}{}",
                    current,
                    match current.eq("/") {
                        true => "",
                        false => "/",
                    },
                    &cmd[5..]
                );
            }
        }
        if cmd.as_bytes()[0].is_ascii_digit() {
            let spl: (&str, &str) = cmd.split_once(' ').unwrap();
            *str.entry(current.clone()).or_insert(0) += spl.0.parse::<u32>().unwrap();
        }
        if cmd.starts_with("dir") {
            let _ = *str.entry(current.clone()).or_insert(0);
        }
        str
    });

    folders.iter().fold(HashMap::new(), |mut hm, f| {
        let similar_folders = folders
            .keys()
            .filter(|k| k.starts_with(f.0))
            .collect::<Vec<&String>>();
        let sum = similar_folders
            .iter()
            .map(|k| folders.get(*k).unwrap())
            .sum();
        hm.entry(f.0.clone()).or_insert(sum);
        hm
    })
}

fn part_2(input: &str) -> u32 {
    let folders = get_folder_sizes(input);
    0
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_part_1() {
        assert_eq!(95437, part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(INPUT));
    }
}
