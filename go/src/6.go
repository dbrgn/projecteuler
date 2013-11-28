/*
   PROBLEM 6:

   The sum of the squares of the first ten natural numbers is,

   1^2 + 2^2 + ... + 10^2 = 385

   The square of the sum of the first ten natural numbers is,

   (1 + 2 + ... + 10)^2 = 55^2 = 3025

   Hence the difference between the sum of the squares of the first ten natural
   numbers and the square of the sum is 3025 - 385 = 2640.

   Find the difference between the sum of the squares of the first one hundred
   natural numbers and the square of the sum.
*/

package main

import (
    "fmt"
)

func numbersGenerator(ch chan int) {
    i := 1
    for {
        ch <- i
        i++
    }
}

func squaresGenerator(ch chan int) {
    i := 1
    for {
        ch <- i * i
        i++
    }
}

func main() {
    numbers := make(chan int)
    squares := make(chan int)

    go numbersGenerator(numbers)
    go squaresGenerator(squares)

    var sumOfSquares, squareOfSums int
    for i := 0; i < 100; i++ {
        sumOfSquares += <-squares
        squareOfSums += <-numbers
    }
    squareOfSums *= squareOfSums
    fmt.Printf("%d - %d = %d\n", squareOfSums, sumOfSquares, squareOfSums-sumOfSquares)
}
