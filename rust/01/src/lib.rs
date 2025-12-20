use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./../../input/01/input";

fn part1(input: String, part1: bool) -> isize {
    let turns: Vec<isize> = input
        .lines()
        .map(|l| match l.chars().nth(0).unwrap() {
            'L' => l[1..].parse::<isize>().unwrap() * -1,
            'R' => l[1..].parse::<isize>().unwrap(),
            _ => panic!(),
        })
        .collect();
    let mut dial: isize = 50;
    let mut p1_zeroes: isize = 0;
    let mut p2_zeroes: isize = 0;
    for turn in turns {
        let full_rots: isize = turn / 100; //seems to truncate
        p2_zeroes += full_rots.abs();
        let rest: isize = turn - full_rots * 100;
        if dial != 0 {
            if dial + rest < 0 || dial + rest > 100 {
                p2_zeroes += 1;
            }
        }
        dial = (dial + rest).rem_euclid(100);
        if dial == 0 {
            p1_zeroes += 1;
        }
    }
    return if part1 {
        p1_zeroes
    } else {
        p2_zeroes + p1_zeroes
    };
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string(INPUT_PATH)?;
    let p1 = part1(input1, true);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string(INPUT_PATH)?;
    let p2 = part1(input2, false);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TINPUT_PATH: &str = "./../../input/01/tinput";

    #[test]
    fn p1_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(3, part1(input, true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(6, part1(input, false));
    }
}
