package main

import "fmt"

/* Recursively calculate the n-th fibonacci number */
func fib(n int) int {
    if n < 2 {
        return 1
    }
    return fib(n - 1) + fib(n - 2)
}

func main() {
    sum := 0
    for i := 1 ;; i++ {
        f := fib(i)
        if f > 4e6 {
            break
        }
        if f % 2 == 0 {
            sum += f
        }
    }
    fmt.Println(sum)
}
