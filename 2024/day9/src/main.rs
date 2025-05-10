use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("9.txt").expect("Unable to read file");
    let mut filemap: BTreeMap<i32, (usize, i32)> = BTreeMap::new();
    let mut space: BTreeSet<(usize, i32)> = BTreeSet::new();
    let mut memory_id: i32 = 0;
    let real_memory = data
        .split("")
        .filter(|x| !x.is_empty() && *x != "\n" && *x != "\r")
        .map(|x| x.parse().unwrap())
        .enumerate()
        .flat_map(|(id, x)| {
            if id % 2 == 0 {
                let mut real_value: Vec<i32> = vec![];
                let file_id = id / 2;
                filemap.insert(file_id as i32, (memory_id as usize, x));
                memory_id += x;
                for _ in 0..x {
                    real_value.push(file_id as i32);
                }
                real_value
            } else {
                space.insert((memory_id as usize, x));
                memory_id += x;
                let mut real_value: Vec<i32> = vec![];
                for _ in 0..x {
                    real_value.push(-1);
                }
                real_value
            }
        })
        .collect::<Vec<i32>>();

    // for r in real_memory.iter() {
    //     if *r == -1 {
    //         print!(".")
    //     } else {
    //         print!("{}", r);
    //     }
    // }
    // println!(": {}", real_memory.len());

    let mut comp_memory = real_memory.clone();
    for (id, i) in real_memory.iter().enumerate() {
        if *i == -1 {
            let to_be_moved_id_inv = comp_memory
                .clone()
                .iter()
                .rev()
                .position(|x| *x > 0)
                .unwrap();
            let to_be_moved_id = real_memory.len() - 1 - to_be_moved_id_inv;
            if to_be_moved_id < id {
                break;
            }
            comp_memory[id] = comp_memory[to_be_moved_id];
            comp_memory[to_be_moved_id] = -1;
            // for r in comp_memory.iter() {
            //     if *r == -1 {
            //         print!(".")
            //     } else {
            //         print!("{}", r);
            //     }
            // }
            // println!(": {} {} {}", comp_memory.len(), to_be_moved_id, id);
        }
    }

    let part1_checksum = comp_memory
        .into_iter()
        .filter(|x| *x >= 0)
        .enumerate()
        .fold(0_f64, |acc, (id, i)| acc + (id as f64 * i as f64));
    println!("part1:{}", part1_checksum);

    let mut new_space = space.clone();
    for (k, v) in filemap.iter_mut().rev() {
        // println!("{}, {:?}", k, v);

        let match_space = space.iter().find(|i| i.1 >= v.1 && i.0 < v.0);
        if let Some(match_space) = match_space {
            new_space.remove(match_space);
            new_space.insert(*v);
            if match_space.1 > v.1 {
                let gap = match_space.1 - v.1;
                new_space.insert((match_space.0 + v.1 as usize, gap));
            }
            v.0 = match_space.0;
            // println!("{:?}, {:?}", space, v);
        }
        space = new_space.clone()
    }

    let mut final_filemap: BTreeMap<usize, (i32, i32)> = BTreeMap::new();
    let mut new_real_memory: Vec<i32> = vec![];
    for (k, v) in filemap.iter() {
        // println!("{}: {:?}", k, v);
        final_filemap.insert(v.0, (*k, v.1));
    }
    for (k, v) in final_filemap.iter() {
        // println!("{}, {:?}", k, v);
        loop {
            if k > &new_real_memory.len() {
                new_real_memory.push(-1)
            } else {
                for _ in 0..v.1 {
                    new_real_memory.push(v.0)
                }
                break;
            }
        }
    }
    let mut part2_checksum = 0;
    for (i, r) in new_real_memory.iter().enumerate() {
        if *r == -1 {
            // print!(".")
        } else {
            // print!("{}", r);
            part2_checksum += i as i64 * *r as i64;
        }
    }

    println!("\npart2:{}", part2_checksum);
}
