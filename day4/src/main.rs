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

    let mut passport = HashSet::<String>::new();
    let mut num_valid = 0;

    let validate_fields = |passport: HashSet<String>| {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&key| passport.contains(key))
    };

    for line in lines {
        if line.is_empty() {
            num_valid += validate_fields(passport) as u32;
            passport = HashSet::<String>::new();
        }
        else {
            line.split(' ').for_each(|x| {
                let field = x.parse::<PassportField>().unwrap();
                passport.insert(field.key);
            });
        }
    }

    // Last one gets missed because the final newline
    // has already been stripped
    num_valid += validate_fields(passport) as u32;

    println!("Part 1: {}", num_valid)
}
