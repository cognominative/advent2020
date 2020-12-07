use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn seat_id(ticket: &str) -> usize {
    usize::from_str_radix(ticket.chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => c // Don't like this
        }).collect::<String>()
        .as_str(), 2).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    let seat_ids: Vec<usize> = lines.map(|x| seat_id(x.as_str())).collect();
    let &highest = seat_ids.iter().max().unwrap();
    let &lowest = seat_ids.iter().min().unwrap();

    println!("Part 1: {}", highest);

    let missing = (lowest..highest)
        .filter(|x| !seat_ids.contains(x))
        .collect::<Vec<usize>>()[0];

    println!("Part 2: {}", missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}