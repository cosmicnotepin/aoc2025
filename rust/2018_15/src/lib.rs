use ndarray::{arr1, Array1};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Combatant {
    race: char,
    pos: Array1<isize>,
    hp: isize,
}

const INPUT_PATH: &str = "./../../input/2018/15/input";
const ELF_HP: isize = 200;
const ELF_DMG: isize = 3;
const GOBLIN_HP: isize = 200;
const GOBLIN_DMG: isize = 3;

fn pprint(map: &Vec<Vec<char>>, cmbtnts: &Vec<Combatant>) {
    println!("");
    for (row_i, row) in map.iter().enumerate() {
        for col in row {
            print!("{}", col);
        }
        print!(" ");
        for cmbtnt in cmbtnts {
            if cmbtnt.pos[0] == row_i as isize {
                print!("{}({})", cmbtnt.race, cmbtnt.hp);
            }
        }
        println!("");
    }
}

fn neighs(pos: &Array1<isize>) -> Vec<Array1<isize>> {
    vec![arr1(&[-1, 0]), arr1(&[0, -1]), arr1(&[1, 0]), arr1(&[0, 1])]
        .iter()
        .map(|e| e + pos)
        .collect()
}

//returns the length of the shortest path from from to to
// or None if there is no path
// adjacent positions have distance 1
fn get_shortest_path_len(
    from: &Array1<isize>,
    to: &Array1<isize>,
    map: &Vec<Vec<char>>,
) -> Option<isize> {
    //flood fill for now
    let mut todo: VecDeque<(Array1<isize>, isize)> = VecDeque::from([(from.clone(), 0)]);
    let mut seen: HashSet<Array1<isize>> = HashSet::from([from.clone()]);
    if from == to {
        return Some(0);
    }
    while todo.len() > 0 {
        let (cur, len) = todo.pop_front().unwrap();
        for neigh in neighs(&cur) {
            if neigh == to {
                return Some(len + 1);
            }
            if map[neigh[0] as usize][neigh[1] as usize] != '.' {
                continue;
            }
            if seen.insert(neigh.clone()) {
                todo.push_back((neigh, len + 1));
            }
        }
    }
    return None;
}

