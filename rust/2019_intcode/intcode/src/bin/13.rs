use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use std::{fs, process, thread};

use intcode::icc::ICC;
const INPUT_PATH: &str = "./input/13/input";
const SHOW_OUTPUT: bool = false;

fn part(input: String, part1: bool) -> isize {
    if part1 {
        let (mut icc, _to_icc, from_icc) = ICC::new(input);
        thread::spawn(move || icc.run());
        let mut p1_res = 0;
        for (_x, _y, tile_id) in from_icc.iter().tuples() {
            if tile_id == 2 {
                p1_res += 1;
            }
        }
        return p1_res;
    } else {
        let (mut icc, to_icc, from_icc) = ICC::new(input);
        icc.memory[0] = 2;
        let jh = thread::spawn(move || icc.run());
        let mut map: HashMap<(isize, isize), isize> = HashMap::new();
        let mut paddle_x = 0;
        let mut ball_x;

        while let Ok(val) = from_icc.recv_timeout(Duration::from_millis(1)) {
            let x = val;
            let y = from_icc.recv().unwrap();
            let tile_id = from_icc.recv().unwrap();
            map.insert((y, x), tile_id);
            if tile_id == 3 {
                paddle_x = x;
            }
        }
        let _ = to_icc.send(1);

        loop {
            if jh.is_finished() {
                pprint(&map);
                return *map.get(&(0, -1)).unwrap();
            }
            while let Ok(val) = from_icc.recv_timeout(Duration::from_millis(1)) {
                let x = val;
                let y = from_icc.recv().unwrap();
                let tile_id = from_icc.recv().unwrap();
                map.insert((y, x), tile_id);
                if tile_id == 3 {
                    paddle_x = x;
                }
                if tile_id == 4 {
                    pprint(&map);
                    ball_x = x;
                    let _ = to_icc.send((ball_x - paddle_x).signum());
                }
            }
        }
    }
}

fn pprint(map: &HashMap<(isize, isize), isize>) {
    if !SHOW_OUTPUT {
        return;
    }
    let (mut max_y, mut max_x) = (0, 0);
    for (y, x) in map.keys() {
        max_y = max(max_y, *y);
        max_x = max(max_x, *x);
    }
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            match map.get(&(y, x)).unwrap() {
                0 => print!(" "),
                1 => print!("#"),
                2 => print!("+"),
                3 => print!("-"),
                4 => print!("o"),
                _ => panic!("what is this??"),
            }
        }
        println!();
    }
    if let Some(score) = map.get(&(0, -1)) {
        println!("score: {}", score);
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

#[cfg(test)]
mod tests {}
