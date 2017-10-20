(*
 * PROBLEM 1
 *
 * If we list all the natural numbers below 10 that are
 * multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of
 * these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *)
 
open Printf

let rec main max =
    if max = 0 then
        0
    else if (max mod 3) = 0 || (max mod 5) = 0 then
        max + (main (max - 1))
    else
        main (max - 1)
    ;;

printf "Sum is %d\n" (main 999)
