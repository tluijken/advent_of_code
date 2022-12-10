use std::collections::HashMap;

#[aoc_generator(day7)]
fn get_folder_sizes(input: &str) -> HashMap<String, u32> {
    let mut current = String::from("");
    let folders = input.lines().skip(1).fold(HashMap::new(), |mut str, cmd| {
        if cmd.starts_with("$ cd") {
            match (cmd.ends_with(".."), current.rfind('/')) {
                (true, Some(pos)) => {
                    current = current[..pos].to_string();
                }
                _ => current = format!("{}/{}", current, &cmd[5..]),
            }
        }
        let entry = str.entry(current.clone()).or_insert(0);
        if cmd.as_bytes()[0].is_ascii_digit() {
            *entry += cmd.split_once(' ').unwrap().0.parse::<u32>().unwrap();
        }
        str
    });
    //sum subfolders
    folders.iter().fold(HashMap::new(), |mut hm, f| {
        let sum = folders
            .keys()
            .filter(|k| k.starts_with(f.0))
            .map(|k| folders.get(k).unwrap_or(&0))
            .sum();
        hm.entry(f.0.to_owned()).or_insert(sum);
        hm
    })
}

#[aoc(day7, part1)]
fn part_1(folders: &HashMap<String, u32>) -> u32 {
    folders
        .iter()
        .filter(|kv| kv.1 <= &100000)
        .map(|kv| kv.1)
        .sum()
}

#[aoc(day7, part2)]
fn part_2(folders: &HashMap<String, u32>) -> u32 {
    let free_space = 70000000 - folders.get("").unwrap();
    let mut cleanup_folders = folders
        .iter()
        .filter(|kv| free_space + kv.1 >= 30000000)
        .map(|kv| *kv.1)
        .collect::<Vec<u32>>();
    cleanup_folders.sort();
    cleanup_folders.first().unwrap().to_owned()
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
        assert_eq!(95437, part_1(&get_folder_sizes(INPUT)));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(24933642, part_2(&get_folder_sizes(INPUT)));
    }
}
