use std::error::Error;
use std::time::Instant;
use std::{fs, process};

const INPUT_PATH: &str = "./input/02/input";

use intcode::icc::ICC;

fn part(input: String, part1: bool) -> isize {
    return if part1 {
        let mut icc = ICC::new(&input);
        icc.memory[1] = 12;
        icc.memory[2] = 2;
        icc.run();
        return icc.memory[0];
    } else {
        for i1 in 0..100 {
            for i2 in 0..100 {
                let mut icc = ICC::new(&input);
                icc.memory[1] = i1;
                icc.memory[2] = i2;
                icc.run();
                if icc.memory[0] == 19690720 {
                    return i1 * 100 + i2;
                }
            }
        }
        input.len().try_into().unwrap()
    };
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

    #[test]
    fn p1_1() {
        let input = "1,1,1,4,99,5,6,0,99".to_string();
        let mut icc = ICC::new(&input);
        icc.run();
        assert_eq!(30, icc.memory[0]);
    }

    #[test]
    fn p1_2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        let mut icc = ICC::new(&input);
        icc.run();
        assert_eq!(3500, icc.memory[0]);
    }
}
