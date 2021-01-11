use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Couldn't read input");

    let sum = input.split("\n\n")
        .fold((0, 0), |sum, group| {
            let any = group.replace("\n", "").chars().collect::<HashSet<_>>();

            let all = any.iter()
                .filter(|&c| group.lines().all(
                    |line| line.chars().collect::<Vec<_>>().contains(c))
                );

            (sum.0 + any.len(), sum.1 + all.count())
        });

    println!("Part 1: {}", sum.0);
    println!("Part 2: {}", sum.1);
}
