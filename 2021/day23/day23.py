data = open('input.txt').read().strip().splitlines()

def map_values(data_str):
    return list(map(lambda v: ord(v)-ord('A'), data_str.strip().replace('#', '')))

r = map_values(data[2])
r2 = map_values(data[3])
tpos = [[*x] for x in zip(r, r2)]
tpos2 = [[*x] for x in zip(r, map_values('D#C#B#A#'), map_values('D#B#A#C#'), r2)]


class Room:
    def __init__(self, id, column):
        self.room_id = id
        self.locked_idx = len(column)
        self.space = column  # 0 - top
        self.top_idx = 0

        was_successful = True
        for v in reversed(self.space):
            if not was_successful: break

            was_successful = self.room_id == v
            if was_successful:
                self.locked_idx -= 1

    @property
    def is_finished(self):
        return self.locked_idx == 0

    @property
    def can_be_entered(self):
        return self.top_idx == self.locked_idx

    def put_item(self, v):
        lock = 0
        if v == self.room_id and self.top_idx == self.locked_idx:
            lock = 1

        self.top_idx -= 1
        self.space[self.top_idx] = v
        self.locked_idx -= lock

    def remove_item(self):
        v = self.space[self.top_idx]
        self.space[self.top_idx] = None
        if v == self.room_id and self.top_idx == self.locked_idx:
            self.locked_idx += 1
        self.top_idx += 1

        return v

    def top_letter(self):
        return self.space[self.top_idx]


class Board:
    def __init__(self, tpos):
        self.hallway = [None for _ in range(11)]
        self.rooms = [Room(i, tpos[i]) for i in range(4)]
        self.cost = 0
        self.banned_positions = set(2 + 2 * x for x in range(4)) # room entries

    def is_finished(self):
        return all(room.is_finished for room in self.rooms)

    def moves_list(self):
        # hallway -> target column
        for i in range(len(self.hallway)):
            column = self.hallway[i]
            if column is None: continue

            # check if hallway to the target column is free
            column_entry_idx = 2 + 2 * column

            start, stop = i, column_entry_idx
            if start != stop:
                if start > stop:
                    start, stop = stop, start
                else:
                    start += 1
                    stop += 1

                if any(self.hallway[i] is not None for i in range(start, stop)): continue

            if self.rooms[column].can_be_entered:
                move_to_column_cost = self.rooms[column].top_idx
                cost = abs(column_entry_idx - i) + move_to_column_cost
                return {('h', i): [(column, cost)]}

        # column -> (hallway|column)
        moves = {}
        for column, room in enumerate(self.rooms):
            if room.is_finished or room.can_be_entered: continue

            move_from_column_cost = room.top_idx + 1
            target_column = room.top_letter()
            column_entry_idx = 2 + 2 * column
            target_column_entry_idx = 2 + 2 * target_column

            # column -> hallway
            collected = []
            column_to_column = None
            idx = column_entry_idx - 1
            while self.hallway[idx] is None:
                if idx not in self.banned_positions:
                    cost = move_from_column_cost + abs(column_entry_idx - idx)
                    collected.append((idx, cost))
                    if idx == target_column_entry_idx:
                        column_to_column = cost
                idx -= 1
                if idx < 0: break

            idx = column_entry_idx + 1
            while self.hallway[idx] is None:
                if idx not in self.banned_positions:
                    cost = move_from_column_cost + abs(column_entry_idx - idx)
                    collected.append((idx, cost))
                    if idx == target_column_entry_idx:
                        column_to_column = cost
                idx += 1
                if idx >= 11: break

            # check if column -> column is possible using information gathered earlier
            if self.rooms[target_column].can_be_entered and column_to_column:
                move_to_column_cost = self.rooms[target_column].top_idx
                cost = column_to_column + move_to_column_cost
                return {('e', column): [(target_column, cost)]}
            elif collected:
                moves[('c', column)] = collected

        return moves

    def do_move(self, move_type, move_data):
        type, from_idx = move_type
        to_idx, cost = move_data

        if type == 'h':  # hallway -> column
            v = self.hallway[from_idx]
            self.rooms[to_idx].put_item(v)
            self.hallway[from_idx] = None
        elif type == 'c':  # column -> hallway
            v = self.rooms[from_idx].remove_item()
            self.hallway[to_idx] = v
        elif type == 'e':  # column -> column
            v = self.rooms[from_idx].remove_item()
            self.rooms[to_idx].put_item(v)

        self.cost += cost * get_energy(v)

    def undo_move(self, move_type, move_data):
        type, from_idx = move_type
        to_idx, cost = move_data

        if type == 'h':  # hallway -> column (back)
            v = self.rooms[to_idx].remove_item()
            self.hallway[from_idx] = v
        elif type == 'c':  # column -> hallway (back)
            v = self.hallway[to_idx]
            self.rooms[from_idx].put_item(v)
            self.hallway[to_idx] = None
        elif type == 'e':  # column -> column (back)
            v = self.rooms[to_idx].remove_item()
            self.rooms[from_idx].put_item(v)

        self.cost -= cost * get_energy(v)


energies = [1, 10, 100, 1000]
def get_energy(v):
    return energies[v]

def search(b: Board):
    global min_cost

    if b.cost >= min_cost: return

    if b.is_finished():
        min_cost = min(min_cost, b.cost)
        return

    move_candidates = b.moves_list()
    for move_type, moves_list in move_candidates.items():
        for move in moves_list:
            b.do_move(move_type, move)
            search(b)
            b.undo_move(move_type, move)

def solve(init_data):
    global min_cost

    min_cost = float('inf')
    b = Board(init_data)
    search(b)
    return min_cost


print(solve(tpos))
print(solve(tpos2))

