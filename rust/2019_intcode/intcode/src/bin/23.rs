use std::collections::VecDeque;
use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{spawn, yield_now};
use std::time::Instant;
use std::{fs, process};

const INPUT_PATH: &str = "./input/23/input";

use intcode::icc::ICC;

static WAIT_COUNTS: [AtomicU64; 50] = {
    const INIT: AtomicU64 = AtomicU64::new(0);
    [INIT; 50]
};

fn part(input: String, part1: bool) -> isize {
    let mut senders: Vec<Sender<(isize, isize)>> = Vec::new();
    let mut receivers: VecDeque<Receiver<(isize, isize)>> = VecDeque::new();
    for _ in 0..50 {
        let (snd, rcv) = mpsc::channel::<(isize, isize)>();
        senders.push(snd);
        receivers.push_back(rcv);
    }
    let (snd255, rec255) = mpsc::channel::<(isize, isize)>();
    for naddr in 0..50 {
        let mut icc = ICC::new(&input);
        icc.input_queue.push_back(naddr);
        let senders_clone = senders.clone();
        let snd255clone = snd255.clone();
        let recv = receivers.pop_front().unwrap();
        spawn(move || {
            loop {
                match icc.run() {
                    intcode::icc::State::Halted => return,
                    intcode::icc::State::Output(targ_naddr) => {
                        WAIT_COUNTS[naddr as usize].store(0, Ordering::Release);
                        let x = icc.run_until_first_output();
                        let y = icc.run_until_first_output();
                        if targ_naddr == 255 {
                            let _ = snd255clone.send((x, y));
                            continue;
                        }
                        let _ = senders_clone[targ_naddr as usize].send((x, y));
                    }
                    intcode::icc::State::Waiting => match recv.try_recv() {
                        Ok((x, y)) => {
                            icc.input_queue.extend([x, y].iter());
                        }
                        Err(_) => {
                            icc.input_queue.push_back(-1);

                            WAIT_COUNTS[naddr as usize].fetch_add(1, Ordering::Release);
                        }
                    },
                }
            }
        });
    }

    if part1 {
        if let Ok((_x, y)) = rec255.recv() {
            return y;
        } else {
            return 0;
        }
    } else {
        let mut last_sent = (-1, -1);
        // last_val = rec255.recv().unwrap();
        loop {
            while !WAIT_COUNTS
                .iter()
                .all(|c| c.load(Ordering::Acquire) >= 10000)
            {
                yield_now();
            }
            let mut val = rec255.recv().unwrap();
            while let Ok(pckt) = rec255.try_recv() {
                val = pckt;
            }
            if val.1 == last_sent.1 {
                return val.1;
            }
            println!("val: {:?}", val);
            let _ = senders[0].send(val);
            WAIT_COUNTS[0].store(0, Ordering::Release);
            last_sent = val;
        }
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
        assert_eq!(0, part("".to_string(), true));
    }
}
