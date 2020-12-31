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

    if n < 3:
        print(1)
        exit()

    left = 0
    right = n
    m = (left+right)//2

    while m > left:
        if m*(m+1) <= 2*(n+1):
            left = m
        else:
            right = m
        m = (left+right)//2

    print(n-m+1)


if __name__ == "__main__":
    main()
