from collections import Counter

def mapper(s):
    p1, _, p2 = s.split()
    return *map(int, p1.split(',')), *map(int, p2.split(','))

lines = open("input.txt").read().strip().splitlines()
lines = map(mapper, lines)

straight, crossed = [], []
for t in lines:
    (crossed, straight)[t[0] == t[2] or t[1] == t[3]].append(t)

# one star
def range_line(start, stop):
    return range(start, stop+1) if start < stop else range(stop, start+1)

points = Counter()
for x1, y1, x2, y2 in straight:
    points.update([(i, j) for i in range_line(x1, x2) for j in range_line(y1, y2)])

print(len({x for x, count in points.items() if count > 1}))

# two stars
def range_crossed(x1, y1, x2, y2):
    dx = x1-x2
    dy = y1-y2
    mov_unit = (-dx//abs(dx), -dy//abs(dy))

    next_point = (x1, y1)
    yield next_point
    while next_point != (x2, y2):
        px, py = next_point
        mx, my = mov_unit
        next_point = (px+mx, py+my)
        yield next_point

for t in crossed:
    points.update(range_crossed(*t))

print(len({x for x, count in points.items() if count > 1}))
