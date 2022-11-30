import matplotlib.pyplot as plt

numbers, flips = open("input.txt").read().strip().split('\n\n')
numbers = list(map(lambda s: tuple(map(int, s.strip().split(','))), numbers.splitlines()))
flips = list(map(lambda t: t.split(' ')[2].split('=') , flips.splitlines()))
flips = list(map(lambda t: (t[0], int(t[1])), flips))

points = set(numbers)

part1 = None
for ax, fold_v in flips:
    new_points = set()
    for p in points:
        x, y = p
        if ax == 'x':
            if x > fold_v:
                new_points.add((fold_v-(x-fold_v), y))
            else:
                new_points.add(p)
        else:
            if y > fold_v:
                new_points.add((x, fold_v+(fold_v-y)))
            else:
                new_points.add(p)

    points = new_points
    if not part1:
        part1 = len(points)

# one star
print(part1)

# two stars
points = list(map(lambda t: (t[0], -int(t[1])), list(points)))
plt.ylim((-6, 30))
plt.scatter(*zip(*points))
plt.show()