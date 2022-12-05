use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("./src/01/input.txt").unwrap();

    let mut cur_elf = 0;
    let mut elves = Vec::new();

    for line in std::io::BufReader::new(file).lines() {
        let calories: Result<u32, _> = line.unwrap().trim().parse();

        if let Ok(calories) = calories {
            cur_elf += calories;
        } else {
            elves.push(cur_elf);
            cur_elf = 0;
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);
}
