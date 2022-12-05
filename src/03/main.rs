use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn get_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        1 + (item as u32) - ('a' as u32)
    } else if item.is_ascii_uppercase() {
       27 + (item as u32) - ('A' as u32)
    } else { unreachable!(); }
}

fn main() {
    const GROUP_SIZE: usize = 3;
    let mut groups: [HashSet<char>; GROUP_SIZE] = Default::default();

    let mut single_sum = 0;
    let mut group_sum = 0;

    let file = File::open("./src/03/input.txt").unwrap();

    for (i, line) in std::io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let comp_1: HashSet<char> = chars.by_ref().take(line.len()/2).collect();
        let comp_2: HashSet<char> = chars.collect();

        single_sum += get_priority(*comp_1.intersection(&comp_2).next().unwrap());

        let group_id = i % GROUP_SIZE;
        groups[group_id] = comp_1.union(&comp_2).copied().collect();

        if group_id == (GROUP_SIZE - 1) {
            let badge = groups[0]
                .intersection(&groups[1])
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&groups[2])
                .copied()
                .next()
                .unwrap();
            group_sum += get_priority(badge);
        }
    }

    println!("1. {single_sum}");
    println!("2. {group_sum}");
}