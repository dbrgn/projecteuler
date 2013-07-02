"""
Problem 17

If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in
words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20
letters. The use of "and" when writing out numbers is in compliance with
British usage.
"""

words_ones = ('', 'one', 'two', 'three', 'four', 'five', 'six', 'seven',
'eight', 'nine', 'ten', 'eleven', 'twelve', 'thirteen', 'fourteen', 'fifteen',
'sixteen', 'seventeen', 'eighteen', 'nineteen')
words_tens = ('', '', 'twenty', 'thirty', 'forty', 'fifty', 'sixty',
'seventy', 'eighty', 'ninety')


def get_word(i):
    ones = i % 10 if i % 100 >= 20 else i % 20
    tens = i % 100 // 10
    hundreds = i % 1000 // 100
    if i == 1000:
        return 'one thousand'
    if i < 100:
        return words_tens[tens] + words_ones[ones]
    if ones == tens == 0:
        return words_ones[hundreds] + 'hundred'
    if i < 1000:
        return words_ones[hundreds] + 'hundred and ' + get_word(i % 100)
    if i > 1000:
        raise NotImplementedError('Only numbers <= 1000 are supported.')
    assert False, 'This should never be reached'


LOWER, UPPER = 1, 1000
words = (get_word(i).replace(' ', '') for i in xrange(LOWER, UPPER + 1))
print sum(map(len, words))
