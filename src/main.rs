use std::{fs, collections::HashMap};

use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

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
    let data = fs::read_to_string("4.txt").expect("Unable to read file");
    let collection1 = data.split_terminator("\r\n").map(|n| {
        let a = n.split(",").map(|b| {
            b.split("-")
            .map(|n| { n.parse::<i32>().unwrap()})
            .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();
        a
    }).collect::<Vec<Vec<Vec<i32>>>>();

    let overlapped = collection1.iter()
        .filter(|&x| {
            (x[0][0] <= x[1][0] && x[0][1] >= x[1][1]) ||
            (x[1][0] <= x[0][0] && x[1][1] >= x[0][1])
        })
        .collect::<Vec<_>>();
        // .enumerate()
        // .map(|(i , x)| {
        //     println!("{:?}-{:?}, {:?}-{:?} {:?} v",x[0][0],x[0][1],x[1][0],x[1][1], i)
        // })

    let overlapping = collection1.iter()
    .filter(|&x| {
        (x[0][0] <= x[1][0] && x[0][1] >= x[1][1]) ||
        (x[1][0] <= x[0][0] && x[1][1] >= x[0][1]) ||
        (x[0][0] <= x[1][0] && x[0][1] >= x[1][0]) ||
        (x[1][0] <= x[0][0] && x[1][1] >= x[0][0])
    })
    .enumerate()
    .map(|(i , x)| {
        println!("{:?}-{:?}, {:?}-{:?} {:?} v",x[0][0],x[0][1],x[1][0],x[1][1], i)
    })
    .collect::<Vec<_>>().len();
    println!("1:{:?}, 2:{:?}", overlapped.len(), overlapping);

    Ok(())
}