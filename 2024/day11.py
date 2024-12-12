import math
import numpy as np
from collections import defaultdict
from functools import reduce
from pprint import pprint

with open('./day11.problem.txt', 'r') as f:
    data = [int(c) for c in f.read().strip().split(' ')]

def num_digits(n):
    return int(math.log10(n)) + 1

def stone(n):
    if n == 0:
        return [1]

    num = num_digits(n)
    if num % 2 == 0:
        d = math.pow(10, num // 2)
        return [n // d, n % d]

    return [n * 2024]

def flatten(l):
    return [v for sub in l for v in sub]

def blink(l):
    return flatten([stone(v) for v in l])


#print(len(reduce(lambda acc, n: blink(acc), range(25), data)))

# part 2
def blink2(d):
    newd = defaultdict(lambda: 0)
    for n, v in d.items():
        num = num_digits(n) if n > 0 else 0
        if n == 0:
            newd[1] += v
        elif num % 2 == 0:
            d = math.pow(10, num // 2)
            newd[n // d] += v
            newd[n % d] += v
        else:
            newd[n * 2024] += v
    return newd

stones = {k: 1 for k in data}

for i in range(75):
    stones = blink2(stones)

print(sum(stones.values()))
