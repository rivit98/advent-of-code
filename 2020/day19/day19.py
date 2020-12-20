with open("./input.txt") as f:
    data = f.read().splitlines()

raw_rules = data[:data.index('')]
words = data[data.index('') + 1:]

rules = []
for rule in raw_rules:
    rule_nr, prods = rule.split(':')
    if '"' in prods or "'" in prods:
        rules.append((int(rule_nr), prods.strip().replace('"', '').replace("'", '')))
    else:
        for p in prods.split('|'):
            rules.append((int(rule_nr), tuple(map(int, p.strip().split(' ')))))

rules = sorted(rules, key=lambda x: x[0])


class Grammar:
    def __init__(self, rules):
        self.rules = rules
        self.to_cnf()
        self.non_terminals = self.setup_productions(False)
        self.terminals = self.setup_productions(True)

    def setup_productions(self, terminals):
        out = []
        for rule in self.rules:
            rn, prods = rule
            if terminals and isinstance(prods, str):
                out.append(rule)
            elif not terminals and isinstance(prods, tuple):
                out.append(rule)

        return out

    def to_cnf(self):
        self.term_step()
        self.unit_step()
        self.non_terminals = self.setup_productions(False)
        self.terminals = self.setup_productions(True)

    def term_step(self):
        max_number = max(self.rules, key=lambda x: x[0])[0]
        to_remove = []
        new_rules = []
        for rule in self.rules:
            rn, prods = rule
            if len(prods) < 3:
                continue

            nr_list = self.shorten_rule(rule, max_number)
            max_number += len(nr_list)
            to_remove.append(rule)
            new_rules.extend(nr_list)

        for rule in to_remove:
            self.rules.remove(rule)

        for rule in new_rules:
            self.rules.append(rule)

    def unit_step(self):
        result = []
        unit_rules = []
        for rn, prods in self.rules:
            if isinstance(prods, tuple) and len(prods) == 1:
                unit_rules.append((rn, prods))
            else:
                result.append((rn, prods))

        while unit_rules:
            u_rule = unit_rules.pop(0)
            un, uprod = u_rule
            target_rules = list(filter(lambda x: x[0] == uprod[0], self.rules))
            for _, prods in target_rules:
                new_rule = (un, prods)
                if len(prods) != 1 or isinstance(prods, str):
                    result.append(new_rule)
                else:
                    unit_rules.append(new_rule)

        self.rules = result

    def shorten_rule(self, rule, last_nr):
        rn, prods = rule
        if len(prods) < 3:
            return [rule]

        new_rule = last_nr + 1
        return [(rn, (prods[0], new_rule))] + self.shorten_rule((new_rule, prods[1:]), new_rule)


class CYK:
    def __init__(self, rules):
        self.grammar = Grammar(rules)
        self.terminals = self.grammar.terminals
        self.var0 = [va[0] for va in self.grammar.non_terminals]
        self.var1 = [va[1] for va in self.grammar.non_terminals]

    def merge_sets(self, first, second):
        if len(first) == 0 or len(second) == 0:
            return {}

        return {(f, s) for f in first for s in second}

    def check_word(self, input):
        length = len(input)
        table = [[set() for _ in range(length - i)] for i in range(length)]  # create cyk table

        for i in range(length):  # fill first row
            for te in self.terminals:
                if input[i] == te[1]:
                    table[0][i].add(te[0])

        for i in range(1, length):
            for j in range(length - i):
                for k in range(i):
                    row = self.merge_sets(table[k][j], table[i - k - 1][j + k + 1])
                    for idx, v in enumerate(self.var1):
                        if v in row:
                            table[i][j].add(self.var0[idx])

        return 0 in table[-1][-1]


cyk = CYK(rules)
print(sum([cyk.check_word(word) * 1 for word in words]))

rules.append((8, (42, 8)))
rules.append((11, (42, 11, 31)))
cyk = CYK(rules)
print(sum([cyk.check_word(word) * 1 for word in words]))
