import math
import numpy as np
from pprint import pprint

with open('day7.problem.txt', 'r') as f:
    data = [l.strip().split(':') for l in f.readlines()]
    data = [(int(d[0]), [int(x) for x in d[1].split(' ')[1:]]) for d in data]

pprint(data)

def walk(target, value, seq, i):
    if value == target:
        return target
    if value > target or i == len(seq):
        return 0

    return walk(target, value + seq[i], seq, i + 1) or walk(target, value * seq[i], seq, i + 1)

print(sum(walk(d[0], d[1][0], d[1], 1) for d in data))

# part 2
def num_digits(n):
    return math.floor(math.log10(n)) + 1

def walk2(target, value, seq, i):
    if value == target:
        return target
    if value > target or i == len(seq):
        return 0

    return (
        walk2(target, value + seq[i], seq, i + 1)
        or walk2(target, value * seq[i], seq, i + 1)
        or walk2(target, value * (10 ** num_digits(seq[i])) + seq[i], seq, i + 1)
    )

print(sum(walk2(d[0], d[1][0], d[1], 1) for d in data))
