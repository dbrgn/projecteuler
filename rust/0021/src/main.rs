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

use std::collections::{HashMap, HashSet};


fn main() {
    let max = 10000;

    let mut solutions: HashMap<u32, u32> = HashMap::new();
    let mut amicable_numbers: HashSet<u32> = HashSet::new();

    // Calculate prober divisor sum for all numbers
    for n in (2..max*2) {  // I feel bad for that * 2...
        let dn = (1..n/2+1).filter(|&x| n % x == 0).fold(0, |a, b| a + b);
        solutions.insert(n, dn);
        println!("Solution pair: d({}) = {}", n, dn);
    }

    // Find amicable pairs
    for (n, dn) in solutions.iter() {
        if n != dn {  // Ignore equal pairs
            match solutions.get(dn) {
                Some(reverse_n) => {
                    if n == reverse_n && n <= &max {
                        amicable_numbers.insert(*n);
                        amicable_numbers.insert(*dn);
                    }
                },
                None => (),
            }
        }
    }

    // Print amicable numbers
    for num in &amicable_numbers {
        println!("Amicable number: {}", num);
    }

    // Print sum
    let amicable_sum = amicable_numbers.iter().fold(0, |a, b| a + *b);
    println!("Sum of amicable numbers <= {}: {}", max, amicable_sum);

}
