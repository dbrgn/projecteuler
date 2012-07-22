-- PROBLEM 6:
--
-- The sum of the squares of the first ten natural numbers is,
--
-- 1^2 + 2^2 + ... + 10^2 = 385
--
-- The square of the sum of the first ten natural numbers is,
--
-- (1 + 2 + ... + 10)^2 = 55^2 = 3025
--
-- Hence the difference between the sum of the squares of the first ten natural
-- numbers and the square of the sum is 3025 - 385 = 2640.
--
-- Find the difference between the sum of the squares of the first one hundred
-- natural numbers and the square of the sum.

sum_of_squares :: Int -> Int
sum_of_squares n = sum $ map (\x -> x^2) [1..n]

square_of_sum :: Int -> Int
square_of_sum n = sum [1..n] ^ 2

main = do
    print $ (square_of_sum 100) - (sum_of_squares 100)
