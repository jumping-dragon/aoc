use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
fn main() {
    let data = fs::read_to_string("12.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|n| n.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut coor_map: BTreeMap<&str, BTreeSet<(usize, usize)>> = BTreeMap::new();

    for (y, line) in lines.iter().enumerate() {
        // println!("{:?}", line);
        for (x, l) in line.iter().enumerate() {
            if l == &"V" {
                print!("{}", l);
            } else {
                print!(".");
            }
            if let Some(coor_set) = coor_map.get_mut(l) {
                coor_set.insert((x, y));
            } else {
                coor_map.insert(l, BTreeSet::from([(x, y)]));
            }
        }
        println!();
    }

    let mut total_cost = 0;
    for (k, v) in coor_map.iter() {
        let groups = get_groups(v);
        for group in groups.iter() {
            let mut total_perimeter = 0;
            let area = group.len() as i32;
            for point in group.iter() {
                let perimeter = get_perimeter(*point, v);
                // println!(
                //     "{}: {:?} area: {} perimeter: {} {:?}",
                //     k, v, area, perimeter, point
                // );
                total_perimeter += perimeter;
            }
            total_cost += area * total_perimeter;
        }
        // println!("{} {:?} {}", k, groups, groups.len());
    }
    println!("part1: total_cost: {}", total_cost);

    let mut total_cost = 0;
    for (k, v) in coor_map.iter() {
        let groups = get_groups(v);
        for group in groups.iter() {
            let mut total_perimeter = 0;
            let area = group.len() as i32;
            let mut perim_types: BTreeMap<(usize, usize), BTreeSet<usize>> = BTreeMap::new();
            for point in group.iter() {
                let sidez = get_sidez(*point, v);
                for side in sidez.iter() {
                    if !perim_types.contains_key(&side.0) {
                        perim_types.insert(side.0, BTreeSet::from([side.1]));
                    } else {
                        if let Some(sides) = perim_types.get_mut(&side.0) {
                            sides.insert(side.1);
                        }
                    }
                }
            }
            for p in perim_types.iter() {
                // let mut map: BTreeMap<usize, &str> = BTreeMap::new();
                // map.insert(0, "left");
                // map.insert(1, "right");
                // map.insert(2, "up");
                // map.insert(3, "down");
                let num_groups = get_num_groups(p.1);
                // println!(
                //     "{}: {:?} area: {} side: {:?} {} {}",
                //     k, v, area, num_groups, map[&p.0 .0], p.0 .1
                // );
                total_perimeter += num_groups.len();
            }
            // println!("k {}, {} {}", k, area, total_perimeter);
            total_cost += area * total_perimeter as i32;
        }
        // println!("{} {:?} {}", k, groups, groups.len());
    }
    println!("part2: total_cost: {}", total_cost);
}

fn get_perimeter(s: (usize, usize), set: &BTreeSet<(usize, usize)>) -> i32 {
    let mut total_perimeter = 0;
    if s.0 == 0 || !set.contains(&(s.0 - 1, s.1)) {
        // up
        total_perimeter += 1;
    }
    if !set.contains(&(s.0 + 1, s.1)) {
        // down
        total_perimeter += 1;
    }
    if s.1 == 0 || !set.contains(&(s.0, s.1 - 1)) {
        // left
        total_perimeter += 1;
    }
    if !set.contains(&(s.0, s.1 + 1)) {
        // right
        total_perimeter += 1;
    }
    total_perimeter
}

fn get_sidez(s: (usize, usize), set: &BTreeSet<(usize, usize)>) -> Vec<((usize, usize), usize)> {
    let mut sidez = vec![];
    if s.0 == 0 || !set.contains(&(s.0 - 1, s.1)) {
        // left
        sidez.push(((0, s.0), s.1));
    }
    if !set.contains(&(s.0 + 1, s.1)) {
        // right
        sidez.push(((1, s.0), s.1));
    }
    if s.1 == 0 || !set.contains(&(s.0, s.1 - 1)) {
        // up
        sidez.push(((2, s.1), s.0));
    }
    if !set.contains(&(s.0, s.1 + 1)) {
        // down
        sidez.push(((3, s.1), s.0));
    }
    sidez
}

fn get_groups(set: &BTreeSet<(usize, usize)>) -> Vec<BTreeSet<(usize, usize)>> {
    let mut set_state = set.clone();
    let mut set_vec: Vec<BTreeSet<(usize, usize)>> = vec![];
    let mut current_vector: BTreeSet<(usize, usize)> = BTreeSet::new();
    while let Some(p) = set_state.pop_first() {
        current_vector.insert(p);
        loop {
            let neighbours = set_state
                .clone()
                .into_iter()
                .filter(|s| {
                    for v in current_vector.iter() {
                        if v.0 == 0 && v.1 == 0 {
                            if (v.1 == s.1 && v.0 + 1 == s.0) || (v.0 == s.0 && v.1 + 1 == s.1) {
                                return true;
                            }
                        } else if v.0 == 0 {
                            if (v.1 == s.1 && v.0 + 1 == s.0)
                                || (v.0 == s.0 && (v.1 + 1 == s.1 || v.1 - 1 == s.1))
                            {
                                return true;
                            }
                        } else if v.1 == 0 {
                            if (v.1 == s.1 && (v.0 + 1 == s.0 || v.0 - 1 == s.0))
                                || (v.0 == s.0 && v.1 + 1 == s.1)
                            {
                                return true;
                            }
                        } else {
                            if (v.1 == s.1 && (v.0 + 1 == s.0 || v.0 - 1 == s.0))
                                || (v.0 == s.0 && (v.1 + 1 == s.1 || v.1 - 1 == s.1))
                            {
                                return true;
                            }
                        }
                    }
                    false
                })
                .collect::<Vec<(usize, usize)>>();
            if neighbours.is_empty() {
                set_vec.push(current_vector.clone());
                current_vector.clear();
                break;
            } else {
                for n in neighbours.iter() {
                    set_state.remove(n);
                    current_vector.insert(*n);
                }
            }
        }
    }

    set_vec
}

fn get_num_groups(i: &BTreeSet<usize>) -> Vec<BTreeSet<usize>> {
    let mut set_state = i.clone();
    let mut set_vec: Vec<BTreeSet<usize>> = vec![];
    let mut current_vector: BTreeSet<usize> = BTreeSet::new();
    while let Some(v) = set_state.pop_first() {
        current_vector.insert(v);
        loop {
            let neighbours = set_state
                .clone()
                .into_iter()
                .filter(|s| {
                    for v in current_vector.iter() {
                        if v == &0 {
                            if *s == v + 1 {
                                return true;
                            }
                        } else {
                            if *s == v + 1 || *s == v - 1 {
                                return true;
                            }
                        }
                    }
                    false
                })
                .collect::<Vec<usize>>();
            if neighbours.is_empty() {
                set_vec.push(current_vector.clone());
                current_vector.clear();
                break;
            } else {
                for n in neighbours.iter() {
                    set_state.remove(n);
                    current_vector.insert(*n);
                }
            }
        }
    }
    set_vec
}
