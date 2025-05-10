use std::{collections::BTreeMap, fs};

fn main() {
    let data = fs::read_to_string("17.txt").expect("Unable to read file");
    let parts = data.split_terminator("\r\n\r\n").collect::<Vec<&str>>();
    let registers = parts[0].split_terminator("\r\n").collect::<Vec<&str>>();
    let mut registers_map: BTreeMap<&str, u64> = BTreeMap::new();
    for reg in registers.iter() {
        let part = reg.split_whitespace().collect::<Vec<&str>>();
        let k = part[1].strip_suffix(":").unwrap();
        let v = part[2].parse::<u64>().unwrap();
        registers_map.insert(k, v);
    }
    println!("{:?}", registers_map);

    let binding = parts[1].trim().strip_prefix("Program: ").unwrap();
    let programs = binding
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let output = run(61657405, programs.clone());
    print!("part1: ");
    println!("{:?}", output);

    let mut nu = 8_u64.pow(15);
    let mut bit: usize = 0;
    loop {
        let test = run(nu, programs.clone());

        if bit <= 15 {
            if test[15 - bit] == programs[15 - bit] {
                // println!(
                //     "{} {:?} {} {:?} {} {} {} {}",
                //     nu,
                //     test,
                //     test.len(),
                //     programs,
                //     programs.len(),
                //     bit,
                //     test[15 - bit],
                //     programs[15 - bit]
                // );
                bit += 1;
            } else {
                let mut iterator = 8_u64.pow(15 - bit as u32);
                nu += iterator;
            }
        } else {
            // println!(
            //     "{} {:?} {} {:?} {}  {}",
            //     nu,
            //     test,
            //     test.len(),
            //     programs,
            //     programs.len(),
            //     bit
            // );
            nu += 1;
            if test == programs {
                break;
            }
        }
    }
    println!("part2: {}", nu);
}

fn run(a: u64, programs: Vec<u64>) -> Vec<u64> {
    let mut registers_map: BTreeMap<&str, u64> = BTreeMap::new();
    registers_map.insert("A", a);
    registers_map.insert("B", 0);
    registers_map.insert("C", 0);

    // println!("{:?}", registers_map);
    let mut ip = 0;
    let mut output: Vec<u64> = vec![];

    loop {
        let opcode = programs[ip];
        let operand = programs[ip + 1];
        // println!("{}) {:?} {} {:?}", ip, opcode, operand, registers_map);
        registers_map = match opcode {
            0 => dv(registers_map, operand, "A"),
            1 => bxl(registers_map, operand),
            2 => bst(registers_map, operand),
            3 => registers_map,
            4 => bxc(registers_map, operand),
            5 => registers_map,
            6 => dv(registers_map, operand, "B"),
            7 => dv(registers_map, operand, "C"),
            _ => todo!(),
        };
        if opcode == 3 {
            let a = registers_map.get_mut("A").unwrap();
            if *a != 0 {
                ip = operand as usize;
            } else {
                ip += 2;
            }
        } else if opcode == 5 {
            if operand > 3 {
                match operand {
                    4 => {
                        if let Some(a) = registers_map.get("A") {
                            output.push(a % 8);
                        }
                    }
                    5 => {
                        if let Some(b) = registers_map.get("B") {
                            output.push(b % 8);
                        }
                    }
                    6 => {
                        if let Some(c) = registers_map.get("C") {
                            output.push(c % 8);
                        }
                    }
                    _ => todo!(),
                }
            } else {
                output.push(operand % 8);
            }
            // println!("{:?} {}", output, operand);
            ip += 2;
        } else {
            ip += 2;
        }
        if ip >= programs.len() {
            break;
        }
    }
    output
}

fn dv<'a>(
    registers_map: BTreeMap<&'a str, u64>,
    operand: u64,
    reg: &'a str,
) -> BTreeMap<&'a str, u64> {
    let mut new_reg_map = registers_map.clone();
    if let Some(r) = new_reg_map.get_mut(reg) {
        if let Some(a) = registers_map.get("A") {
            if operand > 3 {
                match operand {
                    4 => {
                        *r = a / 2_u64.pow((*a).try_into().unwrap());
                    }
                    5 => {
                        if let Some(b) = registers_map.get("B") {
                            *r = a / 2_u64.pow((*b).try_into().unwrap());
                        }
                    }
                    6 => {
                        if let Some(c) = registers_map.get("C") {
                            *r = a / 2_u64.pow((*c).try_into().unwrap());
                        }
                    }
                    _ => todo!(),
                }
            } else {
                *r = a / 2_u64.pow(operand.try_into().unwrap());
            }
        }
    }
    new_reg_map
}

fn bxl(registers_map: BTreeMap<&str, u64>, operand: u64) -> BTreeMap<&str, u64> {
    let mut new_reg_map = registers_map.clone();
    if let Some(b) = new_reg_map.get_mut("B") {
        *b ^= operand;
    }
    new_reg_map
}

fn bst(registers_map: BTreeMap<&str, u64>, operand: u64) -> BTreeMap<&str, u64> {
    let mut new_reg_map = registers_map.clone();
    if let Some(b) = new_reg_map.get_mut("B") {
        if operand > 3 {
            match operand {
                4 => {
                    if let Some(a) = registers_map.get("A") {
                        *b = a % 8;
                    }
                }
                5 => {
                    *b %= 8;
                }
                6 => {
                    if let Some(c) = registers_map.get("C") {
                        *b = c % 8;
                    }
                }
                _ => todo!(),
            }
        } else {
            *b = operand % 8;
        }
    }
    new_reg_map
}

fn bxc(registers_map: BTreeMap<&str, u64>, operand: u64) -> BTreeMap<&str, u64> {
    let mut new_reg_map = registers_map.clone();
    if let Some(b) = new_reg_map.get_mut("B") {
        if let Some(c) = registers_map.get("C") {
            *b ^= *c;
        }
    }
    new_reg_map
}
