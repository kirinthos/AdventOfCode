import math
import numpy as np
from itertools import groupby, combinations, permutations
from pprint import pprint

with open('day8.problem.txt', 'r') as f:
    lines = f.readlines()
    bounds = len(lines[0].strip()), len(lines)
    data = sorted(list(filter(bool, ((c, (x, y)) for y, l in enumerate(lines) for x, c in enumerate(l.strip()) if c != '.'))))
    data = {k: list(map(lambda x: np.array(x[1]), v)) for k, v in groupby(data, key=lambda x: x[0])}

def in_bounds(pt):
    return pt[0] >= 0 and pt[0] < bounds[0] and pt[1] >= 0 and pt[1] < bounds[1]

pts = ((p1 + (p1 - p2), p2 + (p2 - p1)) for v in data.values() for p1, p2 in combinations(v, 2))
pts = set(tuple(p) for l in pts for p in l if in_bounds(p))
print(len(pts))

# find slope of lines
# find all points that lie within bounds
pts = list((p1, p2) for v in data.values() for p1, p2 in permutations(v, 2))

# i feel like there's an easier way to do this but whatever, we'll brute force it
def antinodes_for_points(p1, p2):
    diff = p2 - p1
    pt = p1
    traveled = set()
    while in_bounds(pt):
        traveled.add(tuple(pt))
        pt = pt + diff

    return traveled

nodes = {v for p in pts for v in antinodes_for_points(*p)}
print(len(nodes))
