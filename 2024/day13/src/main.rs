use std::fs;

#[derive(Debug)]
struct Machine {
    a: (f32, f32),
    b: (f32, f32),
    prize: (f32,f32)
}

fn main() {
    let data = fs::read_to_string("13.txt").expect("Unable to read file");
    let machines = data.split_terminator("\n\n")
        .map(|x| {
            let line = x.split_terminator("\n").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let a_line = line[0].replace("Button A: X+", "").replace(" Y+", "").split(",").map(|x| x.parse().unwrap()).collect::<Vec<f32>>();
            let a = (a_line[0], a_line[1]);
            let b_line = line[1].replace("Button B: X+", "").replace(" Y+", "").split(",").map(|x| x.parse().unwrap()).collect::<Vec<f32>>();
            let b = (b_line[0], b_line[1]);
            let prize_line = line[2].replace("Prize: X=", "").replace(" Y=", "").split(",").map(|x| x.parse().unwrap()).collect::<Vec<f32>>();
            let prize = (prize_line[0], prize_line[1]);
            Machine {
                a, b, prize
            }
        })
        .collect::<Vec<Machine>>();
    let minimum_token = machines.iter().map(|machine|{
        let x = (machine.a.0, machine.b.0);
        let y = (machine.a.1, machine.b.1);
        solve(x,y,machine.prize)
    })
        // .filter(|x| x.0 > 0.0 && x.0 <= 100.0 && x.1 > 0.0 && x.1 <= 100.0)
        .map(|x| x.0 * 3.0 + x.1)
        // .collect::<Vec<f32>>();
        .sum::<f32>();
    println!("part1:{:?}", minimum_token);
    let part_2_minimum_token = machines.iter().map(|machine|{
        let x = (machine.a.0 as f64, machine.b.0 as f64);
        let y = (machine.a.1 as f64, machine.b.1 as f64);
        solve_f64(x,y,(machine.prize.0 as f64 + 10000000000000.0, machine.prize.1 as f64 + 10000000000000.0))
    })
        .filter(|x| x.0 > 0.0 && x.1 > 0.0)
        .map(|x| x.0 * 3.0 + x.1)
        // .collect::<Vec<f64>>();
        .sum::<f64>();
    println!("part2:{:?}", part_2_minimum_token );
}

fn solve(x: (f32,f32), y: (f32,f32), prize: (f32,f32)) -> (f32,f32) {
    let determinant = x.0 * y.1 - y.0 * x.1;
    let adj =[[y.1, -x.1], [-y.0, x.0]];
    let inv = [[adj[0][0] * 1.0/determinant, adj[0][1] * 1.0/determinant],[adj[1][0] * 1.0/determinant, adj[1][1] * 1.0/determinant]];
    let ans = (inv[0][0]*prize.0 + inv[0][1] * prize.1, inv[1][0]*prize.0 + inv[1][1] * prize.1);
    // println!("{:?}",ans);
    // println!("{} {}", ans.0 * x.0 + ans.1 *x.1, prize.0);
    let rounded_ans = (((ans.0 * 100.0).round() / 100.0), ((ans.1 * 100.0).round() / 100.0));
    if rounded_ans.0.fract() == 0.0 && rounded_ans.1.fract() == 0.0 {
        rounded_ans
    } else {
        (-1.0,-1.0)
    }
}

fn solve_f64(x: (f64,f64), y: (f64,f64), prize: (f64,f64)) -> (f64,f64) {
    let determinant = x.0 * y.1 - y.0 * x.1;
    let adj =[[y.1, -x.1], [-y.0, x.0]];
    let inv = [[adj[0][0] * 1.0/determinant, adj[0][1] * 1.0/determinant],[adj[1][0] * 1.0/determinant, adj[1][1] * 1.0/determinant]];
    let ans = (inv[0][0]*prize.0 + inv[0][1] * prize.1, inv[1][0]*prize.0 + inv[1][1] * prize.1);
    let rounded_ans = (((ans.0 * 100.0).round() / 100.0), ((ans.1 * 100.0).round() / 100.0));
    // println!("{} {}", rounded_ans.0 * x.0 + rounded_ans.1 *x.1, prize.0);
    if rounded_ans.0.fract() == 0.0 && rounded_ans.1.fract() == 0.0 {
        rounded_ans
    } else {
        (-1.0,-1.0)
    }
}
