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

    let check_collisions = |dx: usize, dy: usize| (0..(height / dy))
        .filter(|&row| trees[row * dy][(row * dx) % width])
        .count();

    println!("Part 1: {}", check_collisions(3, 1));

    let prod = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |prod, pair| {
            let colls = check_collisions(pair.0, pair.1);
            println!("{}", colls);
            prod * colls
        });

    println!("Part 2: {}", prod);
}
