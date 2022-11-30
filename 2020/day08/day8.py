def run_machine(code, changing_instr=False, has_instruction_changed=False):
    acc, pc = 0, 0
    count = {}
    while pc < len(code):
        instr, val = code[pc]
        c = count.get(pc, 0) + 1
        if c > 1:
            # we are changing instructions and reached infinite loop - terminate
            return None if changing_instr else acc

        count[pc] = c

        if instr == 'acc':
            acc += val
            pc += 1
        elif instr == 'jmp':
            if changing_instr and not has_instruction_changed:
                code[pc] = ('nop', val)
                ret = run_machine(code, True, True)
                if ret: return ret
                code[pc] = ('jmp', val)  # restore instr

            pc += val
        else:  # nop
            if changing_instr and not has_instruction_changed:
                code[pc] = ('jmp', val)
                ret = run_machine(code, True, True)
                if ret: return ret
                code[pc] = ('nop', val)  # restore instr

            pc += 1

    return acc


def main():
    with open("./input.txt", "rt") as f:
        data = f.read().splitlines()

    data = list(map(lambda s: s.split(' '), data))
    data = list(map(lambda s: (s[0], int(s[1])), data))

    print(run_machine(data))
    print(run_machine(data, True))


if __name__ == '__main__':
    main()
