import unittest

from typing import List
from math import trunc
import time

TINPUT_PATH: str = "./../../input/01/tinput"
INPUT_PATH: str = "./../../input/01/input"


def main(filename: str, part1: bool) -> int:
    turns: List[int] = []
    with open(filename, "r") as f:
        for line in f:
            amount = int(line[1:])
            match line[0]:
                case "L":
                    turns.append(-amount)
                case "R":
                    turns.append(amount)

    dial = 50
    p1_zeroes = 0
    p2_zeroes = 0
    for turn in turns:
        full_rots = trunc(turn / 100)  # integer division // rounds -3.5 to -4
        rest = turn - full_rots * 100
        p2_zeroes += abs(full_rots)
        if dial != 0:
            if dial + rest < 0 or dial + rest > 100:
                p2_zeroes += 1
        dial = (dial + rest) % 100
        if dial == 0:
            p1_zeroes += 1

    if part1:
        return p1_zeroes
    else:
        return p1_zeroes + p2_zeroes


class Aoc(unittest.TestCase):
    def test_p1(self):
        self.assertEqual(3, main(TINPUT_PATH, True))

    def test_p2_(self):
        self.assertEqual(6, main(TINPUT_PATH, False))


if __name__ == "__main__":
    start = time.time()
    print(f"p1: {main(INPUT_PATH, False)}", end="")
    print(f" in: {time.time() - start}")
    start = time.time()
    print(f"p1: {main(INPUT_PATH, True)}", end="")
    print(f" in: {time.time() - start}")
