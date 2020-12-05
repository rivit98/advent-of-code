import re

def main():
    p = re.compile(r"(\d+)-(\d+) (\w): (\w+)")
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    def mapper(s: str):
        a, b, c, password = p.search(s).groups()
        return int(a), int(b), c, password

    data = list(map(mapper, data))


    def onestar(tpl):
        a, b, letter, password = tpl
        return a <= password.count(letter) <= b

    print(len(list(filter(onestar, data))))

    # 2 stars
    def twostars(tpl):
        a, b, l, p = tpl
        return (p[a - 1] == l) ^ (p[b - 1] == l)

    print(len(list(filter(twostars, data))))


if __name__ == '__main__':
    main()
