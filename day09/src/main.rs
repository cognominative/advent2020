use std::io::BufRead;
use std::str::FromStr;

use itertools::any;


fn main() {
    let mem_length = 25;

    let seq = std::io::stdin().lock().lines()
        .map(|line| usize::from_str(&line.unwrap()).unwrap())
        .collect::<Vec<_>>();

    // Brute force first:
    for idx in mem_length..seq.len() {
        let &target = seq.get(idx).unwrap();
        let memory = seq.get((idx - mem_length)..idx).unwrap();

        if memory.iter().map(|&x| any(memory, |&y| (y != x) & (x + y == target)))
            .collect::<Vec<_>>().iter()
            .all(|&has_pair| has_pair == false) {
                println!("Part 1: {}", target);
                break;
        }
    }
}
