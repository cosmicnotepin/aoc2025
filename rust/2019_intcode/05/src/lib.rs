use std::error::Error;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;
use std::{fs, thread};

const INPUT_PATH: &str = "./input/input";

struct ICC {
    memory: Vec<isize>,
    ip: usize,
    tx: Sender<isize>,
    rx: Receiver<isize>,
}

impl ICC {
    fn new(program: Vec<isize>, tx: Sender<isize>, rx: Receiver<isize>) -> ICC {
        ICC {
            memory: program,
            ip: 0,
            tx,
            rx,
        }
    }
    fn mem(&self, index: usize, immediate: usize) -> isize {
        let mut res = self.memory[index];
        if immediate == 0 {
            res = self.memory[res as usize];
        }
        return res;
    }
    fn decode_opc(&self) -> [usize; 3] {
        let mut res = [0, 0, 0];
        let mut opc = self.memory[self.ip] as usize;
        res[0] = opc % 100;
        opc = opc / 100;
        for i in 1..3 {
            res[i] = opc % 10;
            opc = opc / 10;
        }
        return res;
    }
    fn run(&mut self) {
        loop {
            let [op, m1, m2] = self.decode_opc();
            match op {
                99 => {
                    return;
                }
                1 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    let dest_i = self.memory[self.ip + 3] as usize;
                    self.memory[dest_i] = oprnd1 + oprnd2;
                    self.ip += 4;
                }
                2 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    let dest_i = self.memory[self.ip + 3] as usize;
                    self.memory[dest_i] = oprnd1 * oprnd2;
                    self.ip += 4;
                }
                3 => {
                    let dest_i = self.memory[self.ip + 1] as usize;
                    let input = self.rx.recv().unwrap();
                    self.memory[dest_i] = input;
                    self.ip += 2;
                }
                4 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let _ = self.tx.send(oprnd1);
                    self.ip += 2;
                }
                5 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    if oprnd1 != 0 {
                        self.ip = oprnd2 as usize;
                    } else {
                        self.ip += 3
                    }
                }
                6 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    if oprnd1 == 0 {
                        self.ip = oprnd2 as usize;
                    } else {
                        self.ip += 3
                    }
                }
                7 => {
                    let dest_i = self.memory[self.ip + 3] as usize;
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    self.memory[dest_i] = if oprnd1 < oprnd2 { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    let dest_i = self.memory[self.ip + 3] as usize;
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    self.memory[dest_i] = if oprnd1 == oprnd2 { 1 } else { 0 };
                    self.ip += 4;
                }

                opc => panic!("unexpected opcode: {opc}"),
            }
        }
    }
}

fn part(input: String, part1: bool) -> isize {
    let program: Vec<isize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let id = if part1 { 1 } else { 5 };

    let (to_icc, rx) = mpsc::channel::<isize>();
    let (tx, from_icc) = mpsc::channel::<isize>();
    let mut icc = ICC::new(program, tx, rx);
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

#[cfg(test)]
mod tests {
    use super::*;
    const TINPUT_PATH: &str = "./input/tinput";

    #[test]
    fn p1_1() {
        let input = "1,1,1,4,99,5,6,0,99".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, _from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

        icc.run();
        assert_eq!(30, icc.memory[0]);
    }

    #[test]
    fn p1_2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, _from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

        icc.run();
        assert_eq!(3500, icc.memory[0]);
    }

    #[test]
    fn p1_3() {
        let input = "1002,4,3,4,33".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, _from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

        icc.run();
        assert_eq!(99, icc.memory[4]);
    }

    #[test]
    fn p2_2() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);
        let _ = to_icc.send(4);

        icc.run();
        assert_eq!(999, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_3() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);
        let _ = to_icc.send(190);

        icc.run();
        assert_eq!(1001, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(0, part(input, false));
    }
}
