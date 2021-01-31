import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10**8)


def geta(fn=lambda s: s.decode()):
    return map(fn, readline().split())


def gete(fn=lambda s: s.decode()):
    return fn(readline().rstrip())


class TreeNode:

    def __init__(self, label: int):
        self.label = label
        self.value1 = 0
        self.value2 = 0
        self.children = []
        self.parent = -1


def main():

    n = gete(int)
    e = []
    g = [[] for _ in range(n)]
    used = [False for _ in range(n)]
    tree = [TreeNode(i) for i in range(n)]

    for _ in range(1, n):
        a, b = geta(int)
        e.append((a-1, b-1))
        g[a-1].append(b-1)
        g[b-1].append(a-1)

    used[0] = True
    que = [0]
    while len(que) > 0:
        t = que.pop()
        for _n in g[t]:
            if not used[_n]:
                tree[t].children.append(_n)
                tree[_n].parent = t
                used[_n] = True
                que.append(_n)

    q = gete(int)
    sum2 = 0
    for _ in range(q):
        _t, _e, _x = geta(int)
        a, b = e[_e-1]
        if _t == 2:
            a, b = b, a

        if tree[b].parent == a:
            tree[b].value2 += _x
            sum2 += _x
        else:
            tree[a].value1 += _x

    c = [sum2 for _ in range(n)]

    def dfs(label, s1, s2):
        s1 += tree[label].value1
        s2 += tree[label].value2
        c[label] += s1 - s2
        for _n in tree[label].children:
            dfs(_n, s1, s2)

    dfs(0, 0, 0)

    print(*c, sep="\n")


if __name__ == "__main__":
    main()
