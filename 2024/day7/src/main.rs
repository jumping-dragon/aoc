use std::fs;

#[derive(Debug)]
struct Line(u64, Vec<u64>);

fn main() {
    let data = fs::read_to_string("7.txt").expect("Unable to read file");
    let lines = data
        .split_terminator("\n")
        .map(|x| {
            let lines = x.split_once(": ").unwrap();
            let test_num = lines.0.parse().unwrap();
            let nums = lines.1.split(" ").map(|y| y.parse().unwrap()).collect();
            Line(test_num, nums)
        })
        .collect::<Vec<Line>>();

    let mut valid_total = 0;
    for line in lines.iter() {
        let len = line.1.len() as u32;
        let bit: u32 = len - 1;
        let max_of_bit = 2_u64.pow(bit);
        for num in 0..max_of_bit {
            let symbols: Vec<u64> = (0..bit).rev().map(|n| (num >> n) & 1).collect();
            let total = get_total(&symbols, &line.1);
            if line.0 == total {
                // println!("{} {:?} {:?}", total, line.1, line.0);
                valid_total += line.0;
                break;
            }
        }
    }
    println!("part1: {:?}", valid_total);

    let mut valid_total = 0;
    for line in lines.iter() {
        let len = line.1.len() as u32;
        let bit: u32 = len - 1;
        let max_of_bit = 3_u64.pow(bit);
        for num in 0..max_of_bit {
            let symbols = convert_u64_to_ternary_vector(num, bit as usize);
            let total = get_total(&symbols, &line.1);
            if line.0 == total {
                // println!("{} {:?} {:?} {:?}", total, line.1, line.0, symbols);
                valid_total += line.0;
                break;
            }
        }
    }
    println!("part2: {:?}", valid_total);
}

fn get_total(symbols: &[u64], nums: &[u64]) -> u64 {
    let mut total = 0;
    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            total += num;
        } else if symbols[i - 1] == 2 {
            let total_zeros = num.ilog10() + 1;
            total = total * 10_u64.pow(total_zeros) + num;
        } else if symbols[i - 1] == 1 {
            total *= num;
        } else {
            total += num;
        }
    }
    total
}

fn convert_u64_to_ternary_vector(num: u64, base: usize) -> Vec<u64> {
    if num == 0 {
        vec![0; base]
    } else {
        let mut result = Vec::new();

        let mut temp_num = num;
        while temp_num > 0 {
            result.push(temp_num % 3);
            temp_num /= 3;
        }

        while result.len() < base {
            result.push(0);
        }

        // Reverse the vector since we built it from least to most significant digit
        result.reverse();
        result
    }
}
