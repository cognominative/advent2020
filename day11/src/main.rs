use std::io::BufRead;


fn main() {
    let seating = std::io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let nrows = seating.len();
    let ncols = seating.get(0).unwrap().len();
    let mut seating = seating.iter().flatten().map(|&x| x).collect::<Vec<_>>();

    let neighbors = |x: usize, state: &Vec<char>| {
        let r = (x / ncols) as i32;
        let c = (x as i32) - r * (ncols as i32);

        let nn_diffs = [(-1, 1),  (0, 1),  (1, 1),
                        (-1, 0),           (1, 0),
                        (-1, -1), (0, -1), (1, -1)];

        nn_diffs.iter().filter(|(dr, dc)| (r + dr >= 0) & (c + dc >= 0) & (r + dr < nrows as i32) & (c + dc < ncols as i32))
            .fold(0, |sum, (dr, dc)| {
                let neighbor = (r + dr) as usize * ncols + (c + dc) as usize;
                if let Some(seat) = state.get(neighbor) {
                    if *seat == '#' {
                        return sum + 1
                    }
                }
                sum
            })
    };

    let update = |state: Vec<char>| { (0..state.len())
        .map(|x| {
            let seat = state.get(x).unwrap();
            match seat {
                'L' => if neighbors(x, &state) == 0 {'#'} else {'L'}
                '#' => if neighbors(x, &state) > 3 {'L'} else {'#'},
                _ => *seat               
            }
        })
        .collect::<Vec<_>>()
    };

    // Fighting the borrow checker here, which seems like a sign I'm Doing It Wrong somehow
    let mut new_state = update(seating.iter().map(|&x| x).collect::<Vec<char>>());

    while new_state != seating {
        seating = new_state;
        new_state = update(seating.iter().map(|&x| x).collect::<Vec<char>>());
    }

    let occupied = new_state.iter().filter(|&&seat| seat == '#').collect::<Vec<_>>().len();
    
    println!("Part 1: {:?}", occupied);
}
