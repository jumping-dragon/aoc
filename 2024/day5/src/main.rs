use std::{ collections::{BTreeMap, BTreeSet}, fs};

fn main() {
    let data = fs::read_to_string("5.txt").expect("Unable to read file");
    let parts = data.split_terminator("\n\n").collect::<Vec<&str>>();
    let rules = parts[0].split_terminator("\n").map(|x| x.split("|").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let updates = parts[1].split_terminator("\n").map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    // println!("{:?}", rules);
    let mut rules_map:BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for rule in rules.iter() {
        let key = rule[0];
        let val = rule[1];
        if let Some(rules_set) = rules_map.get_mut(key) {
            rules_set.insert(val);
        } else {
            rules_map.insert(key, BTreeSet::from([val]));
        }
    }
    // println!("{:?}", rules_map);
    // println!("{:?} {}", updates, updates.len());
    let filtered_updates: Vec<Vec<&str>> = updates.clone().into_iter().filter(|x| {
        for (id,u) in x.iter().enumerate() {
            let after = &x[id+1..];
            for a in after.iter() {
                let rules = rules_map.get(a).unwrap();
                if rules.contains(u) {
                    // println!("{} {} {:?}", u, a, rules);
                    return false
                }
            }
            // }
        }
        true
    }).collect();
    // println!("{:?} {}", filtered_updates, filtered_updates.len());

    let middle_sum = filtered_updates.iter().map(|x| x[(x.len() - 1)/2].parse::<i32>().unwrap()).sum::<i32>();
    println!("part1:{}", middle_sum);

    let inc_filtered_updates: Vec<Vec<&str>> = updates.clone().into_iter().filter(|x| {
        for (id,u) in x.iter().enumerate() {
            let after = &x[id+1..];
            for a in after.iter() {
                let rules = rules_map.get(a).unwrap();
                if rules.contains(u) {
                    // println!("{} {} {:?}", u, a, rules);
                    return true
                }
            }
            // }
        }
        false
    }).collect();
    // println!("{:?} {}", inc_filtered_updates, inc_filtered_updates.len());
    let fixed_inc_filtered_updates: Vec<Vec<&str>> = inc_filtered_updates.clone().iter().map(|x| {
        let mut new_x = x.clone();
        let mut a = 0;
        let mut reset = false;
        loop {
            let afters = &new_x.clone()[a+1..];
            for after in afters.iter() {
                if check_rules_map(&rules_map, after, new_x[a]) {
                    let after_id = new_x.iter().position(|a| a == after).unwrap();
                    new_x.remove(after_id);
                    // println!("{:?} {}", new_x, after);
                    new_x.insert(a, after);
                    // println!("{:?} {}", new_x, after);
                    reset = true;
                    break
                }
            }
            if reset {
                a = 0;
                reset = false;
            } else {
            a += 1;
            if a == new_x.len() {
                break
            }
            }
        }
        new_x
    }).collect();
    // println!("{:?} {}", fixed_inc_filtered_updates, fixed_inc_filtered_updates.len());

    let middle_sum = fixed_inc_filtered_updates.iter().map(|x| x[(x.len() - 1)/2].parse::<i32>().unwrap()).sum::<i32>();
    println!("part2:{}", middle_sum);
}

fn check_rules_map(rules_map: &BTreeMap<&str, BTreeSet<&str>>, key: &str, checked: &str) -> bool {
    let rules = rules_map.get(key).unwrap();
    rules.contains(checked)
}
