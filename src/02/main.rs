use std::fs::File;
use std::io::BufRead;

fn main() {
    const SCORE_ROCK: i32 = 1;
    const SCORE_PAPER: i32 = 2;
    const SCORE_SCISSORS: i32 = 3;

    const SCORE_LOSS: i32 = 0;
    const SCORE_DRAW: i32 = 3;
    const SCORE_WIN: i32 = 6;

    let mut score_first = 0;
    let mut score_second = 0;
    let file = File::open("./src/02/input.txt").unwrap();

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let opponent = line.chars().next().unwrap();
        let me = line.chars().nth_back(0).unwrap();

        match me {
            'X' => {
                score_first += SCORE_ROCK + match opponent {
                    'A' => { SCORE_DRAW } // rock
                    'B' => { SCORE_LOSS } // paper
                    'C' => { SCORE_WIN }  // scissors
                    _ => unreachable!()
                };
                score_second += SCORE_LOSS + match opponent {
                    'A' => { SCORE_SCISSORS } // rock
                    'B' => { SCORE_ROCK }     // paper
                    'C' => { SCORE_PAPER }    // scissors
                    _ => unreachable!()
                };
            }
            'Y' => {
                score_first += SCORE_PAPER + match opponent {
                    'A' => { SCORE_WIN }  // rock
                    'B' => { SCORE_DRAW } // paper
                    'C' => { SCORE_LOSS } // scissors
                    _ => unreachable!()
                };
                score_second += SCORE_DRAW + match opponent {
                    'A' => { SCORE_ROCK }     // rock
                    'B' => { SCORE_PAPER }    // paper
                    'C' => { SCORE_SCISSORS } // scissors
                    _ => unreachable!()
                };
            }
            'Z' => {
                score_first += SCORE_SCISSORS + match opponent {
                    'A' => { SCORE_LOSS } // rock
                    'B' => { SCORE_WIN }  // paper
                    'C' => { SCORE_DRAW } // scissors
                    _ => unreachable!()
                };
                score_second += SCORE_WIN + match opponent {
                    'A' => { SCORE_PAPER }    // rock
                    'B' => { SCORE_SCISSORS } // paper
                    'C' => { SCORE_ROCK }     // scissors
                    _ => unreachable!()
                };
            }
            _ => unreachable!()
        }
    }

    println!("1. {score_first}");
    println!("2. {score_second}");
}