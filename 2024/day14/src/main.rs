use std::{collections::BTreeMap, fs, thread, time::Duration};

#[derive(Debug, Clone)]
struct Bot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Bot {
    fn iterate(&mut self, max_x: i32, max_y: i32) {
        let mut new_position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        if new_position.0 < 0 {
            new_position.0 += max_x;
        } else if new_position.0 >= max_x {
            new_position.0 -= max_x;
        }
        if new_position.1 < 0 {
            new_position.1 += max_y;
        } else if new_position.1 >= max_y {
            new_position.1 -= max_y;
        }
        self.position = new_position;
    }
}

fn main() {
    let data = fs::read_to_string("14.txt").expect("Unable to read file");
    let bots = data
        .split_terminator("\r\n")
        .map(|x| {
            let parts = x.split_whitespace().collect::<Vec<&str>>();
            let position = parts[0]
                .replace("p=", "")
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let velocity = parts[1]
                .replace("v=", "")
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            Bot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect::<Vec<Bot>>();
    let max_x = 101;
    let max_y = 103;
    let mid_x = (max_x - 1) / 2;
    let mid_y = (max_y - 1) / 2;

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    let mut bots_iter1 = bots.clone();
    let mut bots_iter2 = bots;
    (0..101).for_each(|a| {
        (0..max_y).for_each(|y| {
            (0..max_x).for_each(|x| {
                let mut bot_amount = 0;
                for bot in bots_iter1.iter() {
                    if bot.position == (x, y) {
                        bot_amount += 1;
                        if a == 100 {
                            if x < mid_x {
                                if y < mid_y {
                                    tl += 1;
                                } else if y > mid_y {
                                    bl += 1;
                                }
                            } else if x > mid_x {
                                if y < mid_y {
                                    tr += 1;
                                } else if y > mid_y {
                                    br += 1;
                                }
                            }
                        }
                    }
                }
                // if bot_amount == 0 {
                //     print!(".");
                // } else {
                //     print!("{}", bot_amount);
                // }
            });
            // println!();
        });
        for bot in bots_iter1.iter_mut() {
            // println!("{:?}", bot);
            bot.iterate(max_x, max_y)
        }
    });
    println!("tl {}", tl);
    println!("tr {}", tr);
    println!("bl {}", bl);
    println!("br {}", br);
    println!("part1 {}", tl * tr * bl * br);

    let mut ms: BTreeMap<(i32, i32, i32, i32), i32> = BTreeMap::new();
    for y in 0..max_x * max_y + 1 {
        // thread::sleep(Duration::from_millis(200));
        let mut x_score = [0; 101];
        let mut y_score = [0; 103];
        (0..max_y).for_each(|y| {
            (0..max_x).for_each(|x| {
                let mut bot_amount = 0;
                for bot in bots_iter2.iter() {
                    if bot.position == (x, y) {
                        bot_amount += 1;
                        y_score[y as usize] += 1;
                        x_score[x as usize] += 1;
                        if x < mid_x {
                            tl += mid_x - x;
                        } else if x > mid_x {
                            tr += x - mid_x;
                        }
                    }
                }
                // if bot_amount == 0 {
                //     print!(".");
                // } else {
                //     print!("{}", bot_amount);
                // }
            });
            // println!();
        });
        // println!("y {}", y);
        if y_score.iter().any(|x| x > &30) && x_score.iter().any(|x| x > &30) {
            (0..max_x).for_each(|y| {
                (0..max_x).for_each(|x| {
                    let mut bot_amount = 0;
                    for bot in bots_iter2.iter() {
                        if bot.position == (x, y) {
                            bot_amount += 1;
                            if x < mid_x {
                                tl += mid_x - x;
                            } else if x > mid_x {
                                tr += x - mid_x;
                            }
                        }
                    }
                    if bot_amount == 0 {
                        print!(".");
                    } else {
                        print!("{}", bot_amount);
                    }
                });
                println!();
            });
            println!("part2: {}", y);
            break;
        } else {
            println!("y {}", y);
            print!("{esc}c", esc = 27 as char);
        }
        for bot in bots_iter2.iter_mut() {
            // println!("{:?}", bot);
            bot.iterate(max_x, max_y)
        }
    }
}
