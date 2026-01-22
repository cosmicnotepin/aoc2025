use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "./input/input";

// find the direction that points from pos to a next pipe in the loop
fn find_entry(
    pos: (usize, usize),
    map: &Vec<Vec<char>>,
    redirect: &HashMap<(char, char), (char, isize)>,
) -> Option<char> {
    let delta = HashMap::from([('^', (-1, 0)), ('<', (0, -1)), ('v', (1, 0)), ('>', (0, 1))]);
    for dir in ['^', '<', 'v', '>'] {
        let new_row = pos.0 as isize + delta[&dir].0;
        let new_col = pos.1 as isize + delta[&dir].1;
        if new_col < 0
            || new_row < 0
            || new_col > map[0].len() as isize - 1
            || new_row > map.len() as isize - 1
        {
            continue;
        }
        let c = map[new_row as usize][new_col as usize];
        if let Some(_) = redirect.get(&(c, dir)) {
            return Some(dir);
        }
    }
    return None;
}
// follow the pipes one step, dir is the direction that we use to find the next pipe coming from pos, so for an F, coming from below with '^' that would be '>'
//
//
//         F-
// pos --> x
// dir: '^'
// follow(pos,dir) = (pos of 'F', '>', ...)
fn follow(
    pos: (usize, usize),
    dir: char,
    map: &Vec<Vec<char>>,
    redirect: &HashMap<(char, char), (char, isize)>,
    turns: isize,
) -> ((usize, usize), char, isize) {
    let delta = HashMap::from([('^', (-1, 0)), ('<', (0, -1)), ('v', (1, 0)), ('>', (0, 1))]);
    let new_pos = (
        (pos.0 as isize + delta[&dir].0) as usize,
        (pos.1 as isize + delta[&dir].1) as usize,
    );
    let c = map[new_pos.0][new_pos.1];
    if c == 'S' {
        //we do not know new_dir for 'S', but we know that iteration stops in the calling function when we return the position of 'S' as pos
        return (new_pos, dir, turns);
    }

    let (new_dir, turn) = redirect[&(c, dir)];
    return (new_pos, new_dir, turns + turn);
}

fn part(input: String, part1: bool) -> isize {
    //how does goung through a pipe change our direction?
    let redirect: HashMap<(char, char), (char, isize)> = HashMap::from([
        (('|', '^'), ('^', 0)),
        (('|', 'v'), ('v', 0)),
        (('-', '<'), ('<', 0)),
        (('-', '>'), ('>', 0)),
        (('F', '^'), ('>', 1)),
        (('F', '<'), ('v', -1)),
        (('7', '^'), ('<', -1)),
        (('7', '>'), ('v', 1)),
        (('J', 'v'), ('<', 1)),
        (('J', '>'), ('^', -1)),
        (('L', 'v'), ('>', -1)),
        (('L', '<'), ('^', 1)),
    ]);
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (row_i, col_i);
            }
        }
    }
    let mut dir = find_entry(start, &map, &redirect).unwrap();
    let mut pipes: Vec<((usize, usize), char, char)> = Vec::new();
    // pipes: ((row,col),entry_dir,exit_dir)) will contain the loop and the direction when travelling each peace (for knowing the "inside" in part two)
    let mut turns: isize = 0; //right turn: +=1, left turn -=1 in follow(...)
    let mut pos = start;
    let mut new_dir = dir;
    pipes.push((pos, dir, new_dir));
    (pos, new_dir, turns) = follow(pos, dir, &map, &redirect, turns);
    while pos != start {
        pipes.push((pos, dir, new_dir));
        dir = new_dir;
        (pos, new_dir, turns) = follow(pos, dir, &map, &redirect, turns);
    }
    let turn = if turns > 0 { 'r' } else { 'l' };

    if part1 {
        return pipes.len() as isize / 2 as isize;
    }

    let border: HashSet<(usize, usize)> = pipes.iter().map(|e| e.0).collect();
    let mut inside: HashSet<(usize, usize)> = HashSet::new();
    let inside_delta: HashMap<(char, char), (isize, isize)> = HashMap::from([
        (('^', 'l'), (0, -1)),
        (('^', 'r'), (0, 1)),
        (('v', 'l'), (0, 1)),
        (('v', 'r'), (0, -1)),
        (('>', 'l'), (-1, 0)),
        (('>', 'r'), (1, 0)),
        (('<', 'l'), (1, 0)),
        (('<', 'r'), (-1, 0)),
    ]);
    for ((row, col), dir1, dir2) in pipes {
        for d in [
            inside_delta.get(&(dir1, turn)),
            inside_delta.get(&(dir2, turn)),
        ] {
            if let Some((row_d, col_d)) = d {
                let inside_candidate = (
                    (row as isize + row_d) as usize,
                    (col as isize + col_d) as usize,
                );
                // i am just flood-filling from all inside fields, this could be https://en.wikipedia.org/wiki/Shoelace_formula for the area
                // and https://en.wikipedia.org/wiki/Pick%27s_theorem for the inside of that
                let mut todo = Vec::from([inside_candidate]);
                while let Some(cur) = todo.pop() {
                    if border.contains(&cur) || !inside.insert(cur) {
                        continue;
                    }
                    for (rd, cd) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                        let neigh = (
                            (cur.0 as isize + rd) as usize,
                            (cur.1 as isize + cd) as usize,
                        );
                        todo.push(neigh);
                    }
                }
            }
        }
    }
    return inside.len() as isize;
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
        assert_eq!(8, part(input, true));
    }

    #[test]
    fn p2_1() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"
        .to_string();
        assert_eq!(10, part(input, false));
    }
}
