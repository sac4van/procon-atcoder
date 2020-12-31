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
    n = gete(int)

    def dfs(n):
        if n == 2:
            return 3

        d = dfs(n-1) - 1

        return d * (n // math.gcd(d, n)) + 1

    print(dfs(n))


if __name__ == "__main__":
    main()