//returns the pos to move to if a move is necessary and possible
// returns None if moving in not necessary (target in range) or impossible (no path to any target)
fn calculate_move(
    cmbtnt: &Combatant,
    cmbtnts: &Vec<Combatant>,
    map: &Vec<Vec<char>>,
) -> Option<Array1<isize>> {
    let mut in_range: Vec<(Array1<isize>, isize)> = Vec::new();
    for trgt in cmbtnts.iter().filter(|c| c.race != cmbtnt.race && c.hp > 0) {
        for trgt_neigh in neighs(&trgt.pos) {
            if trgt_neigh == cmbtnt.pos {
                return None; //target in range, no need to mave
            }
            if map[trgt_neigh[0] as usize][trgt_neigh[1] as usize] != '.' {
                continue;
            }
            if let Some(dist) = get_shortest_path_len(&cmbtnt.pos, &trgt_neigh, map) {
                in_range.push((trgt_neigh.clone(), dist))
            }
        }
    }
    in_range.sort_by_key(|(pos, len)| len * -1000000 + pos[0] * -1000 - pos[1]);
    if let Some((chosen, _)) = in_range.pop() {
        let mut first_steps: Vec<(Array1<isize>, isize)> = Vec::new();
        for neigh in neighs(&cmbtnt.pos) {
            if map[neigh[0] as usize][neigh[1] as usize] != '.' {
                continue;
            }
            if let Some(dist) = get_shortest_path_len(&neigh, &chosen, &map) {
                first_steps.push((neigh.clone(), dist));
            }
        }
        first_steps.sort_by_key(|(pos, len)| len * -1000000 + pos[0] * -1000 - pos[1]); //sorting desc to be able to pop next!
        if let Some(first_step) = first_steps.pop() {
            return Some(first_step.0);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn calculate_target_index(cmbtnt: &Combatant, cmbtnts: &Vec<Combatant>) -> Option<usize> {
    let ns = neighs(&cmbtnt.pos);
    let mut candidates: Vec<(usize, Combatant)> = cmbtnts
        .iter()
        .enumerate()
        .filter(|(_, e)| e.hp > 0 && e.race != cmbtnt.race && ns.contains(&e.pos))
        .map(|(i, e)| (i, e.clone()))
        .collect();
    candidates.sort_by_key(|(_, e)| e.hp * -1000000 + e.pos[0] * -1000 - e.pos[1]);
    if let Some((i, _)) = candidates.pop() {
        return Some(i);
    } else {
        return None;
    }
}

fn part(input: String, part1: bool) -> isize {
    let mut orig_combatants: Vec<Combatant> = Vec::new();
    let mut orig_map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (row_i, row) in orig_map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            match col {
                'E' => orig_combatants.push(Combatant {
                    race: 'E',
                    pos: arr1(&[row_i as isize, col_i as isize]),
                    hp: ELF_HP,
                }),
                'G' => orig_combatants.push(Combatant {
                    race: 'G',
                    pos: arr1(&[row_i as isize, col_i as isize]),
                    hp: GOBLIN_HP,
                }),
                _ => (),
            }
        }
    }
    let orig_elf_count = orig_combatants
        .iter()
        .filter(|c| c.race == 'E' && c.hp > 0)
        .count();
    let mut rounds: isize = 0;
    let mut dmgs: HashMap<char, isize> = HashMap::from([('G', 3), ('E', 3)]);
    'outerouter: loop {
        let mut map = orig_map.clone();
        let mut combatants = orig_combatants.clone();
        rounds = 0;
        'outer: loop {
            for i in 0..combatants.len() {
                if !part1
                    && orig_elf_count
                        != combatants
                            .iter()
                            .filter(|c| c.race == 'E' && c.hp > 0)
                            .count()
                {
                    if let Some(dmg) = dmgs.get_mut(&'E') {
                        *dmg += 1;
                        println!("dmg: {:?}", dmg)
                    }
                    continue 'outerouter;
                }
                if combatants[i].hp <= 0 {
                    continue;
                }
                for race in ['E', 'G'] {
                    if combatants
                        .iter()
                        .filter(|c| c.race == race && c.hp > 0)
                        .count()
                        == 0
                    {
                        break 'outer;
                    }
                }
                if let Some(mv) = calculate_move(&combatants[i], &combatants, &map) {
                    map[combatants[i].pos[0] as usize][combatants[i].pos[1] as usize] = '.';
                    map[mv[0] as usize][mv[1] as usize] = combatants[i].race;
                    combatants[i].pos = mv;
                }
                if let Some(target_index) = calculate_target_index(&combatants[i], &combatants) {
                    let attacker_race = combatants[i].race;
                    combatants[target_index].hp -= dmgs.get(&attacker_race).unwrap();
                    if combatants[target_index].hp <= 0 {
                        map[combatants[target_index].pos[0] as usize]
                            [combatants[target_index].pos[1] as usize] = '.';
                    }
                }
                // pprint(&map, &combatants);
            }
            combatants.retain(|e| e.hp > 0);
            combatants.sort_by_key(|e| e.pos[0] * 1000 + e.pos[1]);
            rounds += 1;
            // println!("rounds: {:?}", rounds);
        }
        combatants.retain(|e| e.hp > 0);
        println!("rounds: {:?}", rounds);
        println!("combatants: {:?}", combatants);
        return combatants.iter().map(|c| c.hp).sum::<isize>() * rounds;
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

#[cfg(test)]
mod tests {
    use super::*;
    const TINPUT_PATH: &str = "./../../input/2018/15/tinput";

    #[test]
    fn p1_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(27730, part(input, true));
    }

    #[test]
    fn p1_2() {
        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
            .to_string();
        assert_eq!(36334, part(input, true));
    }

    #[test]
    fn p2_1() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        assert_eq!(0, part(input, false));
    }
    #[test]
    fn t_get_shortest_path_len() {
        let input = fs::read_to_string(TINPUT_PATH).unwrap();
        let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        assert_eq!(
            Some(3),
            get_shortest_path_len(&arr1(&[1, 2]), &arr1(&[2, 4]), &map)
        );
    }
}
