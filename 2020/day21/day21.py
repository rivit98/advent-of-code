import re


class Recipe:
    def __init__(self, i, a):
        self.ingredients = set(i)
        self.alergens = set(a)


with open("./input.txt") as f:
    data = f.read().splitlines()

    lines = []
    for line in data:
        matches = re.findall(r'(\w+)', line)
        split_word = matches.index('contains')
        lines.append(Recipe(matches[:split_word], matches[split_word + 1:]))

alergens = {}
for line in lines:
    for alg in line.alergens:
        if alg not in alergens:
            alergens[alg] = line.ingredients.copy()
        else:
            alergens[alg] &= line.ingredients

mapping = {}
while len(alergens) != 0:
    to_remove = None
    for k, v in alergens.items():
        if len(v) == 1:
            to_remove = (k, v)
            mapping[k] = list(v)[0]

    del alergens[to_remove[0]]

    for k, v in alergens.items():
        alergens[k] -= to_remove[1]

contains_alergens = set(mapping.values())
print(sum([len(line.ingredients - contains_alergens) for line in lines]))

# 2 star
print(','.join(map(lambda x: x[1], sorted(mapping.items(), key=lambda x: x[0]))))


