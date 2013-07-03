"""
Problem 13

Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
"""

data = open('13.txt', 'r')
nrs = [int(nr) for nr in data.read().split('\n') if nr]
print str(sum(nrs))[:10]
