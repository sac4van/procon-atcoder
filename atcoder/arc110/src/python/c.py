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
    p = list(geta(int))

    ok = True
    head = 1

    ans = []
    tmp = []

    for cur in range(n-1):
        if p[cur + 1] == head:
            head = cur + 2
            p[cur + 1] = p[cur]

            tmp.append(cur+1)
            ans += reversed(tmp)
            tmp = []
        else:
            if p[cur] != cur + 2:
                ok = False
                break

            tmp.append(cur+1)

    if ok and (p[-1] == n):
        print(*ans, sep="\n")
    else:
        print(-1)


if __name__ == "__main__":
    main()
