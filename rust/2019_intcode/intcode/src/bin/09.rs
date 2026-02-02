use std::error::Error;
use std::time::Instant;
use std::{fs, process};

const INPUT_PATH: &str = "./input/09/input";

use intcode::icc::{ICC, State};

fn part(input: String, part1: bool) -> isize {
    let id = if part1 { 1 } else { 2 };

    let mut icc = ICC::new(&input);
    icc.input_queue.push_back(id);
    let mut last_val = 0;
    loop {
        match icc.run() {
            State::Output(val) => last_val = val,
            State::Halted => return last_val,
            _ => panic!("no output"),
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

#[cfg(test)]
mod tests {

    use super::*;
    //const TINPUT_PATH: &str = "./input/tinput";

    #[test]
    fn p2_9() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let mut icc = ICC::new(&input);

        let res = match icc.run_until_halted_or_waiting() {
            intcode::icc::MState::Halted(s) => s,
            intcode::icc::MState::Waiting(s) => s,
        };

        println!("res : {:?}", res);
        assert_eq!(input, res);
    }
    #[test]
    fn p2_91() {
        let input = "1102,34915192,34915192,7,4,7,99,0".to_string();
        let mut icc = ICC::new(&input);

        let res = match icc.run_until_halted_or_waiting() {
            intcode::icc::MState::Halted(s) => s,
            intcode::icc::MState::Waiting(s) => s,
        };

        println!("res : {:?}", res);
        assert_eq!("1219070632396864", res);
    }
    #[test]
    fn p2_92() {
        let input = "104,1125899906842624,99".to_string();
        let mut icc = ICC::new(&input);

        let res = match icc.run_until_halted_or_waiting() {
            intcode::icc::MState::Halted(s) => s,
            intcode::icc::MState::Waiting(s) => s,
        };

        println!("res : {:?}", res);
        assert_eq!("1125899906842624", res);
    }
}
