use std::error::Error;
use std::fs;
use std::time::Instant;

fn part1(input: String) -> i32 {
    return input.len().try_into().unwrap();
}

fn part2(input: String) -> i32 {
    return input.len().try_into().unwrap();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let before1 = Instant::now();
    let input1 = fs::read_to_string("input")?;
    let p1 = part1(input1);
    println!("part 1: {} in {:.2?}", p1, before1.elapsed());
    let before2 = Instant::now();
    let input2 = fs::read_to_string("input")?;
    let p2 = part2(input2);
    println!("part 2: {} in {:.2?}", p2, before2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1() {
        let input = fs::read_to_string("tinput").unwrap();
        assert_eq!(0, part1(input));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string("tinput").unwrap();
        assert_eq!(0, part2(input));
    }
}
