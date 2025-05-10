use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("6.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|x| x.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut guard_pos = (0_i32, 0_i32);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == "^" {
                guard_pos = (y as i32, x as i32);
            }
        }
    }
    let init_guard_pos = guard_pos;
    let mut direction = 0;
    let max_x = lines[0].len() as i32 - 1;
    let max_y = lines.len() as i32 - 1;
    let mut visited_pos: BTreeSet<(i32, i32)> = BTreeSet::new();
    visited_pos.insert(guard_pos);

    loop {
        let mut next_position = guard_pos;
        match direction {
            0 => {
                // up
                next_position.0 -= 1;
            }
            1 => {
                // right
                next_position.1 += 1;
            }
            2 => {
                // down
                next_position.0 += 1;
            }
            3 => {
                // left
                next_position.1 -= 1;
            }
            _ => todo!(),
        }
        if next_position.1 >= 0
            && next_position.1 <= max_x
            && next_position.0 >= 0
            && next_position.0 <= max_y
        {
            if lines[next_position.0 as usize][next_position.1 as usize] == "#" {
                if direction == 3 {
                    direction = 0;
                } else {
                    direction += 1;
                }
                visited_pos.insert(guard_pos);
            } else {
                guard_pos = next_position;
                visited_pos.insert(guard_pos);
            }
        } else {
            break;
        }
    }
    println!("part1:{}", visited_pos.len());

    let mut rock_pos: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != "#" {
                println!("run {:?}", (y, x));
                if run_sim(&lines, &init_guard_pos, &(y as i32, x as i32), max_x, max_y) {
                    println!("found {:?}", (y, x));
                    rock_pos.insert((y as i32, x as i32));
                }
            } else {
                println!("skip # {:?}", (y, x))
            }
            // let has_up = directional_visited_pos
            //     .get(&0)
            //     .unwrap()
            //     .contains(&(y as i32, x as i32));
            // let has_right = directional_visited_pos
            //     .get(&1)
            //     .unwrap()
            //     .contains(&(y as i32, x as i32));
            // let has_down = directional_visited_pos
            //     .get(&2)
            //     .unwrap()
            //     .contains(&(y as i32, x as i32));
            // let has_left = directional_visited_pos
            //     .get(&3)
            //     .unwrap()
            //     .contains(&(y as i32, x as i32));
            // if (y as i32, x as i32) == init_guard_pos {
            //     print!("{}", c);
            // } else if rock_pos.contains(&(y as i32, x as i32)) {
            //     print!("O");
            // } else if (has_up || has_down) && (has_left || has_right) {
            //     print!("+");
            // } else if has_up || has_down {
            //     print!("|");
            // } else if has_left || has_right {
            //     print!("-");
            // } else {
            //     print!("{}", c);
            // }
        }
        // println!()
    }
    // println!("rock_pos:{:?} {}", rock_pos, rock_pos.len());
    println!("part2: {}", rock_pos.len());
}

fn run_sim(
    lines: &Vec<Vec<&str>>,
    init_guard_pos: &(i32, i32),
    rock_position: &(i32, i32),
    max_x: i32,
    max_y: i32,
) -> bool {
    let mut guard_pos = *init_guard_pos;
    let mut direction = 0;
    let mut directional_visited_pos: BTreeMap<(i32, i32), i32> = BTreeMap::new();

    loop {
        let mut next_position = guard_pos;
        match direction {
            0 => {
                // up
                next_position.0 -= 1;
            }
            1 => {
                // right
                next_position.1 += 1;
            }
            2 => {
                // down
                next_position.0 += 1;
            }
            3 => {
                // left
                next_position.1 -= 1;
            }
            _ => todo!(),
        }
        if next_position.1 >= 0
            && next_position.1 <= max_x
            && next_position.0 >= 0
            && next_position.0 <= max_y
        {
            if lines[next_position.0 as usize][next_position.1 as usize] == "#"
                || next_position == *rock_position
            {
                if direction == 3 {
                    direction = 0;
                } else {
                    direction += 1;
                }
                if let Some(pos) = directional_visited_pos.get_mut(&guard_pos) {
                    if *pos == 4 {
                        return true;
                    }
                    *pos += 1;
                } else {
                    directional_visited_pos.insert(guard_pos, 0);
                }
            } else {
                guard_pos = next_position;
                if let Some(pos) = directional_visited_pos.get_mut(&guard_pos) {
                    if *pos == 4 {
                        return true;
                    }
                    *pos += 1;
                } else {
                    directional_visited_pos.insert(guard_pos, 0);
                }
            }
        } else {
            return false;
        }
    }
}
