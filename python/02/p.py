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

    seen = set()
    for lb, ub in ranges:
        lb_digits = len(lb)
        ub_digits = len(ub)
        max_block_count = 2 if part1 else ub_digits
        for block_count in range(2, max_block_count + 1):
            for block_len in range(
                max(1, lb_digits // block_count), max(1, ub_digits // block_count) + 1
            ):
                pre_digits = 10 ** (block_len - 1)
                while len(str(pre_digits)) == block_len:
                    candidate = pre_digits
                    for _ in range(0, block_count - 1):
                        candidate = candidate * 10**block_len + pre_digits

                    if (
                        candidate <= int(ub)
                        and candidate >= int(lb)
                        and candidate not in seen
                    ):
                        seen.add(candidate)
                        p1_res += candidate
                    pre_digits += 1

    return p1_res


class Aoc(unittest.TestCase):
    def test_p1(self):
        self.assertEqual(1227775554, main(TINPUT_PATH, True))

    def test_p2_(self):
        self.assertEqual(4174379265, main(TINPUT_PATH, False))


if __name__ == "__main__":
    start = time.time()
    print(f"p1: {main(INPUT_PATH, True)}", end="")
    print(f" in: {time.time() - start}")
    start = time.time()
    print(f"p2: {main(INPUT_PATH, False)}", end="")
    print(f" in: {time.time() - start}")
