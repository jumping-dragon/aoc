use colored::Colorize;
use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("4.txt").expect("Unable to read file");
    let re = Regex::new(r"XMAS").unwrap();
    let re_inv = Regex::new(r"SAMX").unwrap();

    let horizontal_lines = data.split_terminator("\n").collect::<Vec<&str>>();
    let mut horizontal_results: Vec<&str> = vec![];
    for line in horizontal_lines.iter() {
        let results: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let inv_results: Vec<&str> = re_inv.find_iter(line).map(|m| m.as_str()).collect();
        horizontal_results.extend(results);
        horizontal_results.extend(inv_results);
    }
    // println!("{:?} {}", horizontal_results, horizontal_results.len());

    let hori_len = horizontal_lines[0].len();

    let mut iters: Vec<_> = horizontal_lines
        .clone()
        .into_iter()
        .map(|n| n.split("").filter(|x| !x.is_empty()))
        .collect();
    let vertical_lines: Vec<Vec<&str>> = (0..hori_len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<&str>>()
        })
        .collect();
    let vertical_lines_as_string: Vec<String> =
        vertical_lines.clone().iter().map(|x| x.join("")).collect();
    let mut vertical_results: Vec<&str> = vec![];
    for line in vertical_lines_as_string.iter() {
        let vec = line.as_str();
        let results: Vec<&str> = re.find_iter(vec).map(|m| m.as_str()).collect();
        let inv_results: Vec<&str> = re_inv.find_iter(line).map(|m| m.as_str()).collect();
        vertical_results.extend(results);
        vertical_results.extend(inv_results);
    }
    // println!("{:?} {}", vertical_results, vertical_results.len());

    let hori_lines_matrix = horizontal_lines
        .iter()
        .map(|x| x.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut diagonal_lines_matrix: Vec<Vec<&str>> = vec![];
    (0..(hori_len - 1) * 2).for_each(|a| {
        diagonal_lines_matrix.push(vec![]);
        (0..hori_len).for_each(|y| {
            for x in 0..hori_len {
                if x + y == a {
                    diagonal_lines_matrix[a].push(hori_lines_matrix[y][x])
                }
            }
        });
    });
    let diagonal_lines_as_string: Vec<String> = diagonal_lines_matrix
        .clone()
        .iter()
        .map(|x| x.join(""))
        .collect();
    let mut diagonal_results: Vec<&str> = vec![];
    for line in diagonal_lines_as_string.iter() {
        let vec = line.as_str();
        let results: Vec<&str> = re.find_iter(vec).map(|m| m.as_str()).collect();
        let inv_results: Vec<&str> = re_inv.find_iter(line).map(|m| m.as_str()).collect();
        diagonal_results.extend(results);
        diagonal_results.extend(inv_results);
    }
    // println!("{:?} {}", diagonal_results, diagonal_results.len());
    //
    let mut iv_diagonal_lines_matrix: Vec<Vec<&str>> = vec![];
    (0..(hori_len - 1) * 2).for_each(|a| {
        iv_diagonal_lines_matrix.push(vec![]);
        (0..hori_len).for_each(|y| {
            for x in 0..hori_len {
                if x + y == a {
                    iv_diagonal_lines_matrix[a].push(hori_lines_matrix[y][hori_len - 1 - x])
                }
            }
        });
    });
    let iv_diagonal_lines_as_string: Vec<String> = iv_diagonal_lines_matrix
        .clone()
        .iter()
        .map(|x| x.join(""))
        .collect();
    let mut iv_diagonal_results: Vec<&str> = vec![];
    for line in iv_diagonal_lines_as_string.iter() {
        let vec = line.as_str();
        let results: Vec<&str> = re.find_iter(vec).map(|m| m.as_str()).collect();
        let inv_results: Vec<&str> = re_inv.find_iter(line).map(|m| m.as_str()).collect();
        iv_diagonal_results.extend(results);
        iv_diagonal_results.extend(inv_results);
    }
    // println!("{:?} {}", iv_diagonal_results, iv_diagonal_results.len());
    println!(
        "part1: {}",
        horizontal_results.len()
            + vertical_results.len()
            + diagonal_results.len()
            + iv_diagonal_results.len()
    );

    let mut counter: i32 = 0;
    for (y, hori_line_matrix) in hori_lines_matrix.iter().enumerate() {
        for (x, a) in hori_line_matrix.iter().enumerate() {
            if hori_lines_matrix[y][x] == "X" {
                // print!("{}", hori_lines_matrix[y][x].black());
            } else if !(x == 0
                || y == 0
                || x == (hori_line_matrix.len() - 1)
                || y == (hori_lines_matrix.len() - 1))
            {
                if hori_lines_matrix[y][x] == "A" {
                    if (hori_lines_matrix[y - 1][x - 1] == "M"
                        && hori_lines_matrix[y - 1][x + 1] == "M"
                        && hori_lines_matrix[y + 1][x - 1] == "S"
                        && hori_lines_matrix[y + 1][x + 1] == "S")
                        || (hori_lines_matrix[y - 1][x - 1] == "S"
                            && hori_lines_matrix[y - 1][x + 1] == "S"
                            && hori_lines_matrix[y + 1][x - 1] == "M"
                            && hori_lines_matrix[y + 1][x + 1] == "M")
                        || (hori_lines_matrix[y - 1][x - 1] == "S"
                            && hori_lines_matrix[y - 1][x + 1] == "M"
                            && hori_lines_matrix[y + 1][x - 1] == "S"
                            && hori_lines_matrix[y + 1][x + 1] == "M")
                        || (hori_lines_matrix[y - 1][x - 1] == "M"
                            && hori_lines_matrix[y - 1][x + 1] == "S"
                            && hori_lines_matrix[y + 1][x - 1] == "M"
                            && hori_lines_matrix[y + 1][x + 1] == "S")
                    {
                        // print!("{}", (hori_lines_matrix[y][x]).yellow());
                        counter += 1;
                    } else {
                        // print!("{}", (hori_lines_matrix[y][x]).red());
                    }
                } else {
                    // print!("{}", (hori_lines_matrix[y][x]).white());
                }
            } else {
                // print!("{}", hori_lines_matrix[y][x]);
            }
        }
        // print!("\n");
    }
    println!("part2: {}", counter);
}
