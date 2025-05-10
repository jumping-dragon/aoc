use std::{collections::HashSet, fs};
fn main() {
    let data = fs::read_to_string("2.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|n| {
            n.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut safe_rep = 0;
    let mut unsafe_reps: Vec<Vec<i32>> = vec![];
    for nums in lines.iter() {
        if is_safe(nums) == 0 {
            // println!("unsafe {:?} {}", nums, safe_rep);
            unsafe_reps.push(nums.to_vec())
        } else {
            safe_rep += 1
        }
    }
    println!("part 1:{:?}", safe_rep);
    // println!("{:?}", unsafe_reps);
    for nums in unsafe_reps.iter() {
        for (id, _num) in nums.iter().enumerate() {
            let mut nums_dampened = nums.to_vec();
            nums_dampened.remove(id);
            if is_safe(&nums_dampened) == 1 {
                safe_rep += 1;
                // println!("safe {:?} {}", nums, id);
                break;
            }
        }
    }
    println!("part 2:{:?}", safe_rep);
}

fn is_safe(nums: &[i32]) -> i32 {
    if nums
        .windows(2)
        .all(|w| (w[0] - w[1]) > 0 && (w[0] - w[1]) <= 3)
    {
        // println!("decreasing {:?} {}", nums, safe_rep);
        1
    } else if nums
        .windows(2)
        .all(|w| (w[1] - w[0]) > 0 && (w[1] - w[0]) <= 3)
    {
        // println!("increasing {:?} {}", nums, safe_rep);
        1
    } else {
        // println!("unsafe {:?} {}", nums, safe_rep);
        0
    }
}
