"""
Problem 18
"""
import networkx as nx

# Data

triangle = [
    (75,),
    (95, 64),
    (17, 47, 82),
    (18, 35, 87, 10),
    (20, 04, 82, 47, 65),
    (19, 01, 23, 75, 03, 34),
    (88, 02, 77, 73, 07, 63, 67),
    (99, 65, 04, 28, 06, 16, 70, 92),
    (41, 41, 26, 56, 83, 40, 80, 70, 33),
    (41, 48, 72, 33, 47, 32, 37, 16, 94, 29),
    (53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14),
    (70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57),
    (91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48),
    (63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31),
    (04, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 04, 23),
]

# Build a directed weighted graph

graph = nx.DiGraph()
root = 'start'
for i, row in enumerate(triangle):
    for j, item in enumerate(row):
        sources = []
        if i == 0:
            sources.append(root)
        else:
            if j < i:
                sources.append((i - 1, j, triangle[i - 1][j]))
            if j > 0:
                sources.append((i - 1, j - 1, triangle[i - 1][j - 1]))
        target = (i, j, triangle[i][j])
        weight = triangle[i][j]
        for source in sources:
            graph.add_edge(source, target, weight=100 - weight)

# Connect all bottom nodes to a common end node

last_row_index = len(triangle) - 1
for j, item in enumerate(triangle[last_row_index]):
    start = (last_row_index, j, triangle[last_row_index][j])
    graph.add_edge(start, 'end', weight=0)

# Get shortest path

path = nx.dijkstra_path(graph, 'start', 'end')
cost = sum(node[2] for node in path[1:-1])
print cost
