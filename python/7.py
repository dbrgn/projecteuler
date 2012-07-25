from primes import getprimes

i = 0
for prime in getprimes():
    i += 1
    if i == 10001:
        print 'Prime is %s' % prime
        break
