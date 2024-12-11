import numpy as np
from pprint import pprint

with open('./day10.problem.txt', 'r') as f:
    data = np.array([[int(c) for c in l.strip()] for l in f.readlines()])

directions = np.array([
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1]
])

bounds = len(data[0]), len(data)
def in_bounds(pt):
    return pt[0] >= 0 and pt[0] < bounds[0] and pt[1] >= 0 and pt[1] < bounds[1]

# this is haskell-style solution hahahaha wish list-head concatenation was fast in python
def walk(pt):
    if not in_bounds(pt):
        return None

    if data[tuple(pt)] == 9:
        return [pt]

    pts = [walk(pt + d) for d in directions if in_bounds(pt + d) and data[tuple(pt + d)] - data[tuple(pt)] == 1]
    pts = set(tuple(p) for l in pts for p in l)
    return pts

#print(walk(np.array([0, 2])))
counts = list(walk(np.array([x, y])) for x in range(bounds[0]) for y in range(bounds[1]) if data[x, y] == 0)
print(sum(len(c) for c in counts))

# part 2
def walk(pt):
    if not in_bounds(pt):
        return None

    if data[tuple(pt)] == 9:
        return [pt]

    pts = [walk(pt + d) for d in directions if in_bounds(pt + d) and data[tuple(pt + d)] - data[tuple(pt)] == 1]
    # this is the change for part 2, a list
    pts = list(tuple(p) for l in pts for p in l)
    return pts

counts = list(walk(np.array([x, y])) for x in range(bounds[0]) for y in range(bounds[1]) if data[x, y] == 0)
print('part 2', sum(len(c) for c in counts))
