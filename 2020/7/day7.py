import re


def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    p = re.compile(r'^([\w ]+) bags')
    p2 = re.compile(r'^([ \d]*) ([\w ]+) bag')

    LOOK_FOR = 'shiny gold'
    data = list(map(lambda s: s.split(' contain '), data))
    empty_bags_data = list(filter(lambda s: 'no other' in s[1], data))
    empty_bags_data = list(map(lambda c: (c[0], []), empty_bags_data))
    data = list(filter(lambda s: 'no other' not in s[1], data))

    def mapper(s):
        return p.match(s[0]).group(1), [(int(p2.match(token).group(1)), p2.match(token).group(2)) for token in
                                        s[1].split(',')]

    data = list(map(mapper, data))

    # 1 star
    colors = {LOOK_FOR}
    colors_len = 0
    while colors_len != len(colors):
        colors_len = len(colors)
        for pair in data:
            if any(x[1] in colors for x in pair[1]):
                colors.add(pair[0])

    print(len(colors) - 1)

    # 2 star
    data.extend(empty_bags_data)

    def count_bags(color):
        sum = 0

        for bag, contained_bags in data:
            if bag == color:
                for count, cbag in contained_bags:
                    sum += count * count_bags(cbag)

        return 1 + sum


    print(count_bags('shiny gold')-1)


if __name__ == '__main__':
    main()
