use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("8.txt").expect("Unable to read file");
    let lines = data.split_terminator("\n").collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines[0].len();

    let mut nodes_map: BTreeMap<&str, Vec<(i32, i32)>> = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.split("").filter(|x| !x.is_empty()).enumerate() {
            if c != "." && !c.is_empty() {
                if let Some(node) = nodes_map.get_mut(c) {
                    if !node.contains(&(y as i32, x as i32)) {
                        node.push((y as i32, x as i32));
                    }
                } else {
                    nodes_map.insert(c, vec![(y as i32, x as i32)]);
                }
            }
        }
    }

    let mut antinodes_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (k, v) in nodes_map.iter() {
        for (i, node) in v.iter().enumerate() {
            let after = &v[i + 1..];
            for a in after {
                let antinodes = get_antinodes_part1(node, a, height as i32 - 1, width as i32 - 1);
                for antinode in antinodes.iter() {
                    if !antinodes_set.contains(antinode) {
                        antinodes_set.insert(*antinode);
                        // println!("{:?}:{:?} {:?}", node, a, antinode);
                    }
                }
            }
        }
    }

    // let antinodes = antinodes_map.values().flatten().collect::<Vec<_>>();
    // for (y, line) in lines.iter().enumerate() {
    //     for (x, c) in line.split("").filter(|x| !x.is_empty()).enumerate() {
    //         if antinodes.contains(&&(y as i32, x as i32)) {
    //             print!("#");
    //         } else {
    //             print!("{}", c);
    //         }
    //     }
    //     for (id, antin) in antinodes.iter().enumerate() {
    //         if antin.0 == y as i32 {
    //             print!("  {}, {:?}", id, antin);
    //         }
    //     }
    //     println!();
    // }
    println!("part1:{}", antinodes_set.len());

    let mut antinodes_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (k, v) in nodes_map.iter() {
        for (i, node) in v.iter().enumerate() {
            let after = &v[i + 1..];
            for a in after {
                let antinodes = get_antinodes_part2(node, a, height as i32 - 1, width as i32 - 1);
                for antinode in antinodes.iter() {
                    if !antinodes_set.contains(antinode) {
                        antinodes_set.insert(*antinode);
                        // println!("{:?}:{:?} {:?}", node, a, antinode);
                    }
                }
            }
        }
    }
    println!("part2:{}", antinodes_set.len());
}

fn get_antinodes_part1(
    node1: &(i32, i32),
    node2: &(i32, i32),
    max_y: i32,
    max_x: i32,
) -> BTreeSet<(i32, i32)> {
    let mut antinodes = BTreeSet::new();
    let dist_y = if node1.0 > node2.0 {
        node1.0 - node2.0
    } else {
        node2.0 - node1.0
    };

    let dist_x = if node1.1 > node2.1 {
        node1.1 - node2.1
    } else {
        node2.1 - node1.1
    };

    let anti_node1 = get_antinode(node1, node1.0 > node2.0, node1.1 > node2.1, dist_x, dist_y);
    if anti_node1.1 >= 0 && anti_node1.1 <= max_x && anti_node1.0 >= 0 && anti_node1.0 <= max_y {
        antinodes.insert(anti_node1);
    }
    let anti_node2 = get_antinode(node2, node1.0 < node2.0, node1.1 < node2.1, dist_x, dist_y);
    if anti_node2.1 >= 0 && anti_node2.1 <= max_x && anti_node2.0 >= 0 && anti_node2.0 <= max_y {
        antinodes.insert(anti_node2);
    }
    antinodes
}

fn get_antinodes_part2(
    node1: &(i32, i32),
    node2: &(i32, i32),
    max_y: i32,
    max_x: i32,
) -> BTreeSet<(i32, i32)> {
    let mut antinodes = BTreeSet::new();
    let dist_y = if node1.0 > node2.0 {
        node1.0 - node2.0
    } else {
        node2.0 - node1.0
    };

    let dist_x = if node1.1 > node2.1 {
        node1.1 - node2.1
    } else {
        node2.1 - node1.1
    };

    let mut anti_node1 = get_antinode(node1, node1.0 > node2.0, node1.1 > node2.1, dist_x, dist_y);
    while anti_node1.1 >= 0 && anti_node1.1 <= max_x && anti_node1.0 >= 0 && anti_node1.0 <= max_y {
        antinodes.insert(anti_node1);
        anti_node1 = get_antinode(
            &anti_node1,
            node1.0 > node2.0,
            node1.1 > node2.1,
            dist_x,
            dist_y,
        );
    }
    let mut anti_node2 = get_antinode(node2, node1.0 < node2.0, node1.1 < node2.1, dist_x, dist_y);
    while anti_node2.1 >= 0 && anti_node2.1 <= max_x && anti_node2.0 >= 0 && anti_node2.0 <= max_y {
        antinodes.insert(anti_node2);
        anti_node2 = get_antinode(
            &anti_node2,
            node1.0 < node2.0,
            node1.1 < node2.1,
            dist_x,
            dist_y,
        );
    }

    antinodes.insert(*node1);
    antinodes.insert(*node2);
    antinodes
}

fn get_antinode(
    node1: &(i32, i32),
    y_bigger: bool,
    x_bigger: bool,
    dist_x: i32,
    dist_y: i32,
) -> (i32, i32) {
    let anti_node1_y = if y_bigger {
        node1.0 + dist_y
    } else {
        node1.0 - dist_y
    };
    let anti_node1_x = if x_bigger {
        node1.1 + dist_x
    } else {
        node1.1 - dist_x
    };
    (anti_node1_y, anti_node1_x)
}
