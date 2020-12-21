use std::io::BufRead;
use std::str::FromStr;


fn simple_update(state: (i32, i32, usize), step: &String) -> (i32, i32, usize) {
    let (mut dir, dist) = (step.chars().nth(0).unwrap(),
        i32::from_str(step.get(1..).unwrap()).unwrap());

    if dir == 'F' { dir = ['N', 'E', 'S', 'W'][state.2] };

    match dir {
        'N' => (0, dist, 0),
        'S' => (0, -dist, 0),
        'E' => (dist, 0, 0),
        'W' => (-dist, 0, 0),
        'R' => (0, 0, (dist / 90) as usize),
        'L' => (0, 0, 360 - (dist / 90) as usize),
        _ => state
    }
}

fn main() {
    let steps = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    
    let position = steps.iter().fold((0 as i32, 0 as i32, 1 as usize), |state, step| {
        let update = simple_update(state, step);
        (state.0 + update.0, state.1 + update.1, (state.2 + update.2) % 4)
    });

    println!("Part 1: {:?}", position.0.abs() + position.1.abs());
}
