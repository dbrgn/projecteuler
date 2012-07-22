-- PROBLEM 1:
--
-- If we list all the natural numbers below 10 that are multiples of 3 or 5, we
-- get 3, 5, 6 and 9. The sum of these multiples is 23.
--
-- Find the sum of all the multiples of 3 or 5 below 1000.

isvalid :: Int -> Bool
isvalid x | mod x 3 == 0  = True
          | mod x 5 == 0  = True
          | otherwise     = False

main = do
    print $ sum ( filter isvalid [1..999] )
