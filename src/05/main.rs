use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("./src/05/input.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines().map(Result::unwrap);

    let mut stacks1: Vec<Vec<char>> = vec![];

    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let num_stacks = (line.len() + 1) / 4;
        while stacks1.len() < num_stacks {
            stacks1.push(vec![]);
        }

        let mut chars = line.chars();

        for stack in stacks1.iter_mut() {
            let value = chars.nth(1).unwrap();
            if value.is_ascii_uppercase() {
                stack.push(value);
            }
            let _ = chars.nth(1);
        }
    }

    for stack in stacks1.iter_mut() {
        stack.reverse();
    }

    let mut stacks2 = stacks1.clone();

    for line in lines {
        let mut tokens = line.split(' ');
        let num_moves: usize = tokens.nth(1).unwrap().parse().unwrap();
        let from: usize = tokens.nth(1).unwrap().parse().unwrap();
        let to: usize = tokens.nth(1).unwrap().parse().unwrap();

        for _ in 0..num_moves {
            let el = stacks1[from-1].pop().unwrap();
            stacks1[to-1].push(el);
        }

        let new_length = stacks2[from-1].len() - num_moves;
        let els = stacks2[from-1].as_slice()[new_length..].to_vec();
        stacks2[from-1].truncate(new_length);
        stacks2[to-1].extend(els);
    }

    print!("1. ");
    for stack in stacks1.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();

    print!("2. ");
    for stack in stacks2.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!();
}