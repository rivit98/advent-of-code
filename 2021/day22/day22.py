data = open("input.txt").read().strip().splitlines()

small_area, all_coords = [], []
for d in data:
    onoff, rest = d.split(' ')
    for c in 'xyz=':
        rest = rest.replace(c, '')
    coords = tuple(map(lambda t: tuple(map(int, t.split('..'))), rest.split(',')))
    if all(-50 <= x <= 50 for x in sum(coords, ())):
        small_area.append((onoff, coords))

    all_coords.append((onoff, coords))


def calc_v(coords):
    (xs, xe), (ys, ye), (zs, ze) = coords
    if xe < xs or ye < ys or ze < zs: return 0
    return (abs(xs - xe) + 1) * (abs(ys - ye) + 1) * (abs(zs - ze) + 1)


def calc_overlap(coords1, coords2):
    (xs, xe), (ys, ye), (zs, ze) = coords1
    (xs2, xe2), (ys2, ye2), (zs2, ze2) = coords2

    return (max(xs2, xs), min(xe2, xe)), (max(ys2, ys), min(ye2, ye)), (max(zs2, zs), min(ze2, ze))


def diff_cubes(coords1, overlap):
    (xs, xe), (ys, ye), (zs, ze) = coords1
    (xs2, xe2), (ys2, ye2), (zs2, ze2) = overlap

    def one_dir_diff(xs, xs2, xe, xe2):
        if (xs, xe) == (xs2, xe2):
            return [(xs2, xe2)]
        if xs == xs2:
            return [(xs2, xe2), (xe2 + 1, xe)]
        elif xe == xe2:
            return [(xs2, xe2), (xs, xs2 - 1)]
        else:
            return [(xs, xs2 - 1), (xs2, xe2), (xe2 + 1, xe)]

    ox = one_dir_diff(xs, xs2, xe, xe2)
    oy = one_dir_diff(ys, ys2, ye, ye2)
    oz = one_dir_diff(zs, zs2, ze, ze2)

    new_cubes = []
    for x_d in ox:
        for y_d in oy:
            for z_d in oz:
                t_d = (x_d, y_d, z_d)
                if t_d == overlap: continue
                new_cubes.append(t_d)

    return new_cubes


def count_cubes(parsed):
    splitted = []
    for onoff, coords in parsed:
        new_splitted = []
        for coords_dp in splitted:
            overlap = calc_overlap(coords, coords_dp)
            overlap_v = calc_v(overlap)
            if not overlap_v:
                new_splitted.append(coords_dp)
                continue

            sp = diff_cubes(coords_dp, overlap)
            new_splitted.extend(sp)

        if onoff == 'on':
            new_splitted.append(coords)

        splitted = new_splitted

    return sum(calc_v(coord) for coord in splitted)


# one star
print(count_cubes(small_area))

# two stars
print(count_cubes(all_coords))
