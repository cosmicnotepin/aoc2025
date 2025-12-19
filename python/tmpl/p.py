import unittest

import time

TINPUT_PATH: str = "./../../input/01/tinput"
INPUT_PATH: str = "./../../input/01/input"


def main(filename: str, part1: bool) -> int:
    with open(filename, "r") as f:
        for line in f:
            pass

    return int(part1)


class Aoc(unittest.TestCase):
    def test_p1(self):
        self.assertEqual(3, main(TINPUT_PATH, True))

    def test_p2_(self):
        self.assertEqual(6, main(TINPUT_PATH, False))


if __name__ == "__main__":
    start = time.time()
    print(f"p1: {main(INPUT_PATH, True)}", end="")
    print(f" in: {time.time() - start}")
    start = time.time()
    print(f"p2: {main(INPUT_PATH, False)}", end="")
    print(f" in: {time.time() - start}")
