"""
Problem 67.

This is basically the same code as project 18.

It uses the Dijkstra shortest path algorithm to find the best suited path
through the triangle.
"""
import networkx as nx

# Data

with open('triangle.txt', 'r') as f:
    lines = f.readlines()
    triangle = [map(int, line.split()) for line in lines]

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
