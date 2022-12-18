use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn idx(c: u8) -> usize {
    (c as usize) - ('a' as usize)
}

fn first_n_unique(buffer: &[u8], num_unique: usize) -> usize {
    let mut char_count: [u32; 26] = [0; 26];
    let mut char_set = HashSet::new();

    for i in 0..buffer.len() {
        let j = idx(buffer[i]);
        char_count[j] += 1;
        char_set.insert(j);

        if i >= num_unique {
            let k = idx(buffer[i-num_unique]);
            char_count[k] -= 1;
            if char_count[k] == 0 {
                char_set.remove(&k);
            }

            if char_set.len() == num_unique {
                return i + 1;
            }
        }
    }
    0
}

fn main() {
    let mut buffer = Vec::new();
    let _ = File::open("./src/06/input.txt").unwrap().read_to_end(&mut buffer);
    println!("1. {}", first_n_unique(&buffer, 4));
    println!("2. {}", first_n_unique(&buffer, 14));
}
