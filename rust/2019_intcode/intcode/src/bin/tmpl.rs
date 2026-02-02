use std::error::Error;
use std::time::Instant;
use std::{fs, process};

const INPUT_PATH: &str = "./input/09/input";

use intcode::icc::ICC;

fn part(input: String, part1: bool) -> isize {
    let id = if part1 { 1 } else { 2 };

    let mut icc = ICC::new(&input);
    icc.input_queue.push_back(id);
    return 0;
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
mod tests {
    use super::*;
    //const INPUT_PATH: &str = "./input/09/tinput";

    #[test]
    fn p1_1() {
        assert_eq!(0, part("".to_string(), true));
    }
}
