-- PROBLEM 25:
--
-- What is the first term in the Fibonacci sequence to contain 1000 digits?

fibs = 0 : 1 : zipWith (+) fibs (tail fibs)

main = do
    print $ length $ takeWhile (< 10^999) fibs
