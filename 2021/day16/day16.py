from dataclasses import dataclass
from dataclasses import field
from functools import reduce
from typing import List
from typing import Optional

number = open("input.txt").read().strip()
binnum = bin(int(number, 16))[2:]
padding = len(number) * 4 - len(binnum)
binnum = padding * '0' + binnum


def extract(binnum, start, stop):
    return int(binnum[start:stop], 2)

def default_field(obj):
    return field(default=obj)

class Packet: pass

@dataclass
class Packet:
    version: int
    id: int
    value: Optional[int] = default_field(None)
    plen: Optional[int] = default_field(0)
    child: List[Packet] = field(default_factory=list)


def parse_packet(binnum):
    version = extract(binnum, 0, 3)
    id = extract(binnum, 3, 6)

    if id == 4:  # simple packet
        v = 0
        packet_len = 6
        for i in range(packet_len, len(binnum), 5):
            group = extract(binnum, i, i + 5)
            v <<= 4
            v |= group & 0b1111
            packet_len += 5
            if group & 0b10000 == 0: break

        return Packet(version, id, value=v, plen=packet_len)

    i = extract(binnum, 6, 7)
    if i == 0:  # next field is total len
        l = extract(binnum, 7, 7 + 15)
        parent = Packet(version, id)
        packets_len = 0
        ret = []
        parent.plen = 7 + 15
        while packets_len != l:
            p = parse_packet(binnum[parent.plen + packets_len:])
            ret.append(p)
            packets_len += p.plen

        parent.child = ret
        parent.plen += packets_len
        return parent
    elif i == 1:  # next field contains number of subpackets
        l = extract(binnum, 7, 7 + 11)

        parent = Packet(version, id)
        last_len = 0
        for _ in range(l):
            p = parse_packet(binnum[7 + 11 + last_len:])
            parent.child.append(p)
            last_len += p.plen

        parent.plen = 7 + 11 + sum([p.plen for p in parent.child])
        return parent


def flat_tree(parent):
    r = [parent]
    for child in parent.child:
        r.extend(flat_tree(child))
    return r

op_map = {
    0: lambda *x: sum(x),
    1: lambda *x: reduce(lambda acc, y: acc * y, x),
    2: lambda *x: min(x),
    3: lambda *x: max(x),
    5: lambda *x: int(x[0] > x[1]),
    6: lambda *x: int(x[0] < x[1]),
    7: lambda *x: int(x[0] == x[1]),
}

def calc_tree(node):
    if len(node.child) == 0: return node.value
    return op_map[node.id](*[calc_tree(child) for child in node.child])


packet_tree = parse_packet(binnum)

# one star
print(sum(map(lambda p: p.version, flat_tree(packet_tree))))

# two stars
print(calc_tree(packet_tree))
