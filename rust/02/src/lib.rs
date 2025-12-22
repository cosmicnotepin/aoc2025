use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./../../input/02/input";

fn part(input: String, part1: bool) -> isize {
    let bounds: Vec<(&str, &str)> = input
        .trim()
        .split(",")
        .map(|bounds_str| bounds_str.split("-").next_tuple().unwrap())
        .collect();
    let mut p1_res: usize = 0;
    let mut seen = HashSet::new();
    for (lb, ub) in bounds {
        let ub_digits = ub.len();
        let lb_digits = lb.len();
        let max_block_count = if part1 { 2 } else { ub_digits };
        for block_count in 2..max_block_count + 1 {
            for block_len in max(1, lb_digits / block_count)..max(1, ub_digits / block_count) + 1 {
                let mut pre_digits = 10usize.pow((block_len - 1).try_into().unwrap());
                while pre_digits.to_string().len() == block_len {
                    let mut candidate = pre_digits;
                    for _ in 0..block_count - 1 {
                        candidate =
                            candidate * 10usize.pow(block_len.try_into().unwrap()) + pre_digits;
                    }
                    if candidate <= ub.parse().unwrap()
                        && candidate >= lb.parse().unwrap()
                        && seen.insert(candidate)
                    {
                        p1_res += candidate;
                    }
                    pre_digits += 1;
                }
            }
        }
    }
    return p1_res.try_into().unwrap();
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
    const TINPUT_PATH: &str = "./../../input/02/tinput";

    #[test]
    fn p1_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(1227775554, part(input, true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(4174379265, part(input, false));
    }
}
