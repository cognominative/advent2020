use std::io::BufRead;
use std::str::FromStr;


fn main() {
    let steps = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    
    let position = steps.iter().fold((0, 0, 1), |state, step| {
        let (mut dir, dist) = (step.chars().nth(0).unwrap(),
            i32::from_str(step.get(1..).unwrap()).unwrap());

        if dir == 'F' { dir = ['N', 'E', 'S', 'W'][state.2] };

        match dir {
            'N' => (state.0, state.1 + dist, state.2),
            'S' => (state.0, state.1 - dist, state.2),
            'E' => (state.0 + dist, state.1, state.2),
            'W' => (state.0 - dist, state.1, state.2),
            'R' => (state.0, state.1, (state.2 + (dist / 90) as usize) % 4),
            'L' => (state.0, state.1, (state.2 + 360 - (dist / 90) as usize) % 4),
            _ => state
        }
    });

    println!("Part 1: {:?}", position.0.abs() + position.1.abs());
}
