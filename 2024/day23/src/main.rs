use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("23.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|x| x.split("-").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut nodes_map: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for p in lines.iter() {
        if let Some(node) = nodes_map.get_mut(p[0]) {
            if !node.contains(&p[1]) {
                node.push(p[1]);
            }
        } else {
            nodes_map.insert(p[0], vec![p[1]]);
        }
        if let Some(node) = nodes_map.get_mut(p[1]) {
            if !node.contains(&p[0]) {
                node.push(p[0]);
            }
        } else {
            nodes_map.insert(p[1], vec![p[0]]);
        }
    }

    // for (k, v) in nodes_map.iter() {
    //     println!("{} {:?}", k, v);
    // }

    let mut results: Vec<BTreeSet<&str>> = vec![];
    for (key, val) in nodes_map.iter() {
        // println!("{}: {:?}", key, val);
        for (id, v) in val.iter().enumerate() {
            let after = &val[id + 1..];
            if !after.is_empty() && after.len() < val.len() {
                for a in after.iter() {
                    if nodes_map.get(v).unwrap().contains(a) {
                        let set = BTreeSet::from([v.to_owned(), a.to_owned(), key.to_owned()]);
                        // println!("{:?}: {:?} {}", v, a, key);
                        if !results.contains(&set) {
                            results.push(set)
                        }
                    }
                }
                // println!("{}: {:?}", v, after);
            }
        }
    }
    let filtered_results = results
        .clone()
        .into_iter()
        .filter(|x| x.iter().any(|x| x.starts_with("t")))
        .collect::<Vec<BTreeSet<&str>>>();
    println!("part1: {:?}", filtered_results.len());

    let mut results: BTreeMap<BTreeSet<&str>, Vec<&str>> = BTreeMap::new();
    for (key, val) in nodes_map.iter() {
        // println!("{}: {:?}", key, val);
        for (id, v) in val.iter().enumerate() {
            let after = &val[id + 1..];
            if !after.is_empty() && after.len() < val.len() {
                for a in after.iter() {
                    if nodes_map.get(v).unwrap().contains(a) {
                        let keyset = BTreeSet::from([v.to_owned(), key.to_owned()]);
                        // // println!("{:?}: {:?} {}", v, a, key);
                        // if !results.contains(&set) {
                        //     results.push(set)
                        // }
                        if let Some(nets) = results.get_mut(&keyset) {
                            if !nets.contains(a) {
                                nets.push(a);
                            }
                        } else {
                            results.insert(keyset, vec![*a]);
                        }
                    }
                }
                // println!("{}: {:?}", v, after);
            }
        }
    }

    loop {
        results = extend_nets(results, &nodes_map);
        for (k, r) in results.iter() {
            // println!("{:?}: {:?}", k, r);
        }
        if results.len() < 2 {
            break;
        }
    }

    print!("part2: ");
    for r in results.keys() {
        for (id, ar) in r.iter().enumerate() {
            if id > 0 {
                print!(",")
            }
            print!("{}", ar)
        }
        println!()
    }
}

fn extend_nets<'a>(
    nets: BTreeMap<BTreeSet<&'a str>, Vec<&'a str>>,
    nodes_map: &BTreeMap<&'a str, Vec<&'a str>>,
) -> BTreeMap<BTreeSet<&'a str>, Vec<&'a str>> {
    let mut results: BTreeMap<BTreeSet<&str>, Vec<&str>> = BTreeMap::new();
    for (key, val) in nets.iter() {
        // println!("{}: {:?}", key, val);
        if val.len() == 1 {
            let cons = nodes_map.get(val[0]).unwrap();
            if key.iter().all(|x| cons.contains(x)) {
                let mut keyset = BTreeSet::from([val[0]]);
                for k in key.iter() {
                    keyset.insert(k);
                }
                results.insert(keyset, vec![]);
            }
        } else {
            for (id, v) in val.iter().enumerate() {
                let after = &val[id + 1..];
                if !after.is_empty() && after.len() < val.len() {
                    for a in after.iter() {
                        if nodes_map.get(v).unwrap().contains(a) {
                            let mut keyset = BTreeSet::from([v.to_owned()]);

                            for k in key.iter() {
                                keyset.insert(k);
                            }
                            // println!("{:?}: {:?} {}", v, a, key);
                            // if !results.contains(&set) {
                            //     results.push(set)
                            // }
                            if let Some(nets) = results.get_mut(&keyset) {
                                if !nets.contains(a) {
                                    nets.push(a);
                                }
                            } else {
                                results.insert(keyset, vec![*a]);
                            }
                        }
                    }
                    // println!("{}: {:?}", v, after);
                }
            }
        }
    }
    results
}
