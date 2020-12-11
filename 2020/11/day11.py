with open("./input.txt", "rt") as f:
    data = f.read().splitlines()

w = len(data)
h = len(data[0])
data = ['.' * (w + 2)] + list(map(lambda row: '.' + row + '.', data)) + ['.' * (w + 2)]
directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]

def count_adjacent(d, x, y):
    return list(map(lambda dir: d[x+dir[0]][y+dir[1]], directions)).count('#')


def count_in_one_dir(d, x, y, d_x, d_y):
    n_x, n_y = x, y
    while True:
        n_x += d_x
        n_y += d_y
        if n_x < 0 or n_x >= w + 2 or n_y < 0 or n_y >= h + 2: return 0
        if d[n_x][n_y] == '.': continue
        if d[n_x][n_y] == '#': return 1
        if d[n_x][n_y] == 'L': return 0


def count_adjacent2(d, x, y):
    return sum([count_in_one_dir(d, x, y, *direction) for direction in directions])


def map_seat(d, x, y):
    c = count_adjacent(d, x, y)
    if d[x][y] == 'L' and c == 0:
        return '#'
    elif d[x][y] == '#' and c >= 4:
        return 'L'

    return d[x][y]


def map_seat2(d, x, y):
    c = count_adjacent2(d, x, y)
    if d[x][y] == 'L' and c == 0:
        return '#'
    elif d[x][y] == '#' and c >= 5:
        return 'L'

    return d[x][y]


def count_occupied_seats(mapping_function):
    sim_data = data
    cnt, prev_cnt = 0, -1
    while prev_cnt != cnt:
        prev_cnt = cnt
        empty_data = ['.' * (w + 2)]

        for i in range(1, w + 1):
            empty_data.append('.' + ''.join(map(lambda j: mapping_function(sim_data, i, j), list(range(1, h + 1)))) + '.')

        sim_data = empty_data + ['.' * (w + 2)]
        cnt = ''.join(sim_data).count('#')

    return cnt


print(count_occupied_seats(map_seat))
print(count_occupied_seats(map_seat2))
