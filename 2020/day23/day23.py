with open("./input.txt") as f:
    data = f.read().splitlines()

data = list(map(int, list(data[0])))

class Node:
    def __init__(self, v):
        self.next = None
        self.value = v

    def __repr__(self):
        return str(self.value)


class CircularLinkedList:
    def __init__(self, data):
        self.nodes = {}  # value - node mapping
        nodes = [Node(v) for v in data]
        for i, v in enumerate(data):
            nodes[i].next = nodes[(i + 1) % len(data)]
            self.nodes[nodes[i].value] = nodes[i]


def get_insert_index(current_cup, selected_cups):
    selected_values = list(map(lambda x: x.value, selected_cups))
    target_value = current_cup.value
    while target_value in selected_values or target_value == current_cup.value:
        target_value -= 1
        if target_value <= 0:
            target_value = len(data)

    return clist.nodes[target_value]

def solve(steps):
    current_cup = clist.nodes[data[0]]
    for i in range(steps):
        selected_cups = [current_cup.next, current_cup.next.next, current_cup.next.next.next]
        destination = get_insert_index(current_cup, selected_cups)

        current_cup.next = selected_cups[-1].next
        selected_cups[-1].next = destination.next
        destination.next = selected_cups[0]

        current_cup = current_cup.next


clist = CircularLinkedList(data)
solve(100)
one_node = clist.nodes[1]
answer = ""
while one_node.next != clist.nodes[1]:
    answer += str(one_node.next.value)
    one_node = one_node.next

print(answer)


data = data + list(range(len(data)+1, 1000001))
clist = CircularLinkedList(data)
solve(10000000)
one_node = clist.nodes[1]
print(one_node.next.value * one_node.next.next.value)
