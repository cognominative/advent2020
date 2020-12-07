use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{key}:{val}")]
struct PassportField {
    key: String,
    val: String
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    let mut passports = Vec::<Vec<PassportField>>::new();
    let mut passport = Vec::<PassportField>::new();

    for line in lines {
        if line.is_empty() {
            passports.push(passport);
            passport = Vec::<PassportField>::new();
        }
        else {
            line.split(' ').for_each(|x| {
                passport.push(x.parse::<PassportField>().unwrap());
            });
        }
    }
    // Last one gets missed because the final newline
    // has already been stripped
    passports.push(passport);

    let validate_completeness = |passport: &Vec<PassportField>| {
        let keys: HashSet<&str> = passport.iter()
            .map(|x| x.key.as_str())
            .collect();
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&key| keys.contains(key))
    };

    let num_complete = passports.iter()
        .filter(|&x| validate_completeness(x))
        .count();

    println!("Part 1: {}", num_complete)
}
