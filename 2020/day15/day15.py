with open("./input.txt", "rt") as f:
    data = f.read().splitlines()[0]
    data = list(map(int, data.split(',')))

occurences = {v: i for i, v in enumerate(data)}
while len(data) < 30000000:
    last = data[-1]
    to_add = 0
    v = occurences.get(last)
    if v is not None:
        to_add = len(data)-v-1

    occurences[last] = len(data)-1
    data.append(to_add)

print(data[2020-1])
print(data[30000000-1])
