use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let mut nums = std::io::stdin().lock().lines()
        .map(|line| u32::from_str(&line.unwrap()).unwrap())
        .collect::<Vec<_>>();

    nums.sort_unstable();

    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);

    let counts = nums.iter()
        .tuple_windows::<(_, _)>()
        .fold((0, 0), |counts, tup| {
            match tup.1 - tup.0 {
                1 => (counts.0 + 1, counts.1),
                3 => (counts.0, counts.1 + 1),
                _ => counts
            }
        });

    println!("Part 1: {}", counts.0 * counts.1);
}
