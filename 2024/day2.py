from pprint import pprint
from collections import Counter
from functools import reduce
import numpy as np

with open('day2.problem.1.txt', 'r') as f:
    data = list(map(lambda l: np.array(list(map(int, filter(bool, l.strip().split(' '))))), f.readlines()))


def analyze(l):
    diff = l[:-1] - l[1:]
    sign = len(np.unique(np.sign(diff))) == 1
    diff = np.abs(diff)
    return sign and np.all((diff > 0) & (diff < 4))

print(sum(map(analyze, data)))

def sign(a, b):
    return 0 if b - a == 0 else (b - a) / abs(b - a)

"""

def analyze_part2(acc, n):
    # acc = (previous, sign, dampener, result)
    if acc[3] == False:
        return acc

    diff = abs(n - acc[0])
    s = sign(acc[0], n)
    if diff > 0 and diff < 4 and s!= 0 and (acc[1] == False or s == acc[1]):
        return (n, s, *acc[2:])

    if acc[2]:
        return (*acc[:3], False)
    else:
        return (acc[0], acc[1], True, acc[3])

def f(l):
    result = False

    # i wasn't sure how to handle this properly, so a special case, the starting element
    # needs to be removed...
    diff = abs(l[1] - l[0])
    s1, s2 = sign(l[0], l[1]), sign(l[1], l[2])
    if s1 != s2 or diff < 1 or diff > 3:
        result = reduce(analyze_part2, l[2:], (l[1], False, True, True))[3]

    if not result:
        result = reduce(analyze_part2, l[1:], (l[0], False, False, True))[3]

    return result

print(sum(map(f, data)))
"""

# attempt 2, naive

def analyze(l):
    diff = l[:-1] - l[1:]
    sign = len(np.unique(np.sign(diff))) == 1
    diff = np.abs(diff)
    return sign and np.all((diff > 0) & (diff < 4))

s = 0
for l in data:
    result = analyze(l)
    if result:
        s += 1
    else:
        for i in range(len(l) - 1):
            modified = np.concatenate((l[0:i], l[i+1:]))
            result = analyze(modified)
            if result:
                s += 1
                break

print(s)
