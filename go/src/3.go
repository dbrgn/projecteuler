/*
   PROBLEM 3

   The prime factors of 13195 are 5, 7, 13 and 29.

   What is the largest prime factor of the number 600851475143?
*/

package main

import (
    "fmt"
    "github.com/dbrgn/projecteuler"
    "log"
)

func main() {
    number := 600851475143
    ch := make(chan int)
    var divisors []int

    go sieve.Getprime(ch)

    var solution_found bool = false
    for !solution_found {
        prime := <-ch
        for number%prime == 0 {
            log.Printf("%d mod %d == 0\n", number, prime)
            number /= prime
            divisors = append(divisors, prime)
            if number == 1 {
                solution_found = true
                break
            }
        }
    }
    fmt.Print("Divisors: ")
    fmt.Println(divisors)
}
