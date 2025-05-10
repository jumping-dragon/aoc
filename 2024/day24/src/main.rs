use std::collections::BTreeSet;
use std::{collections::BTreeMap, fs};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Gates {
    XOR,
    OR,
    AND
}

impl FromStr for Gates {
    type Err = ();
    fn from_str(input: &str) -> Result<Gates, Self::Err> {
        match input {
            "XOR"  => Ok(Gates::XOR),
            "OR"  => Ok(Gates::OR),
            "AND"  => Ok(Gates::AND),
            _      => Err(()),
        }
    }
}

fn main() {
    let data = fs::read_to_string("24.txt").expect("Unable to read file");
    let block = data.split_terminator("\n\n").collect::<Vec<&str>>();
    let bits = block[0].split_terminator("\n").collect::<Vec<&str>>();
    let ops = block[1].split_terminator("\n").collect::<Vec<&str>>();

    let mut source_map: BTreeMap<&str, usize> = BTreeMap::new();
    for bit in bits.iter() {
        let parts = bit.split(": ").collect::<Vec<&str>>();
        let key = parts[0];
        let value = parts[1].parse::<usize>().unwrap();
        source_map.insert(key, value);
    }

    let mut ops_map: BTreeMap<&str, (Gates, &str, &str)> = BTreeMap::new();
    for op in ops.iter() {
        let parts = op.split(" -> ").collect::<Vec<&str>>();
        let key = parts[1];
        let values = parts[0].split(" ").collect::<Vec<&str>>();
        ops_map.insert(key, (Gates::from_str(values[1]).unwrap(), values[0], values[2]));
    }
    let mut raw_ops_map = ops_map.clone();

    let mut wire_map = source_map.clone();
    loop {
        if ops_map.is_empty() {
            break
        } else {
            for (k,v) in ops_map.iter() {
                if let Some(val_1) = wire_map.get(v.1) {
                    if let Some(val_2) = wire_map.get(v.2) {
                        wire_map.insert(k, get_result(&v.0, (*val_1,*val_2)));
                    }
                }
            }
            for k in wire_map.keys() {
                if ops_map.contains_key(k) {
                    ops_map.remove(k);
                }
            }
        }
        // for (k,v) in wire_map.iter() {
        //     println!("{} {:?}", k,v);
        // }
        // for (k,v) in ops_map.iter() {
        //     println!("{} {:?}", k,v);
        // }
    }

    let part1_ans_bin = wire_map.iter().filter(|(k,v)| k.starts_with("z")).map(|(k,v)| *v).rev().fold("".to_string(), |acc,x| format!("{}{}", acc, x.to_string().as_str()));
    let part1_ans = isize::from_str_radix(part1_ans_bin.as_str(), 2).unwrap();
    println!("part1: {:?}", part1_ans);

    let x_ans_bin = wire_map.iter().filter(|(k,v)| k.starts_with("x")).map(|(k,v)| *v).rev().fold("".to_string(), |acc,x| format!("{}{}", acc, x.to_string().as_str()));
    let x_ans = isize::from_str_radix(x_ans_bin.as_str(), 2).unwrap();
    println!(" {:b} {} x", x_ans, x_ans);

    let y_ans_bin = wire_map.iter().filter(|(k,v)| k.starts_with("y")).map(|(k,v)| *v).rev().fold("".to_string(), |acc,x| format!("{}{}", acc, x.to_string().as_str()));
    let y_ans = isize::from_str_radix(y_ans_bin.as_str(), 2).unwrap();
    println!(" {:b} {} y", y_ans, y_ans);

    let correct_ans = x_ans + y_ans;
    let correct_ans_bin = format!("{:b}", correct_ans);
    println!("{} {} correct", correct_ans_bin, correct_ans);
    println!("{:b} {} answer", part1_ans,part1_ans);

    let mut part2_ans: BTreeSet<&str> = BTreeSet::new();

    let mut corrected_ops_map = raw_ops_map.clone();

    // Find all wrong XOR sums and its switch pair
    for (k,v) in raw_ops_map.iter() {
        if k.starts_with("z") && k != &"z45"  {
            if v.0 != Gates::XOR {
                println!("{} {:?}", k,v);
                part2_ans.insert(k);
                // wrong_bit_ids.remove(*k);
                let (ksum,valsum) = raw_ops_map.iter().find(|(key,val)| val.0 == Gates::XOR && (val.1 == k.replace("z", "x") || val.1 == k.replace("z", "y"))).unwrap();
                let (ksumxor, valsumxor) = raw_ops_map.iter().find(|(key,val)| val.0 == Gates::XOR && (val.1 == *ksum|| val.2 == *ksum)).unwrap();
                part2_ans.insert(ksumxor);
                let target_switch = corrected_ops_map.insert(k, *valsumxor).unwrap();
                corrected_ops_map.insert(ksumxor, target_switch);
            } else {
                let (ksum,valsum) = raw_ops_map.iter().find(|(key,val)| val.0 == Gates::XOR && (val.1 == k.replace("z", "x") || val.1 == k.replace("z", "y"))).unwrap();
                // println!("{} {:?}", ksum,valsum);
                if ksum == &v.1 || ksum == &v.2 {
                    continue
                } else {
                    if let Some(val_1) = corrected_ops_map.get(v.1) {
                        // println!("  {} {:?}", k, val_1);
                        if val_1.0 == Gates::AND {
                            part2_ans.insert(v.1);
                            part2_ans.insert(ksum);
                            let target_switch = corrected_ops_map.insert(v.1, *valsum).unwrap();
                            corrected_ops_map.insert(ksum, target_switch);
                        }
                    }
                    if let Some(val_2) = corrected_ops_map.get(v.2) {
                        // println!("  {} {:?}", k, val_2);
                        if val_2.0 == Gates::AND {
                            part2_ans.insert(v.2);
                            part2_ans.insert(ksum);
                            let target_switch = corrected_ops_map.insert(v.2, *valsum).unwrap();
                            corrected_ops_map.insert(ksum, target_switch);
                        }
                    }
                }
            }
        }
    }
    for (k,v) in corrected_ops_map.iter() {
            // println!("{} {:?}", k,v);
    }

    println!("part2: {:?} {}", part2_ans, part2_ans.len());
    for p in part2_ans.iter() {
        print!("{},",p);
    }
    println!();
    //
    let mut new_wire_map = source_map.clone();
    loop {
        if corrected_ops_map.is_empty() {
            break
        } else {
            for (k,v) in corrected_ops_map.iter() {
                if let Some(val_1) = new_wire_map.get(v.1) {
                    if let Some(val_2) = new_wire_map.get(v.2) {
                        // println!("{} {}",k, get_result(&v.0, (*val_1,*val_2)));
                        new_wire_map.insert(k, get_result(&v.0, (*val_1,*val_2)));
                    }
                }
            }
            for k in new_wire_map.keys() {
                if corrected_ops_map.contains_key(k) {
                    corrected_ops_map.remove(k);
                }
            }
        }
        // for (k,v) in corrected_ops_map.iter() {
        //     println!("{} {:?}", k,v);
        // }
        // for (k,v) in wire_map.iter() {
        //     println!("{} {:?}", k,v);
        // }
    }
    let corrected_ans_bin = new_wire_map.iter().filter(|(k,v)| k.starts_with("z")).map(|(k,v)| *v).rev().fold("".to_string(), |acc,x| format!("{}{}", acc, x.to_string().as_str()));
    let corrected_ans = isize::from_str_radix(corrected_ans_bin .as_str(), 2).unwrap();
    println!("{} {} correct", correct_ans_bin, correct_ans);
    println!("{:b} {} corrected", corrected_ans,corrected_ans);
}

fn get_result(relation: &Gates, v: (usize, usize)) -> usize {
    match relation {
        Gates::XOR => {
            match v {
                (0,0) => 0,
                (0,1) => 1,
                (1,0) => 1,
                (1,1) => 0,
                _ => todo!()
            }
        }
        Gates::OR => {
            match v {
                (0,0) => 0,
                (0,1) => 1,
                (1,0) => 1,
                (1,1) => 1,
                _ => todo!()
            }
        }
        Gates::AND => {
            match v {
                (0,0) => 0,
                (0,1) => 0,
                (1,0) => 0,
                (1,1) => 1,
                _ => todo!()
            }
        }
    }
}
