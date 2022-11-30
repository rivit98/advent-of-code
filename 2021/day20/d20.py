from collections import defaultdict
from copy import copy

translator, image = open("input.txt").read().strip().split('\n\n')
image = image.split('\n')

image_map = defaultdict(lambda: 0)
for i, v in enumerate(image):
    for j, p in enumerate(v):
        image_map[(i, j)] = '.#'.index(p)


def get_val(image_map, pos):
    x, y = pos
    vals = []
    for i in range(-1, 2, 1):
        for j in range(-1, 2, 1):
            nx, ny = x + i, y + j
            vals.append(image_map[(nx, ny)])

    return int(''.join(map(str, vals)), 2)


def pad_image(iters, img_data):
    PAD_SIZE = iters
    keys = img_data.keys()
    min_key, max_key = min(keys), max(keys)
    minx, miny = min_key
    maxx, maxy = max_key
    for i in range(minx - PAD_SIZE, maxx + 1 + PAD_SIZE):
        for j in range(1, 1 + PAD_SIZE):
            img_data[(i, maxy + j)] = 0
            img_data[(i, miny - j)] = 0

    for i in range(miny - PAD_SIZE, maxy + 1 + PAD_SIZE):
        for j in range(1, 1 + PAD_SIZE):
            img_data[(minx - j, i)] = 0
            img_data[(maxx + j, i)] = 0

    return img_data


def enhance(ITERS, image_map):
    image_map = pad_image(ITERS, image_map)

    for nr in range(ITERS):
        if translator[0] == '#':  # we have to flip bg every iteration
            image_map.default_factory = [
                lambda: 0,
                lambda: 1
            ][nr % 2]

        new_image = copy(image_map)
        for k in list(image_map.keys()):
            pix_val = get_val(image_map, k)
            new_image[k] = '.#'.index(translator[pix_val])

        image_map = new_image

    return len(list(filter(lambda v: v == 1, image_map.values())))

# one star
print(enhance(2, image_map))

# two stars
print(enhance(50, image_map))

