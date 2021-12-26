from itertools import permutations
from math import ceil, floor


class Node:
    def __init__(self, level, ch1, ch2):
        self.level = level
        self.left = ch1
        self.right = ch2
        self.parent = None

    def is_simple_node(self):
        return is_int(self.left) and is_int(self.right)

    def __repr__(self):
        return f'[{self.left},{self.right}]'

    def magnitude(self):
        mag = 0
        if is_int(self.left):
            mag += 3 * self.left
        else:
            mag += 3 * self.left.magnitude()

        if is_int(self.right):
            mag += 2 * self.right
        else:
            mag += 2 * self.right.magnitude()

        return mag


def is_node(v): return isinstance(v, Node)


def is_int(v): return isinstance(v, int)


def parse_snail(d):
    stack = []
    current_level = 0
    for c in filter(lambda s: s not in list(', '), d):
        if c == '[':
            current_level += 1

        elif c == ']':
            v1 = stack.pop()
            v2 = stack.pop()
            n = Node(current_level, v2, v1)
            if is_node(v1): v1.parent = n
            if is_node(v2): v2.parent = n
            stack.append(n)
            current_level -= 1

        else:
            stack.append(int(c))

    return stack[0]


def inc_level(p):
    if not is_node(p): return
    p.level += 1
    inc_level(p.left)
    inc_level(p.right)


def add_snails(s1, s2):
    s = Node(0, s1, s2)
    s1.parent = s
    s2.parent = s
    inc_level(s)
    return s


def explode_left(p, v):
    t = p.parent
    if is_int(t.left):
        t.left += v
        return

    prev = p
    while t and prev == t.left:
        prev = t
        t = t.parent

    if t is None: return

    if is_int(t.left):
        t.left += v
        return

    t = t.left
    while is_node(t.right):
        t = t.right

    t.right += v


def explode_right(p, v):
    t = p.parent
    if is_int(t.right):
        t.right += v
        return

    prev = p
    while t and prev == t.right:
        prev = t
        t = t.parent

    if t is None: return

    if is_int(t.right):
        t.right += v
        return

    t = t.right
    while is_node(t.left):
        t = t.left

    t.left += v


def explode_node(n: Node):
    if n is None: return
    p = n.parent

    explode_right(n, n.right)
    explode_left(n, n.left)

    if p.left == n:
        p.left = 0
    else:
        p.right = 0


def split_node(n: Node):
    if n is None: return

    if is_int(n.left) and n.left > 9:
        v = n.left
        n.left = Node(n.level + 1, floor(v / 2), ceil(v / 2))
        n.left.parent = n
        return

    if is_int(n.right) and n.right > 9:
        v = n.right
        n.right = Node(n.level + 1, floor(v / 2), ceil(v / 2))
        n.right.parent = n
        return


def check_for_explode(p):
    if is_int(p): return None
    if is_node(p) and p.level == 5: return p

    for_explode = check_for_explode(p.left)
    if for_explode: return for_explode
    return check_for_explode(p.right)


def check_for_split(p):
    if is_int(p.left) and is_int(p.right):
        if p.left > 9 or p.right > 9: return p
        return None
    elif is_int(p.left):
        if p.left > 9: return p
        return check_for_split(p.right)
    elif is_int(p.right):
        found = check_for_split(p.left)
        if found: return found
        if p.right > 9: return p
        return None
    else:
        found = check_for_split(p.left)
        if found: return found
        found = check_for_split(p.right)
        if found: return found

    return None


def reduce_snail(snail):
    while True:
        found = check_for_explode(snail)
        if found:
            explode_node(found)
            continue

        split = check_for_split(snail)
        if split:
            split_node(split)

        if not found and not split:
            break

    return snail


def add_list_of_snails(list_snails):
    snails = list(map(parse_snail, list_snails))
    final_sum = snails[0]
    for s in snails[1:]:
        final_sum = add_snails(final_sum, s)
        final_sum = reduce_snail(final_sum)

    return final_sum


snails = open('input.txt').read().strip().splitlines()

# one star
print(add_list_of_snails(snails).magnitude())

# two stars
max_mag = -1
for c in permutations(snails, 2):
    m = add_list_of_snails(list(c))
    max_mag = max(max_mag, m.magnitude())

print(max_mag)
