import math
import numpy as np
from functools import cache
from pprint import pprint

def split_line(l):
    x, y = l[0].split(': ')[1].split(', ')
    ax, ay = int(x[1:]), int(y[1:])
    x, y = l[1].split(': ')[1].split(', ')
    bx, by = int(x[1:]), int(y[1:])
    x, y = l[2].split(': ')[1].split(', ')
    ex, ey = int(x[2:]), int(y[2:])
    return [(ax, ay), (bx, by), (ex, ey)]

with open('day13.problem.txt', 'r') as f:
    data = [l.strip() for l in f.readlines() if l != '\n']
    data = [split_line(l) for l in zip(*(iter(data),) * 3)]

@cache
def search(p, a, b, an, bn):
    c = (a[0] * an + b[0] * bn, a[1] * an + b[1] * bn)
    if c == p:
        return an * 3 + bn
    if c[0] > p[0] or c[1] > p[1]:
        return None

    left, right = search(p, a, b, an + 1, bn), search(p, a, b, an, bn + 1)
    if left is None and right is None:
        return None
    if left is None:
        return right
    if right is None:
        return left
    return min(left, right)

#wins = [search(p, a, b, 0, 0) for a, b, p in data]
#print(sum(w for w in wins if w is not None))

# part 2

# leaving because it's interesting, but the range is still too large to solve for 

## solution if c mod gcd(a, b) = 0
#def has_solution(a, b, p):
#    g = math.gcd(a, b)
#    return p % g == 0
#
## extended euclidian method for finding gcd
## we extract benzout's identity from it, giving us ax' + bx' = gcd(a, b)
#def euclidian(a, b):
#    (old_r, r) = (a, b)
#    (old_s, s) = (1, 0)
#    (old_t, t) = (0, 1)
#    
#    while r != 0:
#        quotient = old_r // r
#        (old_r, r) = (r, old_r - quotient * r)
#        (old_s, s) = (s, old_s - quotient * s)
#        (old_t, t) = (t, old_t - quotient * t)
#    
#    return {
#        'bezout': (old_s, old_t),
#        'gcd': old_r
#    }
#
## the t-th solution to our diophantine equation, given by the formula
## x = x0 + bt/gcd(a,b), y = y0 - at/gcd(a, b)
#def x_t(x0, y0, a, b, t):
#    d = math.gcd(a, b)
#    return x0 + t * b / d, y0 - t * a / d
#
# 
## linear diophantine equations can be solved by the euclidian extended method
## which yields a(sk) + b(tk) = c as long as gcd(a, b)|c
## now we have to find k making both x and y in the original equation positive
#def solve(a, b, p):
#    if not has_solution(a, b, p):
#        return None
#
#    result = euclidian(a, b)
#    d = result['gcd']
#    k = p / d # scale factor
#    x0, y0 = np.array(result['bezout']) * k
#
#    # solutions given by x = x0 + bt/gcd(a,b) and y = y0 - at/gcd(a,b), x > 0 and y > 0
#    # gives -x0 * gcd(a, b) / b < t < y0 * gcd(a,b) / a, integers so can floor and ceiling, respectively
#    # and we scale the whole thing by k, to take it from the space gcd(a,b) to our target number
#    x_lower, x_upper = math.floor(-1 * x0 * d / b), math.ceil(y0 * d / a)
#    print(x_lower, x_upper)
#
#    return [x_t(x0, y0, a, b, i) for i in range(x_lower, x_upper)]
#
## now that we have solution equations, we need to find all positive solutions of
## the first equation, then test them in the second equation, and take the
## minimum result by number of tokens
#
#total_tokens = 0
#i = 0
#for a, b, p in data:
#    print(i)
#    i += 1
#    ax1, ax2 = a
#    by1, by2 = b
#    p1, p2 = np.array(p) + 10000000000000
#
#    if not has_solution(ax1, by1, p1) or not has_solution(ax2, by2, p2):
#        continue
#
#    # find the solutions that apply to both equations
#    solns = np.array([s for s in solve(ax1, by1, p1) if s is not None and ax2 * s[0] + by2 * s[1] == p2]) 
#    if solns.size == 0:
#        continue # no solution to both
#    print(solns.size)
#    # scale the solutions by the number of tokens
#    solns = solns * [3, 1]
#    total_tokens += min(sum(s) for s in solns)
#
#print(total_tokens)

# duh....the lines intersect because we aren't solving a single equation
def intersection(a, b, c):
    A = np.array([[a[0], b[0]],
                  [a[1], b[1]]])
    C = np.array([c[0], c[1]])
    
    # Check if determinant is zero (lines are parallel)
    if np.linalg.det(A) == 0:
        return None  # Lines are parallel, no intersection
    
    # Solve the system of equations
    x, y = np.linalg.solve(A, C)
    return np.array((x, y))

def has_solution(a, b, p):
    g = math.gcd(a, b)
    return p % g == 0

total_tokens = 0
for a, b, p in data:
    print(a, b, p)
    a, b = np.array(a), np.array(b)
    p = np.array(p) + 10000000000000
    if not has_solution(a[0], b[0], p[0]) or not has_solution(a[1], b[1], p[1]):
        continue
    soln = np.round(intersection(a, b, p))

    # rounding issues....
    if soln is None or not np.all(soln[0] * a + soln[1] * b == p):
        continue
    total_tokens += sum(np.array(soln) * [3, 1])

print(total_tokens)
