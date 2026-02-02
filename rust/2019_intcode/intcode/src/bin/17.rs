use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::Instant;
use std::{fs, process};

const INPUT_PATH: &str = "./input/17/input";
const SHOW_OUTPUT: bool = false;

use intcode::icc::{ICC, State};
use itertools::Itertools;

#[allow(unused)]
fn pprint(map: &HashMap<(isize, isize), char>) {
    if !SHOW_OUTPUT {
        return;
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    for (y, x) in map.keys() {
        min_x = min(min_x, *x);
        max_x = max(max_x, *x);
        min_y = min(min_y, *y);
        max_y = max(max_y, *y);
    }
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            print!("{}", map.get(&(y, x)).unwrap())
        }
        println!();
    }
}

fn part(input: String, part1: bool) -> isize {
    let mut icc = ICC::new(&input);
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let (mut y, mut x) = (0, 0);
    while let State::Output(val) = icc.run() {
        if val == 10 {
            y += 1;
            x = 0;
            continue;
        }
        map.insert((y, x), (val as u8) as char);
        x += 1;
    }
    if part1 {
        pprint(&map);
    }
    let (intersections, i_path) = pathify(&map);
    let turn2char = HashMap::from([(1, 'L'), (-1, 'R')]);
    let mut path: Vec<(char, usize)> = i_path
        .iter()
        .map(|(turn, len)| (*turn2char.get(turn).unwrap(), *len))
        .collect();
    if part1 {
        return intersections;
    }
    let a = vec![('R', 12), ('L', 10), ('R', 12)];
    let b = vec![('L', 8), ('R', 10), ('R', 6)];
    let c = vec![('R', 12), ('L', 10), ('R', 10), ('L', 8)];
    let abc = vec![('A', a), ('B', b), ('C', c)];
    for (c, pat) in &abc[..3] {
        let mut i = 0;
        'outer: loop {
            for j in 0..pat.len() {
                if i + j >= path.len() {
                    break 'outer;
                }
                if path[i + j] != pat[j] {
                    i += 1;
                    continue 'outer;
                }
            }
            for _j in 0..pat.len() {
                path.remove(i);
            }
            path.insert(i, (*c, 0));
        }
        // println!("path: {:?}", path);
        // println!("path.len(): {:?}", path.len());
    }
    let mut icc = ICC::new(&input);
    icc.memory[0] = 2;
    let mut config: String = String::new();
    config += &path.iter().map(|(mf, _)| mf).join(",");
    config.push('\n');

    for (_, pat) in abc {
        config += &pat
            .iter()
            .map(|(turn, len)| turn.to_string() + "," + &len.to_string())
            .join(",");
        config.push('\n');
    }
    config.extend(['n', '\n']);
    icc.input_queue.extend(config.chars().map(|c| c as isize));
    while let State::Output(val) = icc.run() {
        if val > 1000 {
            return val;
        }
        if SHOW_OUTPUT {
            print!("{}", (val as u8) as char);
        }
    }
    return 0; //error case
}

fn pathify(map: &HashMap<(isize, isize), char>) -> (isize, Vec<(isize, usize)>) {
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)]; //north,west,south,east
    let mut dir: usize = 0;
    let (mut pos_y, mut pos_x) = (0, 0);
    for ((y, x), val) in map {
        if (*val as u8) as char == '^' {
            pos_y = *y;
            pos_x = *x;
        }
    }
    let mut path: Vec<(isize, usize)> = Vec::new();
    let mut intersections = 0;
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    'outer: loop {
        let mut path_elem = (0, 0);
        //check left and right for scaffolding
        for turn in [1, -1] {
            // dirs is directions in counterclockwise order, thus: dir+1 is "turn left", dir-1 is "turn right"
            let test_dir = (dir as isize + turn).rem_euclid(4) as usize;
            let delta = dirs[test_dir];
            let (y, x) = (pos_y + delta.0, pos_x + delta.1);
            if let Some(val) = map.get(&(y, x)) {
                if *val == '#' {
                    dir = test_dir;
                    path_elem.0 = turn;
                    break;
                }
            }
        }
        //if no scaffolding left or right => finished
        if path_elem.0 == 0 {
            return (intersections, path);
        }
        //walk straight as far as possible
        loop {
            let next_y = pos_y + dirs[dir].0;
            let next_x = pos_x + dirs[dir].1;
            match map.get(&(next_y, next_x)) {
                None | Some('.') => {
                    path.push(path_elem);
                    continue 'outer;
                }
                Some('#') => {
                    path_elem.1 += 1;
                    pos_y = next_y;
                    pos_x = next_x;
                    //see a tile twice=>intersection
                    if !seen.insert((pos_y, pos_x)) {
                        intersections += pos_y * pos_x;
                    }
                }
                _ => panic!("unexpected map"),
            }
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string(INPUT_PATH)?;
    let p1 = part(input1, true);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string(INPUT_PATH)?;
    let p2 = part(input2, false);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
