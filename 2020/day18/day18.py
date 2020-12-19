with open("./input.txt") as f:
    data = f.read().splitlines()

data = list(filter(lambda x: x != ' ', list(data)))

def bracketize(expression: list):
    try:
        pos = expression.index('*', 0)
        br = bracketize(expression[pos+1:])
        return ['(', *expression[:pos], ')', '*', '(', *br, ')']
    except:  # * not found
        return expression


def calc_bracket(bracket, prioritize_addition):
    if prioritize_addition:
        bracketized = bracketize(bracket)
        val = calculate_expression(bracketized)
        return val

    stack = []
    for o in bracket:
        stack.append(o)
        if len(stack) == 3:
            v1, op, v2 = stack.pop(), stack.pop(), stack.pop()
            if op == '+':
                stack.append(v1 + v2)
            elif op == '*':
                stack.append(v1 * v2)

    return stack[0]


def calculate_expression(expression, prioritize_addition=False):
    token_idx, bracket_list, bracket_stack = 0, [], []
    while True:
        if token_idx < len(expression):
            c: str = expression[token_idx]
            token_idx += 1
        else:  # we should have flat list of operands here
            return calc_bracket(bracket_list, prioritize_addition)

        if str(c).isnumeric():
            bracket_list.append(int(c))
        elif c == '(':
            bracket_stack.append(bracket_list)
            bracket_list = []
        elif c in ['+', '*']:
            bracket_list.append(c)
        elif c == ')':
            v = calc_bracket(bracket_list, prioritize_addition)
            bracket_list = bracket_stack.pop()
            bracket_list.append(v)


print(sum([calculate_expression(exp) for exp in data]))
print(sum([calculate_expression(exp, True) for exp in data]))
