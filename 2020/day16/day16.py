import re
from collections import defaultdict
from itertools import groupby
from functools import reduce

with open("./input.txt", "rt") as f:
    data = f.read().splitlines()

chunks = [list(group) for k, group in groupby(data, key=lambda s: s == '') if not k]
conditions, my_ticket, tickets = chunks
my_ticket = list(map(int, my_ticket[1].split(',')))
p1 = re.compile(r"(\d+-\d+)")
conditions = list(map(lambda cond: list(map(lambda c: tuple(map(lambda s: int(s), c.split('-'))), p1.findall(cond))), conditions))
tickets = list(map(lambda ticket: list(map(int, ticket.split(','))), tickets[1:]))


def validate_condition(c, v):
    a, b = c
    return a[0] <= v <= a[1] or b[0] <= v <= b[1]


def validate_ticket(ticket):
    for v in ticket:
        if all([not validate_condition(c, v) for c in conditions]):
            return False, v

    return True, 0


s = 0
valid_tickets = []
for ticket in tickets:
    valid, value = validate_ticket(ticket)
    if not valid:
        s += value
    else:
        valid_tickets.append(ticket)

print(s)

# 2 star
tickets = valid_tickets
tickets = list(map(list, zip(*tickets)))

possibs = defaultdict(list)
for i, c in enumerate(conditions):
    for j, row in enumerate(tickets):
        if all([validate_condition(c, v) for v in row]):
            possibs[j].append(i)

final = {}  # k - condition number, v - field number
while len(possibs):
    for k, v in possibs.items():
        if len(v) == 1:
            to_remove = (k, v[0])
            final[v[0]] = k
            break

    del possibs[to_remove[0]]

    for k, v in possibs.items():
        try:
            v.remove(to_remove[1])
        except:
            pass


product = reduce(lambda a, i: a * my_ticket[final[i]], range(6), 1)
print(product)
