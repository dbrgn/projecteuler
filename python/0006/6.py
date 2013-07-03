def sum_of_squares(n):
    s = sum(map(lambda x: x ** 2, xrange(1, n + 1)))
    print 'Sum of squares: %s' % s
    return s


def square_of_sum(n):
    s = sum(xrange(1, n + 1)) ** 2
    print 'Square of sum: %s' % s
    return s


def square_sum_difference(n):
    print 'Max number: %s' % n
    return square_of_sum(n) - sum_of_squares(n)


print 'Difference: %s' % square_sum_difference(10)
print '-' * 10
print 'Difference: %s' % square_sum_difference(100)
