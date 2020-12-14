from sympy.ntheory.modular import crt

with open("./input.txt", "rt") as f:
    data = f.read().splitlines()
    target_time = int(data[0])
    buses = list(map(lambda x: int(x), list(filter(lambda s: s != 'x', data[1].split(',')))))

next_buses = [((target_time // p) + 1) * p for p in buses]
diffs = list(map(lambda x: (x[0] - target_time) % x[1], zip(next_buses, buses)))
m = min(diffs)
print(buses[diffs.index(m)] * m)

modulo = []
value = []

for i, n in enumerate(data[1].split(',')):
    if n == 'x': continue
    n = int(n)
    modulo.append(n)
    value.append(n - i)

crt_m_v = crt(modulo, value)
print(crt_m_v[0])
