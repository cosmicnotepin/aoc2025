use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./input/input";

struct ICC {
    memory: Vec<isize>,
    ip: usize,
}

impl ICC {
    fn run(&mut self) {
        loop {
            match self.memory[self.ip] {
                99 => {
                    return;
                }
                1 => {
                    let oprnd1 = self.memory[self.memory[self.ip + 1] as usize];
                    let oprnd2 = self.memory[self.memory[self.ip + 2] as usize];
                    let dest_i = self.memory[self.ip + 3] as usize;
                    self.memory[dest_i] = oprnd1 + oprnd2;
                }
                2 => {
                    let oprnd1 = self.memory[self.memory[self.ip + 1] as usize];
                    let oprnd2 = self.memory[self.memory[self.ip + 2] as usize];
                    let dest_i = self.memory[self.ip + 3] as usize;
                    self.memory[dest_i] = oprnd1 * oprnd2;
                }
                opc => panic!("unexpected opcode: {opc}"),
            }
            self.ip += 4;
        }
    }
}

fn part(input: String, part1: bool) -> isize {
    let program: Vec<isize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    return if part1 {
        let mut icc = ICC {
            memory: program,
            ip: 0,
        };
        icc.memory[1] = 12;
        icc.memory[2] = 2;
        icc.run();
        return icc.memory[0];
    } else {
        for i1 in 0..100 {
            for i2 in 0..100 {
                let mut icc = ICC {
                    memory: program.clone(),
                    ip: 0,
                };
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
        let mut icc = ICC {
            memory: program,
            ip: 0,
        };
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
        let mut icc = ICC {
            memory: program,
            ip: 0,
        };
        icc.run();
        assert_eq!(3500, icc.memory[0]);
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(0, part(input, false));
    }
}
