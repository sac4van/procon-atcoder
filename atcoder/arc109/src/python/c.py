import sys
from collections import defaultdict
from itertools import permutations
readline = sys.stdin.buffer.readline
# sys.setrecursionlimit(10**8)


def geta(fn=lambda s: s.decode()):
    return map(fn, readline().split())


def gete(fn=lambda s: s.decode()):
    return fn(readline().rstrip())


def main():
    n, k = geta(int)
    s = gete()

    memo_fn = dict()

    pow2 = [1 for _ in range(k)]
    for i in range(1, k):
        pow2[i] = (pow2[i-1]*2) % n

    def result(x, y):
        if x == "R":
            return "P" if y == "P" else x
        elif x == "S":
            return "R" if y == "R" else x
        else:
            return "S" if y == "S" else x

    def fn(m, l):
        if (m, l) in memo_fn:
            return memo_fn[(m, l)]

        if l == 0:
            memo_fn[(m, l)] = s[m % n]
        else:
            memo_fn[(m, l)] = result(fn(m, l-1), fn((m + pow2[l-1]) % n, l-1))

        return memo_fn[(m, l)]

    print(fn(n, k))


if __name__ == "__main__":
    main()
