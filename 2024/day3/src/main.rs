use std::fs;

fn main() {
    let data = fs::read_to_string("3.txt").expect("Unable to read file");
    let re = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let results: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();
    // println!("{:?}", results);
    let total = results.iter().map(|x| extract_mul(x)).sum::<i32>();
    println!("part1: {:?}", total);
    let re = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();
    let results: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();
    let mut enabled = true;
    let mut total = 0;
    for r in results {
        match r {
            "don't()" => {
                enabled = false;
            }
            "do()" => {
                enabled = true;
            }
            _ => {
                if enabled {
                    total += extract_mul(r)
                }
            }
        }
    }
    println!("part2: {:?}", total);
}

fn extract_mul(mul: &str) -> i32 {
    let nums = mul
        .replace("mul(", "")
        .replace(")", "")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    nums[0] * nums[1]
}
