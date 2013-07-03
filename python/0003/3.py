# BROKEN
import sys
from algorithms import getprimes

factors = []
n = int(sys.argv[1])
for prime in getprimes(int(sys.argv[1])):
    if not n % prime:
        factors.append(prime)
        n = n / prime
print factors
