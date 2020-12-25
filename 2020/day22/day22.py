with open("./input.txt") as f:
    data = f.read()

p1, p2 = data.split('\n\n')

p1 = list(map(int, p1.split('\n')[1:]))
p2 = list(map(int, list(filter(lambda s: s != '', p2.split('\n')[1:]))))

deck1, deck2 = p1.copy(), p2.copy()
while len(deck1) != 0 and len(deck2) != 0:
    c1, c2 = deck1.pop(0), deck2.pop(0)
    if c1 > c2:
        deck1.extend([c1, c2])
    else:
        deck2.extend([c2, c1])

winner = max([deck1, deck2], key=len)
print(sum([a * b for a, b in zip(winner, reversed(range(1, len(winner) + 1)))]))

# 2 star
def combat(deck1, deck2):
    history = set()
    while len(deck1) != 0 and len(deck2) != 0:
        winner_id = None
        if (tuple(deck1), tuple(deck2)) in history:
            return 0, deck1  # player 1
        history.add((tuple(deck1), tuple(deck2)))

        c1, c2 = deck1.pop(0), deck2.pop(0)
        if c1 <= len(deck1) and c2 <= len(deck2):
            winner_id, _ = combat(deck1[:c1].copy(), deck2[:c2].copy())

        if winner_id == 0 or (winner_id is None and c1 > c2):
            deck1.extend([c1, c2])
        else:
            deck2.extend([c2, c1])

    return (1, deck2) if len(deck1) == 0 else (0, deck1)


_, winner = combat(p1.copy(), p2.copy())
print(sum([a * b for a, b in zip(winner, reversed(range(1, len(winner) + 1)))]))
