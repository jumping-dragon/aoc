use std::fs;
fn main() {
    let data = fs::read_to_string("1.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|n| {
            n.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut lefts = lines
        .clone()
        .into_iter()
        .map(|n| n[0])
        .collect::<Vec<i32>>();
    lefts.sort();
    let mut rights = lines.into_iter().map(|n| n[1]).collect::<Vec<i32>>();
    rights.sort();
    let mut distances: Vec<i32> = vec![];
    for (id, num) in lefts.iter().enumerate() {
        if num > &rights[id] {
            distances.push(num - rights[id])
        } else {
            distances.push(rights[id] - num)
        }
    }
    println!("{:?}", distances.into_iter().sum::<i32>());
    let mut similarity_scores: Vec<i32> = vec![];
    for num in lefts.iter() {
        let right_occurences = rights.iter().filter(|&n| *n == *num).count() as i32;
        similarity_scores.push(num * right_occurences)
    }
    println!("{:?}", similarity_scores.into_iter().sum::<i32>());
}
