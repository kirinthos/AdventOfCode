import math
import time
import numpy as np
from collections import Counter
from pprint import pprint

def parse_line(l):
    p, v = [s[2:] for s in l.strip().split(' ')]
    pos = np.array([int(x) for x in p.split(',')])
    vel = np.array([int(x) for x in v.split(',')])
    return pos, vel

with open('day14.problem.txt', 'r') as f:
    data = np.array([parse_line(l) for l in f.readlines()])
#size = np.array([11, 7]) # example
size = np.array([101, 103]) # problem 

def print_grid(size, pts):
    t = np.array([['.' for x in range(size[0])] for y in range(size[1])])
    t[pts[:, 1], pts[:, 0]] = 'X'
    np.set_printoptions(threshold=20000, linewidth=10000)
    print(t)


def count_quads(pts):
    align = (size - 1) / 2

    quads = np.sign(pts - align)
    quads = map(tuple, quads[(quads[:, 0] != 0) & (quads[:, 1] != 0)])
    counts = Counter(quads)
    return [counts[k] for k in sorted(counts.keys())]

pts = np.mod(data[:, 0] + data[:, 1] * 100, size)
print(np.prod(np.array(count_quads(pts))))

# part 2
steps = 0
while True:
    steps += 1
    pts = np.mod(data[:, 0] + data[:, 1] * steps, size)
    align = (size - 1) / 2

    if len(np.unique(pts, axis=0)) == len(pts):
        print_grid(size, pts)
        print(steps)
        input('found')
