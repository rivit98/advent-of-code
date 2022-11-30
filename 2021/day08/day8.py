from collections import defaultdict, Counter

lines = open("input.txt").read().strip().splitlines()
data = list(map(lambda s: list(map(lambda x: x.strip().split(' '), s.split('|'))), lines))

# one star
print(sum([sum([len(n) in [2, 4, 3, 7] for n in digits]) for _, digits in data]))

# two stars
digits_mapping = {
    1: 'cf',
    7: 'acf',
    4: 'bcdf',
    2: 'acdeg',
    3: 'acdfg',
    5: 'abdfg',
    0: 'abcefg',
    6: 'abdefg',
    9: 'abcdfg',
    8: 'abcdefg',
}
digits_mapping = {k: ''.join(sorted(v)) for k, v in digits_mapping.items()}
rev_digits_mapping = {v: k for k, v in digits_mapping.items()}
m = {k: len(v) for k, v in digits_mapping.items()}

class Solver:
    def __init__(self, mixed):
        self.recovered_mapping = {}
        self.seg_num_to_digits = defaultdict(list)
        self.all = mixed
        for m in self.all:
            self.seg_num_to_digits[len(m)].append(set(m))

        self.solve_segment_by_count('e', 4)
        self.solve_segment_by_count('b', 6)
        self.solve_segment_by_count('f', 9)
        self.solve_a_segment()
        self.solve_cf_segments()
        self.solve_dg_segment()
        self.recovered_mapping = {v: k for k, v in self.recovered_mapping.items()}

    def to_num(self, shuffled):
        res = 0
        for digit in shuffled:
            res *= 10
            res += rev_digits_mapping[''.join(sorted(map(lambda l: self.recovered_mapping[l], digit)))]

        return res

    def extract(self, num):
        return self.seg_num_to_digits[m[num]]

    def extract_one(self, num):
        return self.seg_num_to_digits[m[num]][0]

    def get(self, a, b):
        return list(a - b)[0]

    def solve_a_segment(self):
        one = self.extract_one(1)
        seven = self.extract_one(7)
        self.recovered_mapping['a'] = self.get(seven, one)

    def solve_cf_segments(self):
        eight = self.extract_one(8)
        one = self.extract_one(1)
        six_seg = self.extract(9)
        for ss in six_seg:
            diff = eight - ss
            if diff & one:
                self.recovered_mapping['c'] = self.get(diff, set())

        # self.recovered_mapping['f'] = self.get(one, set(self.recovered_mapping['c']))

    def solve_segment_by_count(self, seg, total):
        cnt = Counter(''.join(self.all))
        rev_mapping = {v: k for k, v in cnt.items()}
        self.recovered_mapping[seg] = rev_mapping[total]

    def solve_dg_segment(self):
        four = self.extract_one(4)
        eight = self.extract_one(8)
        self.recovered_mapping['d'] = self.get(four, set(self.recovered_mapping.values()))
        self.recovered_mapping['g'] = self.get(eight, set(self.recovered_mapping.values()))


print(sum([Solver(mixed).to_num(to_decode) for mixed, to_decode in data]))
