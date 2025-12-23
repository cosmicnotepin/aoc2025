use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./../../input/03/input";

fn part(input: String, part1: bool) -> usize {
    let banks: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let nr_bats = if part1 { 2 } else { 12 };
    let mut res: usize = 0;
    for bank in banks {
        let mut next_index = 0;
        let mut joltage = 0;
        for digit in (0..nr_bats).rev() {
            let slice = &bank[next_index..bank.len() - digit]; // doing this with iter().ski(n).take(m) is not that readable for me
            let (max_i, max) = slice
                .iter()
                .enumerate()
                .rev() // max_by returns last max element
                .max_by(|(_, l), (_, r)| l.cmp(r))
                .unwrap();
            next_index += max_i + 1;
            joltage = joltage * 10 + max;
        }
        res += joltage;
    }
    return res;
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
    const TINPUT_PATH: &str = "./../../input/03/tinput";

    #[test]
    fn p1_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(357, part(input, true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(3121910778619, part(input, false));
    }
}
