//! Problem 21
//!
//! Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
//! into n).
//!
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b
//! are called amicable numbers.
//!
//! For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
//! therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//!
//! Evaluate the sum of all the amicable numbers under 10000.

use std::collections::HashSet;


fn divisor_sum(number: u32) -> u32 {
    (1..number/2+1)
        .filter(|&x| number % x == 0)
        .fold(0, |a, b| a + b)
}

fn main() {
    let max = 10_000;
    let mut amicable_numbers: HashSet<u32> = HashSet::new();

    // Find and print amicable numbers
    for n in (1..max) {
        let dn = divisor_sum(n);
        if n != dn && divisor_sum(dn) == n {
            println!("Amicable pair: {} <=> {}", n, dn);
            amicable_numbers.insert(n);
            amicable_numbers.insert(dn);
        }
    }

    // Print sum
    let amicable_sum = amicable_numbers.iter().fold(0, |a, b| a + *b);
    println!("Sum of amicable numbers: {}", amicable_sum);
}
