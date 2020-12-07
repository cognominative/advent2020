use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();

    let trees : Vec<Vec<bool>> = lines.map(|line| {
        line.unwrap().chars().map(|c| c == '#').collect()
    }).collect();

    let height = trees.len();
    let width = trees[0].len();

    let check_collisions = |dx: usize| (0..height)
        .filter(|&row| trees[row][(row * dx) % width])
        .count();

    println!("Part 1: {}", check_collisions(3));
}
