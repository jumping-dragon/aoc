use std::{collections::{BTreeMap, BTreeSet}, fs, str::FromStr, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Ops {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl FromStr for Ops {
    type Err = ();
    fn from_str(input: &str) -> Result<Ops, Self::Err> {
        match input {
            "^"  => Ok(Ops::UP),
            "v"  => Ok(Ops::DOWN),
            "<"  => Ok(Ops::LEFT),
            ">"  => Ok(Ops::RIGHT),
            _      => Err(()),
        }
    }
}
fn main() {
    let data = fs::read_to_string("15.txt").expect("Unable to read file");
    let block = data.split_terminator("\n\n").collect::<Vec<&str>>();
    let lines = block[0].split_terminator("\n").map(|x| x.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let ops_lines = block[1].split_terminator("\n").map(|x| x.split("").filter(|x| !x.is_empty()).map(|x| Ops::from_str(x).unwrap()).collect::<Vec<Ops>>()).collect::<Vec<Vec<Ops>>>();

    let mut map: BTreeMap<&str, BTreeSet<(usize,usize)>> = BTreeMap::new();

    for (y,line) in lines.iter().enumerate(){
        for (x, l) in line.iter().enumerate(){
            if let Some(set) = map.get_mut(l) {
                set.insert((y,x));
            } else {
                map.insert(l, BTreeSet::from([(y,x)]));
            }
        }
    }

    // for (k,v) in map.iter(){
    //     println!("{}: {:?}", k,v);
    // }

    for (y,ops_line) in ops_lines.iter().enumerate(){
        for op in ops_line.iter(){
            // println!("{} {:?}", y,op);
            let new_map = run_op(map, *op);
            map = new_map;
        }
    }

    let mut sum_of_gps = 0;
    let boxes = map.get("O").unwrap();
    for boxx in boxes.iter() {
        sum_of_gps += 100 * boxx.0 + boxx.1;
    }
    println!("part1: {}", sum_of_gps);


    let part2_lines = lines.clone().iter().map(|line|
        line.iter().flat_map(|l| match *l {
            "#" => ["#","#"],
            "O" => ["[","]"],
            "." => [".","."],
            "@" => ["@","."],
            _ => todo!()
        }).collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    let mut part2_map: BTreeMap<&str, BTreeSet<(usize,usize)>> = BTreeMap::new();

    for (y,line) in part2_lines.iter().enumerate(){
        for (x, l) in line.iter().enumerate(){
            if let Some(set) = part2_map.get_mut(l) {
                set.insert((y,x));
            } else {
                part2_map.insert(l, BTreeSet::from([(y,x)]));
            }
        }
    }


    for ops_line in ops_lines.iter(){
        for op in ops_line.iter(){
            // for (y,line) in part2_lines.iter().enumerate(){
            //     for (x, l) in line.iter().enumerate(){
            //         let left_boxes = part2_map.get("[").unwrap();
            //         let right_boxes = part2_map.get("]").unwrap();
            //         let walls = part2_map.get("#").unwrap();
            //         let bot = part2_map.get("@").unwrap();
            //         if left_boxes.contains(&(y,x)){
            //             print!("[");
            //         } else if right_boxes.contains(&(y,x)){
            //             print!("]");
            //         } else if walls.contains(&(y,x)){
            //             print!("#");
            //         } else if bot.contains(&(y,x)){
            //             print!("@");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!()
            // }
            let new_map = run_part2_op(part2_map, *op);
            part2_map = new_map;
        }
    }
    // for (k,v) in part2_map.iter(){
    //     println!("{}: {:?}", k,v);
    // }

    let mut sum_of_gps = 0;
    for left_box in part2_map.get("[").unwrap().iter(){
        sum_of_gps += 100 * left_box.0 + left_box.1;
    }
    println!("part2: {}", sum_of_gps);
}

fn run_op(map: BTreeMap<&str, BTreeSet<(usize,usize)>>, op: Ops) -> BTreeMap<&str, BTreeSet<(usize,usize)>> {
    let mut new_map = map.clone();
    if let Some(bot_set) = map.get("@"){
        let old_bot_pos = bot_set.first().unwrap();
        let new_bot_pos = get_new_pos(old_bot_pos, op);
        if check_op_ok(&map, &new_bot_pos){
            let (boxes, new_boxes, ok) = get_all_boxes(&map, &new_bot_pos, op);
            // println!("{:?} {:?} {}", boxes, new_boxes, ok);
            if ok {
                new_map.remove("@");
                new_map.insert("@", BTreeSet::from([new_bot_pos]));
            }
            if boxes.is_empty(){
                return new_map;
            } else {
                if let Some(box_set) = new_map.get_mut("O"){
                    for old_box in boxes.iter(){
                        box_set.remove(old_box);
                    }
                    for new_box in new_boxes.iter(){
                        box_set.insert(*new_box);
                    }
                }
                return new_map;
            }
        } else {
            return new_map;
        }
    }
    new_map
}

fn run_part2_op(map: BTreeMap<&str, BTreeSet<(usize,usize)>>, op: Ops) -> BTreeMap<&str, BTreeSet<(usize,usize)>> {
    let mut new_map = map.clone();
    if let Some(bot_set) = map.get("@"){
        let old_bot_pos = bot_set.first().unwrap();
        let new_bot_pos = get_new_pos(old_bot_pos, op);
        if check_op_ok(&map, &new_bot_pos){
            let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(&map, &new_bot_pos, op);
            // println!("{:?} {:?} {}", new_left_boxes, new_right_boxes, ok);
            if ok {
                // println!("{:?}", op);
                new_map.remove("@");
                new_map.insert("@", BTreeSet::from([new_bot_pos]));
                if !left_boxes.is_empty() {
                    if let Some(box_set) = new_map.get_mut("["){
                        for old_box in left_boxes.iter(){
                            box_set.remove(old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            box_set.insert(*new_box);
                        }
                    }
                }
                if !right_boxes.is_empty() {
                    if let Some(box_set) = new_map.get_mut("]"){
                        for old_box in right_boxes.iter(){
                            box_set.remove(old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            box_set.insert(*new_box);
                        }
                    }
                }
                return new_map;
            } else {
                return new_map;
            }
        } else {
            return new_map;
        }
    }
    new_map
}

fn check_op_ok(map: &BTreeMap<&str, BTreeSet<(usize,usize)>>, new_bot_pos: &(usize,usize)) -> bool {
    let walls = map.get("#").unwrap();
    !walls.contains(new_bot_pos)
}

fn get_all_boxes(map: &BTreeMap<&str, BTreeSet<(usize,usize)>>, new_bot_pos: &(usize,usize), op: Ops) -> (Vec<(usize, usize)>,Vec<(usize, usize)>, bool) {
    let mut pushed_boxes: Vec<(usize, usize)> = vec![];
    let mut new_pushed_boxes: Vec<(usize, usize)> = vec![];
    let boxes = map.get("O").unwrap();
    if boxes.contains(new_bot_pos) {
        let old_box_pos = new_bot_pos;
        let new_box_pos = get_new_pos(old_box_pos, op);
        if check_op_ok(map, &new_box_pos){
            pushed_boxes.push(*old_box_pos);
            new_pushed_boxes.push(new_box_pos);
            if boxes.contains(&new_box_pos){
                let (boxes, new_boxes, ok) = get_all_boxes(map, &new_box_pos, op);
                // println!("{:?} {:?} {:?}", new_bot_pos, boxes, new_boxes);
                if boxes.is_empty() {
                    return (vec![], vec![], ok);
                } else {
                    for old_box in boxes.iter() {
                        pushed_boxes.push(*old_box);
                    }
                    for new_box in new_boxes.iter(){
                        new_pushed_boxes.push(*new_box);
                    }
                }
            } else {
                return (pushed_boxes, new_pushed_boxes, true);
            }
        } else {
            return (vec![], vec![], false);
        }
    }
    (pushed_boxes, new_pushed_boxes, true)
}

fn get_all_boxes_wide(map: &BTreeMap<&str, BTreeSet<(usize,usize)>>, new_bot_pos: &(usize,usize), op: Ops) -> (
Vec<(usize, usize)>,
Vec<(usize, usize)>,
Vec<(usize, usize)>,
Vec<(usize, usize)>,
bool
) {
    let mut pushed_left_boxes: Vec<(usize, usize)> = vec![];
    let mut pushed_right_boxes: Vec<(usize, usize)> = vec![];
    let mut new_pushed_left_boxes: Vec<(usize, usize)> = vec![];
    let mut new_pushed_right_boxes: Vec<(usize, usize)> = vec![];
    let left_boxes = map.get("[").unwrap();
    let right_boxes = map.get("]").unwrap();
    if left_boxes.contains(new_bot_pos) || right_boxes.contains(new_bot_pos) {
        let old_box_pos = new_bot_pos;
        let (old_left, new_left_box_pos, new_right_box_pos, old_right) = if left_boxes.contains(new_bot_pos) {
            let left = get_new_pos(old_box_pos, op);
            let old_right = (old_box_pos.0, old_box_pos.1 + 1);
            let right = get_new_pos(&old_right, op);
            (*old_box_pos, left, right, old_right)
        } else {
            let old_left = (old_box_pos.0, old_box_pos.1 - 1);
            let left = get_new_pos(&old_left, op);
            let right = get_new_pos(old_box_pos, op);
            (old_left, left, right,*old_box_pos)
        };
        if !check_op_ok(map, &new_right_box_pos) || !check_op_ok(map, &new_left_box_pos){
            return (vec![], vec![], vec![], vec![], false);
        } else {
            pushed_left_boxes.push(old_left);
            new_pushed_left_boxes.push(new_left_box_pos);
            pushed_right_boxes.push(old_right);
            new_pushed_right_boxes.push(new_right_box_pos);
            if op == Ops::LEFT {
                if right_boxes.contains(&new_left_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_left_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
            }
            if op == Ops::RIGHT {
                if left_boxes.contains(&new_right_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_right_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
            }
            if op == Ops::UP || op == Ops::DOWN {
                if left_boxes.contains(&new_left_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_left_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
                if left_boxes.contains(&new_right_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_right_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
                if right_boxes.contains(&new_right_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_right_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
                if right_boxes.contains(&new_left_box_pos){
                    let (left_boxes, new_left_boxes, right_boxes, new_right_boxes, ok) = get_all_boxes_wide(map, &new_left_box_pos, op);
                    if left_boxes.is_empty() && right_boxes.is_empty() {
                        return (vec![], vec![], vec![], vec![], ok);
                    } else {
                        for old_box in left_boxes.iter() {
                            pushed_left_boxes.push(*old_box);
                        }
                        for new_box in new_left_boxes.iter(){
                            new_pushed_left_boxes.push(*new_box);
                        }
                        for old_box in right_boxes.iter() {
                            pushed_right_boxes.push(*old_box);
                        }
                        for new_box in new_right_boxes.iter(){
                            new_pushed_right_boxes.push(*new_box);
                        }
                    }
                }
            }
        }
    }
    // println!("pushed_left_boxes: {:?}, new_pushed_left_boxes{:?}, pushed_right_boxes{:?}, new_pushed_right_boxes{:?}",pushed_left_boxes, new_pushed_left_boxes, pushed_right_boxes, new_pushed_right_boxes);
    (pushed_left_boxes, new_pushed_left_boxes, pushed_right_boxes, new_pushed_right_boxes, true)
}

fn get_new_pos(old_pos: &(usize, usize), op: Ops) ->(usize,usize){
        match op {
            Ops::UP => (old_pos.0 - 1, old_pos.1),
            Ops::DOWN => (old_pos.0 + 1, old_pos.1),
            Ops::LEFT => (old_pos.0, old_pos.1 - 1),
            Ops::RIGHT => (old_pos.0, old_pos.1 + 1),
        }
}
