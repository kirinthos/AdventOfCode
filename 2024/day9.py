import math
import numpy as np
from pprint import pprint

# Note to readers, I tried really hard to avoid making a RAM map and actually moving it...
# sorry it came out so messy

with open('day9.problem.txt', 'r') as f:
    data = [int(c) for c in f.read().strip()]

storage = data[:]

# two pointers, one at the beginning, one at the end
"""
start_id, end_id = 0, len(data) // 2
start_idx, end_idx = 0, len(data) - 1
data_idx = 0
checksum = 0

while start_idx <= end_idx:
    # advance to next free space, accumulating checksum
    # OR fill the free space from the end, accumulating checksum
    if start_idx % 2 == 0: # data
        checksum += sum(start_id * i for i in range(data_idx, data_idx + data[start_idx]))
        data_idx += data[start_idx]
        start_idx += 1
        start_id += 1
    else: # free space
        # fill free space from end_idx until there isn't free space
        free_blocks = data[start_idx]
        to_move = min(free_blocks, data[end_idx])
        checksum += sum(end_id * i for i in range(data_idx, data_idx + to_move))
        data_idx += to_move
        data[start_idx] -= to_move
        data[end_idx] -= to_move

        if data[start_idx] == 0:
            start_idx += 1
        if data[end_idx] == 0:
            end_idx -= 2
            end_id -= 1


print(checksum)
"""

# part 2

def find(f, l):
    return next((i for i, e in enumerate(l) if f(i, e)), None)

def find_free_block(data, idx):
    return find(lambda i, x: x[0] == -1 and x[1] >= data[idx][1] and idx > i, data)

data = storage
# we've got to restructure the data to include the ID i think because i can't
# track it as we walk backward :thinking-face:
data = [(i // 2 if i % 2 == 0 else -1, int(v)) for i, v in enumerate(data)]
end_idx = len(data) - 1

while end_idx > 1:
    if data[end_idx][0] == -1:
        end_idx -= 1
        continue

    free_block = find_free_block(data, end_idx)
    if free_block is not None:
        # pull it out
        to_move = data[end_idx]
        # replace it with space
        data[end_idx] = (-1, to_move[1])

        # cleanup, join nearby free spaces
        # in front of us
        if end_idx < len(data) - 1 and data[end_idx + 1][0] == -1:
            data[end_idx] = (-1, data[end_idx][1] + data[end_idx + 1][1])
            data.pop(end_idx + 1)

        # and behind
        if data[end_idx - 1][0] == -1:
            data[end_idx - 1] = (-1, data[end_idx - 1][1] + data[end_idx][1])
            data.pop(end_idx)

        # now actually move the data
        p = data[free_block]
        # move it to the new location
        data[free_block] = p[0], p[1] - to_move[1]
        # if there's no free space, replace it
        if data[free_block][1] == 0:
            data[free_block] = to_move
        else:
            # otherwise insert it before
            data.insert(free_block, to_move)
 

    end_idx -= 1

print(data)

# and calc that checksum
checksum = 0
data_idx = 0
for i, n in data:
    if i == -1:
        data_idx += n
        continue

    checksum += sum(i * r for r in range(data_idx, data_idx + n))
    data_idx += n

print(checksum)
