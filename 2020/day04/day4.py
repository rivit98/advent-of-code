import re
from itertools import groupby

def main():
    p = re.compile(r'^(\d+)')
    p2 = re.compile(r'^#[0-9a-f]{6}$')
    p3 = re.compile(r'^\d{9}$')

    def height_validator(x):
        ma = p.match(x)
        if 'cm' in x:
            return 150 <= int(ma.group(1)) <= 193
        elif 'in' in x:
            return 59 <= int(ma.group(1)) <= 76

        return False

    fields = {
        "byr": lambda x: (1920 <= int(x) <= 2002),
        "iyr": lambda x: (2010 <= int(x) <= 2020),
        "eyr": lambda x: (2020 <= int(x) <= 2030),
        "hgt": height_validator,
        "hcl": lambda x: p2.match(x),
        "ecl": lambda x: x in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
        "pid": lambda x: p3.match(x)
    }

    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    chunks = [
        sum(map(lambda x: list(filter(lambda s: 'cid' not in s, x.split(' '))), group), [])
        for k, group in groupby(data, lambda x: x == '') if not k
    ]

    def mapper(kv_list):
        return {k: v for k, v in list(map(lambda x: x.split(':'), kv_list))}

    chunks = list(map(mapper, chunks))

    # 1 star
    def validator1(d: dict):
        keys = list(d.keys())
        return all(v in keys for v in fields)

    validated = list(filter(validator1, chunks))
    print(len(validated))

    # 2 stars
    def validator2(d):
        for k, v in d.items():
            r = fields[k](v)
            if not r or r is False:
                return False

        return True

    print(len(list(filter(validator2, validated))))


if __name__ == '__main__':
    main()
