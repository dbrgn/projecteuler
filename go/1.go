package main

import "fmt"

const maxint = 1000
var sum int

func main() {
    for i := 1 ; i < maxint ; i++ {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    fmt.Println(sum)
}
