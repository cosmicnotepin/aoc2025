use std::{collections::VecDeque, iter};

const ADD: usize = 1;
const MUL: usize = 2;
const IN: usize = 3;
const OUT: usize = 4;
const JNZ: usize = 5;
const JIZ: usize = 6;
const LT: usize = 7;
const EQ: usize = 8;
const SRB: usize = 9;
const HLT: usize = 99;

pub enum State {
    Halted,
    Output(isize),
    Waiting,
}

pub enum MState {
    Halted(String),
    Waiting(String),
}

pub struct ICC {
    pub memory: Vec<isize>,
    ip: usize,
    pub input_queue: VecDeque<isize>,
    rel_base: usize,
}

impl ICC {
    pub fn run_until_first_output(&mut self) -> isize {
        match self.run() {
            State::Output(val) => return val,
            _ => panic!("no output"),
        }
    }

    pub fn run_until_halted_or_waiting(&mut self) -> MState {
        let mut output: Vec<String> = Vec::new();

        loop {
            match self.run() {
                State::Output(val) => output.push(val.to_string()),
                State::Halted => return MState::Halted(output.join(",")),
                State::Waiting => return MState::Waiting(output.join(",")),
            }
        }
    }
    pub fn new(program_string: &str) -> ICC {
        let mut program: Vec<isize> = program_string
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        program.extend(iter::repeat_n(0, 10000));
        ICC {
            memory: program,
            ip: 0,
            input_queue: VecDeque::new(),
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
    fn decode_inst(&mut self) -> (usize, isize, isize, usize) {
        let [op, m_a, m_b, m_c] = self.decode_opc();
        let (mut a, mut b, mut c) = (0, 0, 0);
        match op {
            ADD | MUL | LT | EQ => {
                a = self.mem(self.ip + 1, m_a);
                b = self.mem(self.ip + 2, m_b);
                c = self.mem_dest_i(self.ip + 3, m_c);
                self.ip += 4;
            }
            JNZ | JIZ => {
                a = self.mem(self.ip + 1, m_a);
                b = self.mem(self.ip + 2, m_b);
                self.ip += 3;
            }
            SRB | OUT => {
                a = self.mem(self.ip + 1, m_a);
                self.ip += 2;
            }
            IN => {
                a = self.mem_dest_i(self.ip + 1, m_a) as isize;
            }
            HLT => (),
            _ => panic!("unexpected op in decode_inst"),
        }
        (op, a, b, c)
    }
    fn set(&mut self, dest_i: usize, val: isize) {
        self.memory[dest_i] = val;
    }
    pub fn run(&mut self) -> State {
        loop {
            let (op, a, b, c) = self.decode_inst();
            match op {
                HLT => return State::Halted,

                ADD => self.set(c, a + b),

                MUL => self.set(c, a * b),

                IN => match self.input_queue.pop_front() {
                    Some(input) => {
                        self.set(a as usize, input);
                        self.ip += 2
                    }
                    None => return State::Waiting,
                },
                OUT => return State::Output(a),
                JNZ => {
                    if a != 0 {
                        self.ip = b as usize;
                    }
                }
                JIZ => {
                    if a == 0 {
                        self.ip = b as usize;
                    }
                }
                LT => self.set(c, if a < b { 1 } else { 0 }),
                EQ => self.set(c, if a == b { 1 } else { 0 }),
                SRB => self.rel_base = (self.rel_base as isize + a) as usize,
                opc => panic!("unexpected opcode: {opc}"),
            }
        }
    }
}
