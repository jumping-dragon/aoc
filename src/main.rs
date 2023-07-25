use std::cell::Cell;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::{collections::HashMap, fs};

use anyhow::Error;

fn main() -> Result<(), Error> {
    // --- Day 1: Calorie Counting ---
    //
    // let data = fs::read_to_string("1.txt").expect("Unable to read file");
    // let mut collection = data.split_terminator("\r\n\r\n").map(|n| {
    //     n.split_terminator("\r\n")
    //     .map(|n| { n.parse::<i32>().unwrap()})
    //     .fold(0, |acc, x| acc + x)
    // }).collect::<Vec<i32>>();
    // collection.sort();

    // let max = collection[collection.len() - 1];
    // let sum = collection.as_slice()[collection.len()-3..].iter().fold(0, |acc, x| acc + x);

    // println!("max:{:?}, sum: {}", max, sum);

    // --- Day 2: Rock Paper Scissors ---
    //
    // let data = fs::read_to_string("2.txt").expect("Unable to read file");
    // let scores1 = HashMap::from([
    //     ("A Y", 8),
    //     ("A X", 4),
    //     ("A Z", 3),
    //     ("B X", 1),
    //     ("B Y", 5),
    //     ("B Z", 9),
    //     ("C X", 7),
    //     ("C Y", 2),
    //     ("C Z", 6),
    // ]);

    // let collection1 = data.split_terminator("\r\n")
    // .map(|n| {scores1.get(n).unwrap().clone()})
    // .fold(0, |acc, x| acc + x);

    // let scores2 = HashMap::from([
    //     ("A Z", 8),
    //     ("A Y", 4),
    //     ("A X", 3),
    //     ("B X", 1),
    //     ("B Y", 5),
    //     ("B Z", 9),
    //     ("C Z", 7),
    //     ("C X", 2),
    //     ("C Y", 6),
    // ]);

    // let collection2 = data.split_terminator("\r\n")
    // .map(|n| {scores2.get(n).unwrap().clone()})
    // .fold(0, |acc, x| acc + x);

    // println!("collection1: {:?}, collection2: {:?}", collection1,collection2) ;

    //--- Day 3: Rucksack Reorganization ---
    //
    // let priority_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").collect::<Vec<&str>>();
    // let data = fs::read_to_string("3.txt").expect("Unable to read file");
    // let collection1 = data.split_terminator("\r\n").map(|n| {
    //     let (half_1, half_2) = n.split_at(n.len()/2);
    //     let s_half_2 = half_2.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
    //     let s_half_1 = half_1.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
    //     let same_val = s_half_1.iter().find(|x| s_half_2.contains(*x)).unwrap();
    //     *same_val
    //     // .fold(0, |acc, x| acc + x)
    // }).map(|n| {
    //     priority_map.iter().position(|&x| x == n).unwrap()
    // }).collect::<Vec<usize>>();

    // let collection2 = data.split("\r\n").collect::<Vec<&str>>();
    // let third_chunks: Vec<_> = collection2
    //     .chunks(3)
    //     .collect();
    // let filtered_third_chunks:Vec<_> = third_chunks.iter().map(|n| {
    //     let first = n[0].split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
    //     let second = n[1].split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
    //     let third = n[2].split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
    //     let same_val1: Vec<_> = first.iter().filter(|x| second.contains(*x)).collect();
    //     same_val1.iter().find(|x| third.contains(*x)).unwrap().to_string()
    // }).collect();

    // let final_priority = filtered_third_chunks.iter().map(|n| {
    //     priority_map.iter().position(|&x| x == n).unwrap()
    // }).fold(0, |acc, x| acc + x);

    // println!("1:{:?}",collection1.iter().fold(0, |acc, x| acc + x));
    // println!("2:{:?}", final_priority);

    //--- Day 4: Camp Cleanup ---
    //
    // let data = fs::read_to_string("4.txt").expect("Unable to read file");
    // let collection1 = data.split_terminator("\r\n").map(|n| {
    //     let a = n.split(",").map(|b| {
    //         b.split("-")
    //         .map(|n| { n.parse::<i32>().unwrap()})
    //         .collect::<Vec<i32>>()
    //     }).collect::<Vec<Vec<i32>>>();
    //     a
    // }).collect::<Vec<Vec<Vec<i32>>>>();
    //
    // let overlapped = collection1.iter()
    //     .filter(|&x| {
    //         (x[0][0] <= x[1][0] && x[0][1] >= x[1][1]) ||
    //         (x[1][0] <= x[0][0] && x[1][1] >= x[0][1])
    //     })
    //     .collect::<Vec<_>>();
    //     // .enumerate()
    //     // .map(|(i , x)| {
    //     //     println!("{:?}-{:?}, {:?}-{:?} {:?} v",x[0][0],x[0][1],x[1][0],x[1][1], i)
    //     // })
    //
    // let overlapping = collection1.iter()
    // .filter(|&x| {
    //     (x[0][0] <= x[1][0] && x[0][1] >= x[1][1]) ||
    //     (x[1][0] <= x[0][0] && x[1][1] >= x[0][1]) ||
    //     (x[0][0] <= x[1][0] && x[0][1] >= x[1][0]) ||
    //     (x[1][0] <= x[0][0] && x[1][1] >= x[0][0])
    // })
    // .enumerate()
    // .map(|(i , x)| {
    //     println!("{:?}-{:?}, {:?}-{:?} {:?} v",x[0][0],x[0][1],x[1][0],x[1][1], i)
    // })
    // .collect::<Vec<_>>().len();
    // println!("1:{:?}, 2:{:?}", overlapped.len(), overlapping);

    // --- Day 5: Supply Stacks ---

    // [Q] [J]                         [H]
    // [G] [S] [Q]     [Z]             [P]
    // [P] [F] [M]     [F]     [F]     [S]
    // [R] [R] [P] [F] [V]     [D]     [L]
    // [L] [W] [W] [D] [W] [S] [V]     [G]
    // [C] [H] [H] [T] [D] [L] [M] [B] [B]
    // [T] [Q] [B] [S] [L] [C] [B] [J] [N]
    // [F] [N] [F] [V] [Q] [Z] [Z] [T] [Q]
    //  1   2   3   4   5   6   7   8   9

    //    // let data = fs::read_to_string("5.txt").expect("Unable to read file");
    // let data = fs::read_to_string("5.txt").expect("Unable to read file");
    // let collection1 = data
    //     .split_terminator("\n")
    //     .map(|n| {
    //         let t = n.split_terminator(" ").collect::<Vec<&str>>();
    //         vec![
    //             t[1].parse::<usize>().unwrap(),
    //             t[3].parse::<usize>().unwrap(),
    //             t[5].parse::<usize>().unwrap(),
    //         ]
    //     })
    //     .collect::<Vec<Vec<usize>>>();
    //
    // let stack1 = vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'];
    // let stack2 = vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'];
    // let stack3 = vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'];
    // let stack4 = vec!['V', 'S', 'T', 'D', 'F'];
    // let stack5 = vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'];
    // let stack6 = vec!['Z', 'C', 'L', 'S'];
    // let stack7 = vec!['Z', 'B', 'M', 'V', 'D', 'F'];
    // let stack8 = vec!['T', 'J', 'B'];
    // let stack9 = vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'];
    // // let mut stacks = vec![
    // //     stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9,
    // // ];
    // let mut stacks = HashMap::from([
    //     (1, stack1),
    //     (2, stack2),
    //     (3, stack3),
    //     (4, stack4),
    //     (5, stack5),
    //     (6, stack6),
    //     (7, stack7),
    //     (8, stack8),
    //     (9, stack9),
    // ]);
    //
    // // for (_, val) in stacks.iter_mut() {
    // //     val.reverse()
    // // }
    // // stacks.iter_mut().for_each(|n| n.reverse());
    //
    // let tops = collection1.iter().for_each(|n| {
    //     let amount = n[0];
    //     // let from = stacks.get_many_mut([&(n[1] - 1), &(n[2] - 1)]);
    //     let from = n[1];
    //     let to = n[2];
    //
    //     let mut gg = stacks.get(&from).unwrap().clone();
    //     gg.reverse();
    //     let (left, right) = gg.split_at_mut(amount);
    //     right.reverse();
    //     stacks.insert(from, right.to_vec());
    //     let tos = stacks.get_mut(&to).unwrap();
    //     // left.reverse(); // uncomment for part2
    //     tos.extend_from_slice(left);
    //     // println!("1:{:?},2:{:?},3:{:?}", left, right, tos);
    // });
    //
    // let mut answer: Vec<_> = stacks.iter().collect();
    //
    // answer.sort_by(|a, b| a.0.cmp(b.0));
    //
    // println!(
    //     "{:?}",
    //     answer
    //         .iter()
    //         .map(|(_, val)| val.last().unwrap())
    //         .fold(String::from(""), |acc, a| [acc, a.to_string()].concat()),
    // );

    // --- Day 6: Tuning Trouble ---
    let data = fs::read_to_string("6.txt").expect("Unable to read file");
    let mut current = VecDeque::new();
    let mut counter = 0;

    for c in data.chars() {
        current.push_back(c);
        counter = counter + 1;
        // if current.len() == 4 { // q1
        if current.len() == 14 {
            if test_code(&current) {
                println!("1:{:?}, c: {:?}", current, counter);
                break;
            } else {
                current.pop_front();
            }
        }
    }
    // let collection1 = data
    //     .split_terminator("\n")
    //     .map(|n| {
    //         let t = n.split_terminator(" ").collect::<Vec<&str>>();
    //         vec![
    //             t[1].parse::<usize>().unwrap(),
    //             t[3].parse::<usize>().unwrap(),
    //             t[5].parse::<usize>().unwrap(),
    //         ]
    //     })
    //     .collect::<Vec<Vec<usize>>>();
    //
    // println!("1:{:?}, 2:", data);
    Ok(())
}

fn test_code(v:&VecDeque<char>) -> bool {
    let mut code = BTreeSet::new();
    for n in v.iter() {
        if code.insert(n) {
            continue;
        } else {
            return false;
        }
    }
    true
}
