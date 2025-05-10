use std::{collections::BTreeMap, fs};

fn main() {
    let data = fs::read_to_string("11.txt").expect("Unable to read file");
    let line = data.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    let mut new_line = line.clone();
    for n in 0..25 {
        new_line = blink(&new_line);
    }
    println!("part1: {}", new_line.len());

    // let mut new_line = line;
    let mut stone_map: BTreeMap<i64, i64> = BTreeMap::new();
    for i in line.iter() {
        if let Some(stonez) = stone_map.get_mut(i){
            *stonez += 1;
        } else {
            stone_map.insert(*i, 1);
        }
    }

    let mut last_stone_map: BTreeMap<i64, i64> = stone_map;
    for n in 0..75 {
        let mut new_stone_map: BTreeMap<i64, i64> = BTreeMap::new();
        for (k,v) in last_stone_map.iter() {
            if *k == 0 {
                if let Some(stonez) = new_stone_map.get_mut(&1){
                    *stonez += v;
                } else {
                    new_stone_map.insert(1, *v);
                }
            } else {
                // println!("{} :{:?}", k,v);
                let digits = k.ilog10() + 1;
                // println!("{}",digits);
                if digits % 2 == 0 {
                    let new_x_1 = k / 10_i64.pow(digits / 2);
                    let new_x_2 = k % 10_i64.pow(digits / 2);
                    if let Some(stonez) = new_stone_map.get_mut(&new_x_1){
                        *stonez += v;
                    } else {
                        new_stone_map.insert(new_x_1, *v);
                    }
                    if let Some(stonez) = new_stone_map.get_mut(&new_x_2){
                        *stonez += v;
                    } else {
                        new_stone_map.insert(new_x_2, *v);
                    }
                } else {
                    let new_x = k * 2024;
                    if let Some(stonez) = new_stone_map.get_mut(&new_x){
                        *stonez += v;
                    } else {
                        new_stone_map.insert(new_x, *v);
                    }
                }
            }
        }
        // for (k,v) in new_stone_map.iter() {
        //     println!("{} :{:?}", k,v);
        // }
        // println!();
        last_stone_map = new_stone_map;
    }

    for (k,v) in last_stone_map.iter() {
        // println!("{} :{:?}", k,v);
    }

    println!("part2 :{}", last_stone_map.values().sum::<i64>());
}

fn blink(input: &Vec<i64>) -> Vec<i64> {
    input.iter().flat_map(|x|{
        if *x == 0 {
            [1].to_vec()
        } else {
            let digits = x.ilog10() + 1;
            // println!("{}",digits);
            if digits % 2 == 0 {
                let new_x_1 = x / 10_i64.pow(digits / 2);
                let new_x_2 = x % 10_i64.pow(digits / 2);
                [new_x_1, new_x_2].to_vec()
            } else {
                let new_x = x * 2024;
                [new_x].to_vec()
            }
        }
    }).collect::<Vec<i64>>()
}
