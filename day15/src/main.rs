use std::io::Read;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Couldn't read input");

    let mut last_spoken = input.trim().split(",")
        .enumerate()
        .map(|(idx, num)| (usize::from_str(num).unwrap(), (idx + 1, 0)))
        .collect::<HashMap<_, _>>();
    
    let prev = usize::from_str(input.trim().split(",").last().unwrap()).unwrap();
    let start_idx = last_spoken.len();

    let last = (start_idx..30000000).fold(prev, |num, idx| {
        let history = last_spoken.get(&num).unwrap();
        let diff = if history.1 == 0 {0} else {history.0 - history.1};
        if let Some(hist) = last_spoken.get(&diff) {  // borrow checker warns here, should fix
            last_spoken.insert(diff, (idx + 1, hist.0));
        } else {
            last_spoken.insert(diff, (idx + 1, 0));
        }
        
        if idx + 1 == 2020 {
            println!("Part 1: {}", diff);
        }

        diff
    });
    
    println!("Part 2: {}", last);
}
