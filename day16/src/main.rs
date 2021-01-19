use std::io::Read;
use std::str::FromStr;
use std::string::ParseError;


struct Field {
    ranges: Vec<(u32, u32)>
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s.split(": ")
            .nth(1).unwrap()
            .split(" or ")
            .map(|seg| seg.split('-')
                .map(|num| u32::from_str(num).unwrap())
                .collect::<Vec<_>>())
            .filter_map(|v| {
                match v.len() {
                    2 => Some((v[0], v[1])),
                    _ => None
                }
            }).collect::<Vec<_>>();
        Ok(Field {ranges})
    }
}

impl Field {
    fn accepts_value(&self, val: u32) -> bool {
        self.ranges.iter().any(|r| (r.0 <= val) & (val <= r.1))
    }    
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Couldn't read input");

    let mut chunks = input.split("\n\n");

    let fields: Vec<Field> = chunks.next().unwrap()
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let _you = chunks.next().unwrap();

    let nearby = chunks.next().unwrap()
        .trim()
        .strip_prefix("nearby tickets:\n")
        .unwrap()
        .split('\n')
        .fold(0, |sum, line| {
            sum + line.split(',')
                .fold(0, |line_sum, x| {
                    let num = u32::from_str(x).unwrap();
                    if fields.iter().any(|field| field.accepts_value(num)) {line_sum} else {line_sum + num}  
                })
        });

    println!("Part 1: {}", nearby);
}
