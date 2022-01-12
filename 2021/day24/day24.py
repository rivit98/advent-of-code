from z3 import Ints, Optimize

def create_optimizer():
    s = Optimize()

    input = Ints(' '.join(map(lambda s: str(s), range(14))))
    for i in input:
        s.add(i>0, i<10)

    data = [
        (1, 11, 16),
        (1, 12, 11),
        (1, 13, 12),
        (26, -5, 12),
        (26, -3, 12),
        (1, 14, 2),
        (1, 15, 11),
        (26, -16, 4),
        (1, 14, 12),
        (1, 15, 9),
        (26, -7, 10),
        (26, -11, 11),
        (26, -6, 6),
        (26, -11, 15),
    ]

    x,y,z = 0,0,0
    for (zz,b,c),i in zip(data, input):
        w = i
        x = z % 26
        z /= zz
        x += b
        x = x != w
        y = 25
        y *= x
        y += 1
        z *= y
        y = w + c
        y *= x
        z += y

    s.add(z == 0)

    num = 0
    for i in input:
        num *= 10
        num += i

    return s, num


def check_solution(optimizer):
    optimizer.check()
    m = optimizer.model()

    out = []
    for i in sorted(m, key=lambda v: int(str(v))):
        out.append(m[i].as_long())

    print(''.join(map(str, out)))


s, num = create_optimizer()
s.maximize(num)
check_solution(s)

s, num = create_optimizer()
s.minimize(num)
check_solution(s)




