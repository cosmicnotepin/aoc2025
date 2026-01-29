use std::{
    iter,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct ICC {
    pub memory: Vec<isize>,
    ip: usize,
    tx: Sender<isize>,
    rx: Receiver<isize>,
    rel_base: usize,
}

impl ICC {
    pub fn new(program_string: String) -> (ICC, Sender<isize>, Receiver<isize>) {
        let mut program: Vec<isize> = program_string
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        program.extend(iter::repeat_n(0, 1000));
        let (to_icc, rx) = mpsc::channel::<isize>();
        let (tx, from_icc) = mpsc::channel::<isize>();
        (
            ICC {
                memory: program,
                ip: 0,
                tx,
                rx,
                rel_base: 0,
            },
            to_icc,
            from_icc,
        )
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
    pub fn run(&mut self) {
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
                    match self.rx.recv() {
                        Ok(input) => {
                            self.memory[dest_i] = input;
                            self.ip += 2;
                        }
                        Err(_) => return,
                    }
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
