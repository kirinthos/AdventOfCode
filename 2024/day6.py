import math
import numpy as np
from pprint import pprint

with open('day6.problem.txt', 'r') as f:
    data = [[c for c in l.strip()] for l in f.readlines()]

def print_data(data):
    for row in data:
        print(''.join(row))
    print()

directions = np.array([
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
])

char_for_dir = {
    0: '^',
    1: '>',
    2: 'V',
    3: '<',
}


grid = [[1 if c[0] == '#' else 0 for c in row] for row in data]
start = np.array(next((c, x, y) for y, row in enumerate(data) for x, c in enumerate(row) if c == '^')[1:])

def walk_grid(pos, dir_idx, track=True):
    moved_spaces = set()
    hit_walls = set()
    while True:
        newpos = pos + directions[dir_idx]

        if newpos[0] < 0 or newpos[0] >= len(grid[0]) or newpos[1] < 0 or newpos[1] >= len(grid):
            break

        if grid[newpos[1]][newpos[0]] == 1:
            dir_idx = (dir_idx + 1) % len(directions)
            p = (*newpos, dir_idx)
            if p in hit_walls:
                return None
            hit_walls.add(p)
        else:
            #data[pos[1]][pos[0]] = 'X'
            pos = newpos
            if track:
                moved_spaces.add(tuple(pos))

        #data[pos[1]][pos[0]] = char_for_dir[dir_idx]
        #print_data(data)

    return moved_spaces

moved_spaces = walk_grid(start, 0) or []
print(len(moved_spaces))

# part 2

counter = 0
for p in moved_spaces:
    print('trying', p)
    grid[p[1]][p[0]] = 1
    if walk_grid(start, 0) is None:
        counter += 1
    grid[p[1]][p[0]] = 0
print(counter)
