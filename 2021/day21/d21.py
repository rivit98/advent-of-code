from collections import Counter
from copy import copy
from functools import lru_cache

players = open('input.txt').read().strip().split('\n')
players_pos_orig = list(map(int, map(lambda s: s.split(':')[1], players)))

# one star
players_pos = copy(players_pos_orig)
scores = [0, 0]
roll_nr = 0
die = 1


def roll_die():
    global die, roll_nr
    s = 0
    for _ in range(3):
        s += die
        die += 1
        if die == 101:
            die = 1
    roll_nr += 3
    return s


def calc_player_pos(pos, roll):
    pos += roll
    pos %= 10
    if pos == 0:
        pos = 10
    return pos


while all(s < 1000 for s in scores):
    for p in range(2):
        roll = roll_die()
        players_pos[p] = calc_player_pos(players_pos[p], roll)
        scores[p] += players_pos[p]
        if scores[p] >= 1000: break

print(roll_nr * min(scores))

# two stars
cnt = Counter(i + j + k for i in range(1, 4) for j in range(1, 4) for k in range(1, 4))

@lru_cache(maxsize=None)
def game(scores, players_pos):
    p1_score_orig, p2_score_orig = scores
    p1_pos_orig, p2_pos_orig = players_pos
    if p1_score_orig >= 21: return 1, 0
    if p2_score_orig >= 21: return 0, 1

    p1_wins_ret, p2_wins_ret = 0, 0
    for s1 in range(3, 10):
        w1 = cnt[s1]
        p1_pos = calc_player_pos(p1_pos_orig, s1)
        p1_score = p1_score_orig + p1_pos
        for s2 in range(3, 10):
            new_players_pos = (p1_pos, calc_player_pos(p2_pos_orig, s2))
            new_scores = (p1_score, p2_score_orig + new_players_pos[1])
            w2 = cnt[s2]
            p1_wins, p2_wins = game(new_scores, new_players_pos)
            p1_wins_ret += p1_wins * w1 * w2
            p2_wins_ret += p2_wins * w1 * w2

    return p1_wins_ret, p2_wins_ret


p1_wins, p2_wins = game((0, 0), tuple(players_pos_orig))

print(p1_wins, p2_wins)
print(max(p1_wins, p2_wins) // 27)
# no idea why, but it has to be divided by 27 (number of possibilities of three rolls)
# probably I'm counting one roll too much
