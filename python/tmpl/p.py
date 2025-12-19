import unittest
import time


def main(filename: str, part1: bool) -> int:
    with open(filename, "r") as f:
        for line in f:
            pass

    return int(part1)


class Aoc(unittest.TestCase):
    def test_p1(self):
        self.assertEqual(-1, main("tinput", True))

    def test_p2_(self):
        self.assertEqual(-1, main("tinput", False))


if __name__ == "__main__":
    start = time.time()
    print(f"p1: {main('input', False)}", end="")
    print(f" in: {time.time() - start}")
    start = time.time()
    print(f"p1: {main('input', True)}", end="")
    print(f" in: {time.time() - start}")
