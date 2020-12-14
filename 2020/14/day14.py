import re
from itertools import product

with open("./input.txt", "rt") as f:
    data = f.read().splitlines()

mem_pattern = re.compile(r"mem\[(\d+)] = (\d+)")
def unpack(line):
    m = mem_pattern.match(line)
    return int(m.group(1)), int(m.group(2))


def apply_mask(val, mask):
    for m in mask:
        bit, v = m
        if v == '0':
            val &= ~(1 << bit)
        elif v == '1':
            val |= (1 << bit)

    return val


mem = {}
mask = []
for line in data:
    if 'mask' in line:
        mask = [(i, c) for i, c in enumerate(line.split(' = ')[1][::-1])]
    else:  # mem
        addr, val = unpack(line)
        mem[addr] = apply_mask(val, mask)

print(sum(mem.values()))


def apply_mask2(addr, mask):
    for m in mask:
        bit, v = m
        if v == '1':
            addr |= (1 << bit)
        elif v == 'X':
            addr &= ~(1 << bit)

    return [addr + sum(values) for values in product(*[(0, 2**b) for b, v in mask if v == 'X'])]

mem = {}
for line in data:
    if 'mask' in line:
        mask = [(i, c) for i, c in enumerate(line.split(' = ')[1][::-1])]
    else:  # mem
        addr, val = unpack(line)
        for a in apply_mask2(addr, mask):
            mem[a] = val

print(sum(mem.values()))
