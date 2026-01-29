use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::time::Instant;
use std::{fs, process, thread};

const INPUT_PATH: &str = "./input/15/input";
struct Dir {
    cmd: isize,
    rev_cmd: isize,
    delta: (isize, isize),
}
const DIRS: [Dir; 4] = [
    Dir {
        cmd: 1,     //north
        rev_cmd: 2, //south
        delta: (-1, 0),
    },
    Dir {
        cmd: 3,     //west
        rev_cmd: 4, //east
        delta: (0, -1),
    },
    Dir {
        cmd: 2, //south
        rev_cmd: 1,
        delta: (1, 0),
    },
    Dir {
        cmd: 4,     //east
        rev_cmd: 3, //west
        delta: (0, 1),
    },
];

use intcode::icc::ICC;

//recursive DFS makes it easy to have the robot backtrack :)
fn explore(
    dir: Dir,
    pos: (isize, isize),
    map: &mut HashMap<(isize, isize), char>,
    to_icc: &std::sync::mpsc::Sender<isize>,
    from_icc: &std::sync::mpsc::Receiver<isize>,
) {
    let new_pos = (pos.0 + dir.delta.0, pos.1 + dir.delta.1);
    if map.contains_key(&new_pos) {
        return;
    }
    let _ = to_icc.send(dir.cmd);
    match from_icc.recv().unwrap() {
        0 => {
            map.insert(new_pos, '#');
            return;
        }
        1 => {
            map.insert(new_pos, '.');
        }
        2 => {
            map.insert(new_pos, 'O');
        }
        _ => println!("unexpected status"),
    }
    for dir in DIRS {
        explore(dir, new_pos, map, &to_icc, &from_icc);
    }
    let _ = to_icc.send(dir.rev_cmd);
    let _ = from_icc.recv();
}

#[allow(unused)]
fn pprint(map: &HashMap<(isize, isize), char>) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    for (y, x) in map.keys() {
        min_x = min(min_x, *x);
        max_x = max(max_x, *x);
        min_y = min(min_y, *y);
        max_y = max(max_y, *y);
    }
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if let Some(c) = map.get(&(y, x)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

//time to fill to 'to', or time to fill eveything if 'to' cannot be found
fn floodfill(map: &HashMap<(isize, isize), char>, from: (isize, isize), to: char) -> isize {
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut todo: VecDeque<((isize, isize), isize)> = VecDeque::from([(from.clone(), 0)]);
    let mut max_len = 0;
    while let Some(((y, x), len)) = todo.pop_front() {
        let c = *map.get(&(y, x)).unwrap();
        if seen.contains(&(y, x)) || c == '#' {
            continue;
        }
        if c == to {
            return len;
        }
        max_len = len;
        seen.insert((y, x));
        for dir in DIRS {
            let neigh = (y + dir.delta.0, x + dir.delta.1);
            todo.push_back((neigh, len + 1));
        }
    }
    return max_len;
}

fn part(input: String, part1: bool) -> isize {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let (mut icc, to_icc, from_icc) = ICC::new(input);
    thread::spawn(move || icc.run());
    map.insert((0, 0), '.');
    for dir in DIRS {
        explore(dir, (0, 0), &mut map, &to_icc, &from_icc);
    }
    // pprint(&map);
    if part1 {
        return floodfill(&map, (0, 0), 'O');
    } else {
        let oxygen_pos = *map.iter().find(|e| *e.1 == 'O').unwrap().0;
        return floodfill(&map, oxygen_pos, ' ');
    }
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

fn main() {
    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //const INPUT_PATH: &str = "./input/09/tinput";

    #[test]
    fn p1_1() {
        let input = "1,1,1,4,99,5,6,0,99".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(30, icc.memory[0]);
    }

    #[test]
    fn p1_2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(3500, icc.memory[0]);
    }

    #[test]
    fn p1_3() {
        let input = "1002,4,3,4,33".to_string();
        let (mut icc, _, _) = ICC::new(input);

        icc.run();
        assert_eq!(99, icc.memory[4]);
    }

    #[test]
    fn p2_2() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let (mut icc, to_icc, from_icc) = ICC::new(input);
        let _ = to_icc.send(4);

        icc.run();
        assert_eq!(999, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_3() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let (mut icc, to_icc, from_icc) = ICC::new(input);
        let _ = to_icc.send(190);

        icc.run();
        assert_eq!(1001, from_icc.recv().unwrap());
    }

    #[test]
    fn p2_9() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!(input, res);
    }
    #[test]
    fn p2_91() {
        let input = "1102,34915192,34915192,7,4,7,99,0".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!("1219070632396864", res);
    }
    #[test]
    fn p2_92() {
        let input = "104,1125899906842624,99".to_string();
        let (mut icc, _to_icc, from_icc) = ICC::new(input.clone());

        icc.run();
        drop(icc); //without this the iterator below just blocks it seems

        let res = from_icc
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("res : {:?}", res);
        assert_eq!("1125899906842624", res);
    }
}
