use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("10.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\r\n")
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut coor_map: BTreeMap<usize, BTreeSet<(usize, usize)>> = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, l) in line.iter().enumerate() {
            if let Some(coor_set) = coor_map.get_mut(l) {
                coor_set.insert((y, x));
            } else {
                coor_map.insert(*l, BTreeSet::from([(y, x)]));
            }
        }
    }

    let max_y = lines.len();
    let max_x = lines[0].len();

    // for (k, v) in coor_map.iter() {
    //     println!("{:?}: {:?}", k, v);
    // }

    let mut trail_set: BTreeSet<Vec<(usize, usize)>> = BTreeSet::new();
    let trailhead_map = get_next(&coor_map, &0, max_y, max_x);

    for (k, v) in trailhead_map.iter() {
        for c in v.iter() {
            trail_set.insert(vec![*k, *c]);
        }
    }

    for (k, v) in coor_map.iter() {
        if *k != 0 {
            let key_map = get_next(&coor_map, k, max_y, max_x);
            let mut next_trail_set = trail_set.clone();
            // println!("{:?} {:?}", v, last);
            for (key, value) in key_map.iter() {
                for v in trail_set.iter() {
                    let last = v.last().unwrap();
                    if key == last {
                        for next in value.iter() {
                            let mut next_trail = v.clone();
                            next_trail.push(*next);
                            next_trail_set.insert(next_trail);
                        }
                    }
                }
            }
            trail_set = next_trail_set;
        }
    }

    let full_paths = trail_set.iter().filter(|x| x.len() > 9).collect::<Vec<_>>();
    let mut trailhead_tails: BTreeSet<((usize, usize), (usize, usize))> = BTreeSet::new();
    for t in full_paths.iter() {
        trailhead_tails.insert((*t.first().unwrap(), *t.last().unwrap()));
    }

    println!("part1:{:?}", trailhead_tails.len());
    println!("part2:{:?}", full_paths.len());

    // for value in trail_set.iter() {
    //     println!("{:?}", value);
    // }
}

fn get_next(
    coor_map: &BTreeMap<usize, BTreeSet<(usize, usize)>>,
    current_key: &usize,
    max_y: usize,
    max_x: usize,
) -> BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> {
    let mut current_key_map: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> = BTreeMap::new();
    if let Some(coor_set) = coor_map.get(current_key) {
        for c in coor_set.iter() {
            if let Some(next_coor_set) = coor_map.get(&(current_key + 1)) {
                if c.0 > 0 && next_coor_set.contains(&(c.0 - 1, c.1)) {
                    if let Some(current_key_map_set) = current_key_map.get_mut(c) {
                        current_key_map_set.insert((c.0 - 1, c.1));
                    } else {
                        current_key_map.insert(*c, BTreeSet::from([(c.0 - 1, c.1)]));
                    }
                }
                if c.0 < max_y && next_coor_set.contains(&(c.0 + 1, c.1)) {
                    if let Some(current_key_map_set) = current_key_map.get_mut(c) {
                        current_key_map_set.insert((c.0 + 1, c.1));
                    } else {
                        current_key_map.insert(*c, BTreeSet::from([(c.0 + 1, c.1)]));
                    }
                }
                if c.1 > 0 && next_coor_set.contains(&(c.0, c.1 - 1)) {
                    if let Some(current_key_map_set) = current_key_map.get_mut(c) {
                        current_key_map_set.insert((c.0, c.1 - 1));
                    } else {
                        current_key_map.insert(*c, BTreeSet::from([(c.0, c.1 - 1)]));
                    }
                }
                if c.1 < max_x && next_coor_set.contains(&(c.0, c.1 + 1)) {
                    if let Some(current_key_map_set) = current_key_map.get_mut(c) {
                        current_key_map_set.insert((c.0, c.1 + 1));
                    } else {
                        current_key_map.insert(*c, BTreeSet::from([(c.0, c.1 + 1)]));
                    }
                }
            }
        }
    }
    current_key_map
}
