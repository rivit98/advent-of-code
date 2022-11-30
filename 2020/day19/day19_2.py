import nltk

with open("./input.txt") as f:
    data = f.read().splitlines()

raw_rules = data[:data.index('')]
words = data[data.index('') + 1:]

raw_rules = list(map(lambda s: s.replace(':', ' -> '), raw_rules))
raw_rules = sorted(raw_rules)


def count_valid():
    grammar = nltk.CFG.fromstring('\n'.join(raw_rules))
    parser = nltk.EarleyChartParser(grammar)

    cnt = 0
    for word in words:
        try:
            if len(list(parser.parse(word))):
                cnt += 1
        except:
            pass

    return cnt


print(count_valid())

# 2 star
raw_rules.append('8 -> 42 8')
raw_rules.append('11 -> 42 11 31')
print(count_valid())
