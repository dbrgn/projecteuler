"""
Problem 9

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""

import sys

def triplet(m, n):
    """Euclid's formula. Generates a pythagoraen triple given an arbitrary pair
    of positive integers m and n with m > n."""
    a = m**2 - n**2
    b = 2 * m * n
    c = m**2 + n**2
    return a, b, c

for m in xrange(2, 100):
    for n in xrange(1, m - 1):
        t = triplet(m, n)
        result = sum(t)
        if result == 1000:
            print 'FOUND TRIPLET!', t, result
            print 'Product:', t[0] * t[1] * t[2]
            sys.exit(0)
        print t, result
