use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("./src/04/input.txt").unwrap();

    let mut contained = 0;
    let mut overlap = 0;

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (elf1, elf2) = line.split_once(',').unwrap();

        let elf1 = elf1.split_once('-').unwrap();
        let mut elf1: (u32, u32) = (elf1.0.parse().unwrap(), elf1.1.parse().unwrap());

        let elf2 = elf2.split_once('-').unwrap();
        let mut elf2: (u32, u32) = (elf2.0.parse().unwrap(), elf2.1.parse().unwrap());

        if elf1.0 > elf2.0 {
            std::mem::swap(&mut elf1, &mut elf2);
        }

        if (elf1.1 >= elf2.1) || (elf1.0 == elf2.0) {
            contained += 1;
        }

        if elf1.1 >= elf2.0 {
            overlap += 1;
        }
    }

    println!("1. {contained}");
    println!("2. {overlap}")
}