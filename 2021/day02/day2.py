mov_dict = {
    'forward': (1, 0),
    'down': (0, 1),
    'up': (0, -1)
}

def mapper(s):
    tokens = s.split(' ')
    return tuple(int(tokens[1]) * m for m in mov_dict[tokens[0]])

data = open("input.txt").read().strip().splitlines()
data = list(map(mapper, data))

# one star
x, d = map(sum, zip(*data))
print(x * d)

# two stars
a, x, d = 0, 0, 0
for df, dd in data:
    x += df
    a += dd
    d += a * df

print(x * d)