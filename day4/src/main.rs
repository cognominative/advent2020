#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

use regex::Regex;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{key}:{val}")]
struct PassportField {
    key: String,
    val: String
}

impl PassportField {
    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            static ref HCL_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        match self.key.as_str() {
            "byr" => {
                let year: u32 = self.val.parse().unwrap();
                (1920 <= year) & (year <= 2002)
            },
            "iyr" => {
                let year: u32 = self.val.parse().unwrap();
                (2010 <= year) & (year <= 2020)
            },
            "eyr" => {
                let year: u32 = self.val.parse().unwrap();
                (2020 <= year) & (year <= 2030)
            },
            "hgt" => {
                if let Some(caps) = HGT_RE.captures(self.val.as_str()) {
                    let value = caps.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
                    let unit = caps.get(2).map_or("", |m| m.as_str());
                    match unit {
                        "cm" => (150 <= value) & (value <= 193),
                        "in" => (59 <= value) & (value <= 76),
                        _ => false
                    }
                }
                else {
                    false
                }
            },
            "hcl" => HCL_RE.is_match(self.val.as_str()),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .contains(&self.val.as_str()),
            "pid" => PID_RE.is_match(self.val.as_str()),
            _ => true
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_byr() {
        let field = PassportField {key: "byr".to_string(),
                                   val: "2002".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "byr".to_string(),
                                   val: "2010".to_string()};
        assert!(!field.is_valid());
    }

    #[test]
    fn test_validate_hgt() {
        let field = PassportField {key: "hgt".to_string(),
                                   val: "60in".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "hgt".to_string(),
                                   val: "190cm".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "hgt".to_string(),
                                   val: "190in".to_string()};
        assert!(!field.is_valid());

        let field = PassportField {key: "hgt".to_string(),
                                   val: "60".to_string()};
        assert!(!field.is_valid());
    }

    #[test]
    fn test_validate_ecl() {
        let field = PassportField {key: "ecl".to_string(),
                                   val: "brn".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "ecl".to_string(),
                                   val: "wat".to_string()};
        assert!(!field.is_valid());
    }

    #[test]
    fn test_validate_hcl() {
        let field = PassportField {key: "hcl".to_string(),
                                   val: "#123abc".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "hcl".to_string(),
                                   val: "#123abz".to_string()};
        assert!(!field.is_valid());

        let field = PassportField {key: "hcl".to_string(),
                                   val: "123abc".to_string()};
        assert!(!field.is_valid());
    }

    #[test]
    fn test_validate_pid() {
        let field = PassportField {key: "pid".to_string(),
                                   val: "000000001".to_string()};
        assert!(field.is_valid());

        let field = PassportField {key: "pid".to_string(),
                                   val: "0123456789".to_string()};
        assert!(!field.is_valid());
    }
}