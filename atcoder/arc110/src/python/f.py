import sys
from collections import defaultdict
from itertools import permutations
import math
readline = sys.stdin.buffer.readline
# sys.setrecursionlimit(10**8)


def geta(fn=lambda s: s.decode()):
    return map(fn, readline().split())


def gete(fn=lambda s: s.decode()):
    return fn(readline().rstrip())


def main():
    a, b, x, y = geta(int)

    ans = x
    ans += abs(a-b - 1) * min(2*x, y) if a > b \
        else abs(a-b) * min(2*x, y)

    print(ans)


if __name__ == "__main__":
    main()
