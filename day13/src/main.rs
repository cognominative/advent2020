use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let input = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let earliest = u32::from_str(input.get(0).unwrap()).unwrap();
    let buses = input.get(1).unwrap().split(",")
        .filter(|&bus| bus != "x")
        .map(|bus| bus.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut arrivals = buses.iter()
        .map(|bus| {(bus, bus * ((earliest / bus) + u32::from(earliest % bus != 0)))})
        .collect::<Vec<_>>();

    arrivals.sort_by(|a, b| (a.1).cmp(&b.1));

    println!("Part 1: {}", (arrivals[0].1 - earliest) * arrivals[0].0);
}
