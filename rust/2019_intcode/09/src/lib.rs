use std::error::Error;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;
use std::{fs, iter, thread};

const INPUT_PATH: &str = "./input/input";

struct ICC {
    memory: Vec<isize>,
    ip: usize,
    tx: Sender<isize>,
    rx: Receiver<isize>,
    rel_base: usize,
}

impl ICC {
    fn new(mut program: Vec<isize>, tx: Sender<isize>, rx: Receiver<isize>) -> ICC {
        program.extend(iter::repeat_n(0, 1000));
        ICC {
            memory: program,
            ip: 0,
            tx,
            rx,
            rel_base: 0,
        }
    }
    fn mem(&self, index: usize, mode: usize) -> isize {
        let mut res = self.memory[index];
        if mode == 0 {
            res = self.memory[res as usize];
        }
        if mode == 2 {
            res = self.memory[(res + self.rel_base as isize) as usize];
        }
        return res;
    }
    fn mem_dest_i(&self, index: usize, mode: usize) -> usize {
        if mode == 1 {
            panic!("Denkfehler?");
        }
        let mut res = self.memory[index];
        if mode == 2 {
            res += self.rel_base as isize;
        }
        return res as usize;
    }
    fn decode_opc(&self) -> [usize; 4] {
        let mut res = [0, 0, 0, 0];
        let mut opc = self.memory[self.ip] as usize;
        res[0] = opc % 100;
        opc = opc / 100;
        for i in 1..4 {
            res[i] = opc % 10;
            opc = opc / 10;
        }
        return res;
    }
    fn run(&mut self) {
        loop {
            let [op, m1, m2, m3] = self.decode_opc();
            match op {
                99 => {
                    return;
                }
                1 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    let dest_i = self.mem_dest_i(self.ip + 3, m3);
                    self.memory[dest_i] = oprnd1 + oprnd2;
                    self.ip += 4;
                }
                2 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    let dest_i = self.mem_dest_i(self.ip + 3, m3);
                    self.memory[dest_i] = oprnd1 * oprnd2;
                    self.ip += 4;
                }
                3 => {
                    let dest_i = self.mem_dest_i(self.ip + 1, m1);
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
                    let dest_i = self.mem_dest_i(self.ip + 3, m3);
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    self.memory[dest_i] = if oprnd1 < oprnd2 { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    let dest_i = self.mem_dest_i(self.ip + 3, m3);
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    let oprnd2 = self.mem(self.ip + 2, m2);
                    self.memory[dest_i] = if oprnd1 == oprnd2 { 1 } else { 0 };
                    self.ip += 4;
                }
                9 => {
                    let oprnd1 = self.mem(self.ip + 1, m1);
                    self.rel_base = (self.rel_base as isize + oprnd1) as usize;
                    self.ip += 2;
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

    let id = if part1 { 1 } else { 2 };

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
    fn p2_9() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

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
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

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
        let program: Vec<isize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (_to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        let mut icc = ICC::new(program, tx, rx);

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
