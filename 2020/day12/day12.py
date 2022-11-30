
with open("./input.txt", "rt") as f:
    data = f.read().splitlines()
    data = list(map(lambda s: (s[0], int(s[1:])), data))

dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
ship_x, ship_y, ship_dir = 0, 0, 0
for c, v in data:
    if c == 'N':   ship_y += v
    elif c == 'S': ship_y -= v
    elif c == 'E': ship_x += v
    elif c == 'W': ship_x -= v
    elif c == 'L': ship_dir = (ship_dir + (v // 90)) % len(dirs)
    elif c == 'R': ship_dir = (ship_dir - (v // 90)) % len(dirs)
    elif c == 'F':
        d_x, d_y = tuple([x * v for x in dirs[ship_dir]])
        ship_x += d_x
        ship_y += d_y

print(abs(ship_x) + abs(ship_y))



ship_x, ship_y, ship_dir, waypoint_x, waypoint_y = 0, 0, 0, 10, 1
for c, v in data:
    if c == 'N':   waypoint_y += v
    elif c == 'S': waypoint_y -= v
    elif c == 'E': waypoint_x += v
    elif c == 'W': waypoint_x -= v
    elif c == 'L':
        for _ in range(v // 90):
            waypoint_x, waypoint_y = -waypoint_y, waypoint_x
    elif c == 'R':
        for _ in range(v // 90):
            waypoint_x, waypoint_y = waypoint_y, -waypoint_x
    elif c == 'F':
        d_x, d_y = tuple([x * v for x in [waypoint_x, waypoint_y]])
        ship_x += d_x
        ship_y += d_y

print(abs(ship_x) + abs(ship_y))