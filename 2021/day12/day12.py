from collections import defaultdict
from copy import copy

data = open("input.txt").read().strip().splitlines()
graph = defaultdict(list)

for line in data:
    a, b = map(str.strip, line.split('-'))
    graph[a].append(b)
    graph[b].append(a)

# one star
def dfs(node, visited):
    if node == 'end': return 1

    if node.islower(): visited.add(node)

    c = 0
    for n in graph[node]:
        if n in visited: continue
        c += dfs(n, copy(visited))

    return c


print(dfs('start', {'start'}))

# two stars
def dfs2(node, visited):
    if node == 'end': return 1

    if node.islower():
        visited[node] += 1

    c = 0
    small_visited_twice = any([x == 2 for x in visited.values()])
    for n in graph[node]:
        if visited[n] == 2: continue
        if visited[n] == 1 and small_visited_twice: continue
        c += dfs2(n, copy(visited))

    return c

for k, v in graph.items():
    if k != 'start' and 'start' in v:
        v.remove('start')

graph['end'] = []

print(dfs2('start', defaultdict(int, {'start': 0})))

