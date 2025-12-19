from typing import List
from math import trunc


def main(filename: str):
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
        full_rots = trunc(turn / 100)
        rest = turn - full_rots * 100
        p2_zeroes += abs(full_rots)
        if dial != 0:
            if dial + rest < 0 or dial + rest > 100:
                p2_zeroes += 1
        dial = (dial + rest) % 100
        if dial == 0:
            p1_zeroes += 1

    print(f"p1_zeroes: {p1_zeroes}")
    print(f"p2_zeroes: {p2_zeroes + p1_zeroes}")


if __name__ == "__main__":
    main("t_input")
    main("input")
