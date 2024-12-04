from pprint import pprint

with open('day4.problem.txt', 'r') as f:
    grid = [[c for c in l.strip()] for l in f.readlines() ]

# 8-directional
directions = [-1, -1, 0, 1, -1, 1, 1, 0, -1]
directions = list(zip(directions, directions[1:]))


def search_for_xmas(grid, pt):
    return sum([
        ''.join(grid[pt[1] + d[1] * i][pt[0] + d[0] * i] for i in range(4)) == 'XMAS'
        for d in directions
        if pt[0] + d[0] * 3 >= 0 and pt[0] + d[0] * 3 < len(grid[0])
            and pt[1] + d[1] * 3 >= 0 and pt[1] + d[1] * 3 < len(grid)
    ])


print(sum(search_for_xmas(grid, (x, y)) for y in range(len(grid)) for x in range(len(grid[0]))))

direction_pairs = [
    ((-1, -1), (1, 1)),
    ((-1, 1), (1, -1))
]

def search_for_mas(grid, pt):
    return all(set(grid[pt[1] + d[1]][pt[0] + d[0]] for d in direction) == {'M', 'S'} for direction in direction_pairs)

print(sum(search_for_mas(grid, (x, y)) for y in range(1, len(grid) - 1) for x in range(1, len(grid[0]) - 1) if grid[y][x] == 'A'))
