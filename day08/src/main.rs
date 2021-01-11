use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut instructions = std::io::stdin().lock().lines()
        .map(|line| {
            let line = line.unwrap();
            (line[0..3].to_string(), i32::from_str(&line[4..]).unwrap(), false)
        })
        .collect::<Vec<_>>();

    let mut ptr = 0;
    let mut acc = 0;

    loop {
        match instructions[ptr].2 {
            true => break,
            false => instructions[ptr].2 = true
        }
        match instructions[ptr].0.as_str() {
            "nop" => ptr += 1,
            "acc" => {acc += instructions[ptr].1; ptr += 1},
            "jmp" => {ptr = (ptr as i32 + instructions[ptr].1) as usize},
            _ => {}
        }

        // println!("{:?}, {}", instructions[ptr], acc);
    }

    println!("Part 1: {:?}", acc);
}
