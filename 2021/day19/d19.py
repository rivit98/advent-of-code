from collections import Counter
from itertools import starmap
from operator import add, sub

scanners = open("input.txt").read().strip().split('\n\n')
scanners = list(map(lambda d: list(map(lambda t: tuple(map(int, t.split(','))), d.split('\n')[1:])), scanners))

def identity(x, y, z): return x, y, z
def rot90z(x, y, z): return y, -x, z
def rot90y(x, y, z): return z, y, -x
def rot90x(x, y, z): return x, z, -y

rotation_scheme = [
    identity,
    rot90z, rot90z, rot90z, rot90z,
    rot90y,
    rot90z, rot90z, rot90z, rot90z,
    rot90y,
    rot90z, rot90z, rot90z, rot90z,
    rot90y,
    rot90z, rot90z, rot90z, rot90z,
    rot90y,

    rot90x,
    rot90z, rot90z, rot90z, rot90z,
    rot90x, rot90x,
    rot90z, rot90z, rot90z, rot90z,
]

class Scanner:
    id = 0

    def __init__(self, beacons):
        self.id = Scanner.id
        self.beacons = beacons
        self.possibs = self.generate_possibilities()
        Scanner.id += 1
        self.pos = (0, 0, 0)

    def __repr__(self):
        return f's{self.id} {self.pos}'

    def generate_possibilities(self):
        out = []
        pos = self.beacons
        for r in rotation_scheme:
            pos = list(starmap(r, pos))
            out.append(pos)
        return out

def tuple_sub(t1, t2): return tuple(map(sub, t1, t2))

def tuple_add(t1, t2): return tuple(map(add, t1, t2))

def dist(t1, t2): return sum([abs(p1 - p2) for p1, p2 in zip(t1, t2)])

beacons = set()

def pair_scanners(scanner):
    for possib in scanner.possibs:
        c = Counter()
        for ps1 in beacons:
            for ps2 in possib:
                diff = tuple_sub(ps1, ps2)
                c.update([diff])

        [(recovered_pos, count)] = c.most_common(1)
        if count < 12: continue

        new_beacons = list(map(lambda t: tuple_add(recovered_pos, t), possib))
        scanner.beacons = new_beacons
        scanner.pos = recovered_pos

        beacons.update(new_beacons)
        return scanner


scanners = list(map(Scanner, scanners))
unmatched = scanners[1:]
found_scanners = [scanners[0]]
beacons.update(scanners[0].beacons)
while unmatched:
    for un in unmatched:
        found_scanner = pair_scanners(un)
        if found_scanner:
            found_scanners.append(found_scanner)
            unmatched.remove(un)

# one star
print(len(beacons))

# two stars
max_dist = -1
for s1 in found_scanners:
    for s2 in found_scanners:
        if s1.id == s2.id: continue
        max_dist = max(max_dist, dist(s1.pos, s2.pos))

print(max_dist)
