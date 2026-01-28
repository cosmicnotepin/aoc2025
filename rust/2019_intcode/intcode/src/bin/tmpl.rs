use std::error::Error;
use std::time::Instant;
use std::{fs, process, thread};

const INPUT_PATH: &str = "./input/09/input";

use intcode::icc::ICC;

fn part(input: String, part1: bool) -> isize {
    let id = if part1 { 1 } else { 2 };

    let (mut icc, to_icc, from_icc) = ICC::new(input);
    thread::spawn(move || icc.run());
    let _ = to_icc.send(id);
    let mut last_val = 0;
    for val in from_icc {
        last_val = val;
    }
    return last_val;
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
        let input = "1,1,1,4,99,5,6,0,99".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(30, icc.memory[0]);
    }

    #[test]
    fn p1_2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(3500, icc.memory[0]);
    }

    #[test]
    fn p1_3() {
        let input = "1002,4,3,4,33".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(99, icc.memory[4]);
    }

    #[test]
    fn p2_2() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let (mut icc, to_icc, from_icc) = ICC::new(input);
        let _ = to_icc.send(4);

        icc.run();
        assert_eq!(999, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_3() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let (mut icc, to_icc, from_icc) = ICC::new(input);
        let _ = to_icc.send(190);

        icc.run();
        assert_eq!(1001, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_9() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!(input, res);
    }
    #[test]
    fn p2_91() {
        let input = "1102,34915192,34915192,7,4,7,99,0".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!("1219070632396864", res);
    }
    #[test]
    fn p2_92() {
        let input = "104,1125899906842624,99".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!("1125899906842624", res);
    }
}
