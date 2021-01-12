use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;


fn parse_mask(mask: &str) -> (u64, u64) {
    mask.chars().fold((0, 0), |masks, c| {
        match c {
            '1' => ((masks.0 << 1) | 1, (masks.1 << 1) | 1),
            '0' => (masks.0 << 1, masks.1 << 1),
            _ =>  (masks.0 << 1, (masks.1 << 1) | 1)
        }
    })
}


fn main() {
    let lines = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut masks = (0b0u64, 0b0u64);
    let mut registers = HashMap::<String, u64>::new();

    for line in lines {
        if line.starts_with("mask = ") {
            masks = parse_mask(&line[7..]);
        } else if line.starts_with("mem") {
            let mut chunks = line.split(" = ");
            let register = chunks.next().unwrap();
            let value = u64::from_str(chunks.next().unwrap()).unwrap();

            registers.insert(register.to_string(), (value | masks.0) & masks.1);
        }
    }

    println!("{:?}", registers.values().sum::<u64>());
}
