import math
from functools import lru_cache

data = open("input.txt").read().strip().split(':')[1].split(',')
xs = data[0].split('..')
ys = data[1].split('..')

xmin, xmax = int(xs[0].split('=')[1]), int(xs[1])
ymin, ymax = int(ys[0].split('=')[1]), int(ys[1])


# xmin <= (t * (2 * xspeed - t + 1)) / 2 <= xmax && xspeed > 0
# ymin <= (t * (2 * yspeed - t + 1)) / 2 <= ymax
# 0 <= xspeed <= xmax


def boundary(x, t):
    v = (2 * x + t * t - t) / (2 * t)
    return v

@lru_cache(maxsize=None)
def get_sum(xstart, t):
    return sum(filter(lambda x: x > 0, [xstart - i for i in range(t)]))

def boundary_x(xmin, xmax, t):
    ret = []
    for xstart in range(1, xmax + 1):
        if xmin <= get_sum(xstart, t) <= xmax:
            ret.append(xstart)

    return ret


all_possibs = set()
for t in range(1, 10000):  # cannot find a cap for time :(
    blow = boundary(ymin, t)
    bhi = boundary(ymax, t)

    possibs_y = list(range(math.ceil(blow), math.floor(bhi) + 1))
    for yspeed in possibs_y:
        possibs_x = boundary_x(xmin, xmax, t)
        for xspeed in possibs_x:
            # print(f't={t} {xspeed},{yspeed}')
            all_possibs.add((xspeed, yspeed))

# one star
yspeed_max = max(all_possibs, key=lambda t: t[1])
print(sum(range(yspeed_max[1] + 1)))

# two stars
print(len(all_possibs))
