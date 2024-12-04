import re


with open('day3.problem.txt', 'r') as f:
    data = f.read().replace('\\n', '')

mul_re = re.compile(r'mul\((\d{1,3}),(\d{1,3})\)')

print(sum(int(a) * int(b) for a, b in mul_re.findall(data)))

# part 2
mul_re = re.compile(r'(do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\))')


enabled = 1
s = 0
for w, a, b in mul_re.findall(data):
    if w.startswith("don't"):
        enabled = 0
    elif w.startswith("do"):
        enabled = 1
    else:
        s += int(a) * int(b) * enabled

print(s)
