from concurrent.futures.process import _global_shutdown
import unittest

import time

TINPUT_PATH: str = "./../../input/02/tinput"
INPUT_PATH: str = "./../../input/02/input"


def main(filename: str, part1: bool) -> int:
    p1_res: int = 0
    with open(filename, "r") as f:
        ranges = [
            (lb, ub) for lb, ub in [r.split("-") for r in f.read().strip().split(",")]
        ]

    for lb, ub in ranges:
        lb_digits = len(lb)
        if lb_digits % 2 == 1:
            pre_digits = 10 ** (lb_digits // 2)
        else:
            pre_digits = int(lb[0 : lb_digits // 2])
        while True:
            candidate = pre_digits * 10 ** len(str(pre_digits)) + pre_digits
            if candidate > int(ub):
                break
            if candidate >= int(lb):
                p1_res += candidate
            pre_digits += 1

    if part1:
        return p1_res
    else:
        return 0


class Aoc(unittest.TestCase):
    def test_p1(self):
        self.assertEqual(1227775554, main(TINPUT_PATH, True))

    def test_p2_(self):
        self.assertEqual(6, main(TINPUT_PATH, False))


if __name__ == "__main__":
    start = time.time()
    print(f"p1: {main(INPUT_PATH, True)}", end="")
    print(f" in: {time.time() - start}")
    start = time.time()
    print(f"p2: {main(INPUT_PATH, False)}", end="")
    print(f" in: {time.time() - start}")
