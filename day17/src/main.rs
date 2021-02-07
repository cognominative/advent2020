#[macro_use] extern crate itertools;

use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let lines = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    
    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();

    let mut xlim = (0, rows as i32);
    let mut ylim = (0, cols as i32);
    let mut zlim = (0, 1);
    let mut wlim = (0, 1);

    let mut state = iproduct!(0..rows, 0..rows).map(
        |(x, y)| {
            let init = lines.get(y).unwrap().chars().nth(x).unwrap();
            ((x as i32, y as i32, 0, 0), init)
        }
    ).collect::<HashMap<(i32, i32, i32, i32), char>>();

    let neighbors = |(x, y, z, w): (i32, i32, i32, i32), state: &HashMap<(i32, i32, i32, i32), char>| {
        iproduct!(-1..2, -1..2, -1..2, -1..2).filter(|(dx, dy, dz, dw)| {
            state.get(&(x + dx, y + dy, z + dz, w + dw)).unwrap_or(&'.') == &'#'
        }).count()
    };

    for _ in 0..6 {
        xlim = (xlim.0 - 1, xlim.1 + 1);
        ylim = (ylim.0 - 1, ylim.1 + 1);
        zlim = (zlim.0 - 1, zlim.1 + 1);
        wlim = (wlim.0 - 1, wlim.1 + 1);

        let new_state = iproduct!(xlim.0..xlim.1, ylim.0..ylim.1, zlim.0..zlim.1, wlim.0..wlim.1).map(|coord| {
            match neighbors(coord, &state){
                4 => (coord, *state.get(&coord).unwrap_or(&'.')),
                3 => (coord, '#'),
                _ => (coord, '.')
            }
        }).collect::<HashMap<_,_>>();

        state = new_state;
    }

    println!("Part 2: {}", state.iter().filter(|(_, &x)| x == '#').count());
}
