use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{fs, process};

use intcode::icc::{ICC, State};
const INPUT_PATH: &str = "./input/13/input";
const SHOW_OUTPUT: bool = false;

fn part(input: String, part1: bool) -> isize {
    if part1 {
        let mut icc = ICC::new(&input);
        let mut p1_res = Vec::new();
        while let State::Output(val) = icc.run() {
            p1_res.push(val);
        }
        return p1_res
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|&a| *a == 2)
            .count() as isize;
    } else {
        let mut icc = ICC::new(&input);
        icc.memory[0] = 2;
        let mut map: HashMap<(isize, isize), isize> = HashMap::new();
        let mut paddle_x = 0;
        let mut ball_x = 0;
        loop {
            match icc.run() {
                State::Halted => {
                    pprint(&map);
                    break;
                }
                State::Output(x) => {
                    let y = icc.run_until_first_output();
                    let tile_id = icc.run_until_first_output();
                    map.insert((y, x), tile_id);
                    match tile_id {
                        3 => paddle_x = x,
                        4 => ball_x = x,
                        _ => (),
                    }
                }
                State::Waiting => {
                    icc.input_queue.push_back((ball_x - paddle_x).signum());
                }
            }
        }
        return *map.get(&(0, -1)).unwrap();
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
            match map.get(&(y, x)) {
                Some(0) => print!(" "),
                Some(1) => print!("#"),
                Some(2) => print!("+"),
                Some(3) => print!("-"),
                Some(4) => print!("o"),
                Some(_) => panic!("what is this?"),
                None => print!(" "),
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
