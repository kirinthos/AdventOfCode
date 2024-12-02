from pprint import pprint
from collections import Counter

with open('day1.problem.1.txt', 'r') as f:
    data = list(map(lambda l: tuple(map(int, filter(bool, l.strip().split(' ')))), f.readlines()))


l = list(zip(*data))
print(sum(map(lambda x: abs(x[0] - x[1]), zip(sorted(l[0]), sorted(l[1])))))

left, right = l[0], Counter(l[1])
print(sum(map(lambda x: right[x] * x, left)))
