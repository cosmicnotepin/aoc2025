use memoize::memoize;
use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./input/input";

#[memoize]
fn possible_variations(springs: Vec<char>, free_wrkng: usize, broken_blocks: Vec<usize>) -> usize {
    let mut res = 0;
    'outer: for block_size in 0..free_wrkng + 1 {
        // does a block of working at this position fit? if not -> a longer block will not fit either
        for i in 0..block_size {
            if springs[i] == '#' {
                break 'outer;
            }
        }
        // if there are no more broken blocks and we have fitted all working -> Success
        // else
        if broken_blocks.len() == 0 {
            if springs.len() == block_size {
                return 1;
            }
            continue 'outer;
        }
        // does the next block of broken fit right after? if not -> it might fit after a longer block of working
        for i in block_size..block_size + broken_blocks[0] {
            if springs[i] == '.' {
                continue 'outer;
            }
        }
        // if there is one more broken block we need a separator
        if broken_blocks.len() > 1 && springs[block_size + broken_blocks[0]] == '#' {
            continue 'outer;
        }
        // if there is one more broken block after this, also remove the separator-'.' from springs
        let mut springs_offset = block_size + broken_blocks[0];
        if broken_blocks.len() > 1 {
            springs_offset += 1
        }
        res += possible_variations(
            springs[springs_offset..].to_vec(),
            free_wrkng - block_size,
            broken_blocks[1..].to_vec(),
        )
    }
    return res;
}

fn part(input: String, part1: bool) -> isize {
    let mut all_springs: Vec<(Vec<char>, Vec<usize>)> = Vec::new();
    for line in input.lines() {
        let (springs_s, blocks_s) = line.split_once(' ').unwrap();
        let springs = springs_s.chars().collect();
        let blocks = blocks_s
            .split(',')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        all_springs.push((springs, blocks));
    }
    let mut res = 0;
    for (o_springs, o_blocks) in &all_springs {
        let mut springs = o_springs.clone();
        let mut blocks = o_blocks.clone();
        if !part1 {
            for _ in 0..4 {
                springs.push('?');
                springs.extend(o_springs);
                blocks.extend(o_blocks);
            }
        }
        let nr_wrkng = springs.len() - blocks.iter().sum::<usize>();
        let free_wrkng = nr_wrkng - (blocks.len() - 1); //mandatory working spring between broken blocks

        // now distribute the working on the working blocks and check against spring
        res += possible_variations(springs.to_vec(), free_wrkng, blocks.to_vec());
    }
    return res as isize;
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
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(21, part(input, true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(525152, part(input, false));
    }
}
