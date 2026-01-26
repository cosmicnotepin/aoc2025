use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./input/input";

fn part(input: String, part1: bool) -> isize {
    let prefix_len = if part1 { 5 } else { 7 };
    'outer: for i in 0.. {
        let digest = md5::compute(input.trim().to_owned() + i.to_string().as_str());
        let hex = format!("{:x}", digest);
        for c in hex.chars().take(prefix_len) {
            if c != '0' {
                continue 'outer;
            }
        }
        return i;
    }
    return 0;
}

//3-4x speedup using "by_exponential_blocks", speeddown without it :)
fn partp(input: String, part1: bool) -> isize {
    let prefix_len = if part1 { 5 } else { 7 };
    let res = (0..isize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|i| {
            let digest = md5::compute(input.trim().to_owned() + i.to_string().as_str());
            let hex = format!("{:x}", digest);
            let mut ret = true;
            for c in hex.chars().take(prefix_len) {
                if c != '0' {
                    ret = false;
                    break;
                }
            }
            ret
        });
    return res.unwrap() as isize;
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
        assert_eq!(609043, part("abcdef".to_string(), true));
    }

    #[test]
    fn p1_2() {
        assert_eq!(1048970, part("pqrstuv".to_string(), true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(0, part(input, false));
    }
}
