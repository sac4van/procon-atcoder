import sys
from collections import defaultdict
from itertools import permutations
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10**8)


def geta(fn=lambda s: s.decode()):
    return map(fn, readline().split())


def gete(fn=lambda s: s.decode()):
    return fn(readline().rstrip())


def main():

    n = gete(int)

    t = gete()

    def check(_n):
        if _n == 1:
            return True

        if _n == 2:
            return t[0:2] in ["10", "01", "11"]

        ret = check(_n-1)

        if not ret:
            return False
        else:
            return t[_n-3:_n] in ["110", "101", "011"]

    ans = 0

    if check(n):
        if t[0] == '0':
            ans = 10**10 - ((n+1) // 3)
        else:
            if len(t) == 1:
                ans = 2 * (10**10)
            elif t[0:2] == '10':
                # t[0:3] == "101"
                ans = 10**10 - (n//3)
            else:
                # t[0:2] == '110'
                ans = 10**10 - ((n-1)//3)

    print(ans)


if __name__ == "__main__":
    main()
