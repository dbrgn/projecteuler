package main

import (
    "fmt"
    "github.com/dbrgn/projecteuler"
)

func main() {
    ch := make(chan int) // Create a new channel
    go sieve.Getprime(ch)
    for i := 0; i < 1000; i++ {
        prime := <-ch
        fmt.Printf("Prime %d: %d\n", i + 1, prime)
    }
}
