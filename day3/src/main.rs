use std::env;
use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();

    let sum = lines.fold((0, 0),
        |sum, line| {
            let line = line.unwrap();

            let tree = (line.chars().nth(sum.1).unwrap() == '#') as u32;
            let col = (sum.1 + 3) % line.chars().count();

            (sum.0 + tree, col)
        }
    );

    println!("Trees encountered: {}", sum.0);
}
