use std::error::Error;
use std::fs;
use std::time::Instant;

use num_integer::Integer;
//101741582076661 shuffles
//119315717514047 cards

const INPUT_PATH: &str = "./input/input";

fn compose(
    f: Box<dyn Fn(isize) -> isize>,
    g: Box<dyn Fn(isize) -> isize>,
) -> Box<dyn Fn(isize) -> isize> {
    Box::new(move |i| g(f(i)))
}

fn inv_mod(nr: isize, modulus: isize) -> isize {
    nr.extended_gcd(&modulus).x
}

fn cut(i: isize, c: isize, m: isize) -> isize {
    (i + c).rem_euclid(m)
}

fn dwi(i: isize, incr: isize, m: isize) -> isize {
    let ui = i as u128;
    let mut im = inv_mod(incr, m);
    while im < 0 {
        im += m;
    }
    let uim = im as u128;
    let ures = ((ui * uim).rem_euclid(m as u128)) as isize;
    ures
    // (i * inv_mod(incr, m)).rem_euclid(m)
}

fn dins(i: isize, m: isize) -> isize {
    (m - 1 - i).rem_euclid(m)
}

fn part(input: String, part1: bool, m: isize) -> isize {
    let mut shuff_res: Box<dyn Fn(isize) -> isize> = Box::new(move |i: isize| i);
    for instr_s in input.lines() {
        // println!("instr_s: {:?}", instr_s);
        let to_comp: Box<dyn Fn(isize) -> isize> = match instr_s.chars().nth(0).unwrap() {
            'd' => match instr_s.chars().nth(5).unwrap() {
                'w' => {
                    let val: isize = instr_s[20..].parse().unwrap();
                    Box::new(move |i| dwi(i, val, m))
                }
                'i' => Box::new(move |i| dins(i, m)),
                _ => Box::new(move |i| i),
            },
            'c' => {
                let val: isize = instr_s[4..].parse().unwrap();
                Box::new(move |i| cut(i, val, m))
            }
            _ => Box::new(move |i| i),
        };
        shuff_res = compose(to_comp, shuff_res);
    }
    // for ui in 0..m {
    //     let i = ui as isize;
    //     print!("{} ", shuff_res(i));
    //     // print!("{}", dwi(dins(dins(i, m), m), 7, m));
    // }
    // println!();
    // for ui in 0..m {
    //     let i = ui as isize;
    //     print!("{}", cut(dwi(dins(i, m), 7, m), 6, m));
    // }
    // println!();

    return if part1 {
        // return shuff_res(2019);
        let mut ret = 0;
        for i in 0..m {
            if shuff_res(i) == 2019 {
                ret = i;
                break;
            }
        }
        ret
    } else {
        let mut i = 2020;
        for j in 0isize..101741582076661 {
            i = shuff_res(i);
            if i == 2020 {
                println!(" j: {:?}", j);
            }
        }
        i
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
