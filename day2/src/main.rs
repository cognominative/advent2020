use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::string::ParseError;

struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

impl FromStr for PasswordEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&['-', ':', ' '][..]);

        // This is... so tragic
        let min = parts.next().unwrap().parse().unwrap();
        let max = parts.next().unwrap().parse().unwrap();
        let letter = parts.next().unwrap().parse().unwrap();
        parts.next(); // discard extra space
        let password = parts.next().unwrap().parse().unwrap();

        Ok(PasswordEntry {
            min,
            max,
            letter,
            password
        })
    }
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        (count >= self.min) && (count <= self.max)
    }

    fn is_valid_positional(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        (chars[self.min - 1] == self.letter) ^ (chars[self.max - 1] == self.letter)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();

    let input: Vec<PasswordEntry> = lines.map(|x| x.unwrap().parse().unwrap()).collect();

    let day1 = input.iter()
                    .filter(|x| x.is_valid())
                    .count();

    println!("Valid passwords in part 1: {}", day1);

    let day2 = input.iter()
                    .filter(|x| x.is_valid_positional())
                    .count();

    println!("Valid passwords in part 2: {}", day2);

}
