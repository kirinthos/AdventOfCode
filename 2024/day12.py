import numpy as np
from pprint import pprint

from numpy._typing import NDArray

with open('day12.problem.txt', 'r') as f:
    data = np.array([[c for c in l.strip()] for l in f.readlines()])

# Note: using numpy tuple indexing switches x and y
# because (1, 0) looks at the second row, first column
directions = np.array([
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
])

corner_directions = np.array([
    [[0, 1], [1, 1], [1, 0]],
    [[1, 0], [1, -1], [0, -1]],
    [[0, -1], [-1, -1], [-1, 0]],
    [[-1, 0], [-1, 1], [0, 1]],
])

bounds = len(data[0]), len(data)
def in_bounds(pt):
    return pt[0] >= 0 and pt[0] < bounds[0] and pt[1] >= 0 and pt[1] < bounds[1]

visited = np.array([[False for x in range(bounds[0])] for y in range(bounds[1])])
def walk(pt, marker=None):
    tpt = tuple(pt)
    if not in_bounds(pt) or visited[tpt]:
        return np.array([0, 0])

    visited[tpt] = True

    if marker is None:
        marker = data[tpt]

    pts = [pt + d for d in directions if in_bounds(pt + d) and data[tuple(pt + d)] == marker]
    perimeter = 4 - len(pts)
    return np.array([1, perimeter]) + sum(walk(p) for p in pts)

print(sum(np.prod(walk(np.array([x, y]))) for x in range(bounds[0]) for y in range(bounds[1])))

# part 2
# count the corners

visited = np.array([[False for x in range(bounds[0])] for y in range(bounds[1])])
def walk2(pt, marker=None):
    tpt = tuple(pt)
    if not in_bounds(pt) or visited[tpt]:
        return np.array([0, 0])

    visited[tpt] = True

    if marker is None:
        marker = data[tpt]

    pts = [pt + d for d in directions if in_bounds(pt + d) and data[tuple(pt + d)] == marker]
    corners = [[data[tuple(pt + d)] != marker if in_bounds(pt + d) else True for d in c] for c in corner_directions]
    num_corners = len([x for x in corners if x == [True, False, True] or x == [False, True, False] or x == [True, True, True]])

    return np.array([1, num_corners]) + sum(walk2(p, marker) for p in pts)

print('part 2', sum(np.prod(walk2(np.array([x, y]))) for x in range(bounds[0]) for y in range(bounds[1])))
