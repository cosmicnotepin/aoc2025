use std::error::Error;
use std::fs;
use std::time::Instant;

use num_integer::Integer;
// 9223372036854775807
//     101741582076661 shuffles
//     119315717514047 cards

const INPUT_PATH: &str = "./input/input";

fn shffl(i: isize, a: isize, b: isize, m: isize) -> isize {
    ((a as u128 * i as u128).rem_euclid(m as u128) as isize + b).rem_euclid(m)
}

fn inv_mod(nr: isize, modulus: isize) -> isize {
    nr.extended_gcd(&modulus).x.rem_euclid(modulus)
}

fn part(input: String, part1: bool, m: isize) -> isize {
    let mut a: isize = 1;
    let mut b: isize = 0;
    for instr_s in input.lines().rev() {
        // println!("instr_s: {:?}", instr_s);
        match instr_s.chars().nth(0).unwrap() {
            'd' => match instr_s.chars().nth(5).unwrap() {
                'w' => {
                    let val: isize = instr_s[20..].parse().unwrap();
                    let big_val = inv_mod(val, m) as u128;
                    b = (b as u128 * big_val).rem_euclid(m as u128) as isize;
                    a = (a as u128 * big_val).rem_euclid(m as u128) as isize;
                }
                'i' => {
                    a = (a * -1).rem_euclid(m);
                    b = (-b + m - 1).rem_euclid(m);
                }
                _ => (),
            },
            'c' => {
                let val: isize = instr_s[4..].parse().unwrap();
                b = (b + val).rem_euclid(m);
            }
            _ => (),
        };
    }

    return if part1 {
        let mut ret = 0;
        for i in 0..m {
            if shffl(i, a, b, m) == 2019 {
                ret = i;
                break;
            }
        }
        ret
    } else {
        // ^n -> a^n i + b(a^n-1 + a^n-2 + ... + a^0)
        // a^n * i + b ((1-a^n)/(1-a)) => division modulo m => MULTIPLICATIVE INVERSE!
        let a_n = mod_exp::mod_exp(a as u128, 101741582076661 as u128, m as u128) as isize;
        let first_term = (a_n as u128 * 2020 as u128).rem_euclid(m as u128) as isize;
        let brkt = ((1 - a_n).rem_euclid(m) as u128 * inv_mod((1 - a).rem_euclid(m), m) as u128)
            .rem_euclid(m as u128);
        let second_term = (b as u128 * brkt).rem_euclid(m as u128) as isize;
        (first_term + second_term).rem_euclid(m)
    };
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string(INPUT_PATH)?;
    let p1 = part(input1, true, 10007);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string(INPUT_PATH)?;
    let p2 = part(input2, false, 119315717514047);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TINPUT_PATH: &str = "./input/tinput";

    #[test]
    fn p1_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(-1, part(input, true, 10));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(0, part(input, false, 10));
    }
}
