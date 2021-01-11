use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;


fn pair_product(values: &HashSet<i64>, target: i64) -> Option<i64> {
    for value in values {
        if values.contains(&(target - value)) {
            return Some(value * (target - value));
        }
    }

    None
}

fn triple_product(values: &HashSet<i64>, target: i64) -> Option<i64> {
    for first in values {
        if let Some(product) = pair_product(&values, target - first) {
            return Some(product * first);
        }
    }

    None
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let values : HashSet<i64> = lines.map(|x| x.unwrap().parse().unwrap()).collect();

    println!("{}", pair_product(&values, 2020).expect("No such pair"));
    println!("{}", triple_product(&values, 2020).expect("No such triple"));
}
