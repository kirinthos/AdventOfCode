import math
from pprint import pprint
from itertools import groupby
from functools import cmp_to_key

with open('day5.problem.txt', 'r') as f:
    data = [l.strip() for l in f.readlines()]
    idx = data.index('')
    page_order, updates = data[:idx], data[idx + 1:]

# attempt:
#   - build up a map of of sets of pages that come before
#   - iterate through each update, collect printed pages in a set
#     - when printing each page, check the set of previous pages against the set in our map
#     - if there's nonzero intersection, bail
#   - if we complete the iteration, include the update

pages = {k: set(map(lambda x: x[1], v)) for k, v in groupby(sorted([tuple(map(int, v.split('|'))) for v in page_order]), key=lambda x: x[0]) }

updates = [[int(v) for v in l.split(',')] for l in updates ]
    
def check_update(update):
    passed = set()
    for v in update:
        if v in pages and len(pages[v].intersection(passed)) > 0:
            return False
        passed.add(v)
    return True

print(sum( u[math.floor(len(u) / 2)] for u in updates if check_update(u) ))

def compare(a, b):
    if a in pages and b in pages[a]:
        return -1
    elif b in pages and a in pages[b]:
        return 1
    else:
        return 0

bad_updates = [u for u in updates if not check_update(u)]
for u in bad_updates:
    u.sort(key=cmp_to_key(compare))

print(sum( u[math.floor(len(u) / 2)] for u in bad_updates ))
