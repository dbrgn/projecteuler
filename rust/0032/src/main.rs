//! Pandigital products
//! 
//! We shall say that an n-digit number is pandigital if it makes use of all
//! the digits 1 to n exactly once; for example, the 5-digit number, 15234, is
//! 1 through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
//! multiplicand, multiplier, and product is 1 through 9 pandigital.
//!
//! Find the sum of all products whose multiplicand/multiplier/product identity
//! can be written as a 1 through 9 pandigital.
//!
//! HINT: Some products can be obtained in more than one way so be sure to only
//! include it once in your sum.

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    // The number of digits in the multiplicand and multiplier cannot be more
    // than 5 (pseudo-proof: 1 x n where n is a 5-digit number would result in
    // a 5 digit number, bringing the total digits up to 11).
    
    // First, get all potential combinations of numbers.
    let mut combinations: Vec<(u32, u32)> = Vec::new();
    for i in 1..9 {
        for j in 1234..9876 {
            combinations.push((i, j));
        }
    }
    for i in 12..98 {
        for j in 123..987 {
            combinations.push((i, j));
        }
    }
    println!("Step 1: Got {} possible combinations", combinations.len());

    // Next, kick out all combinations where digits on the left side repeat
    combinations.retain(|&pair| {
        let mut seen = HashSet::new();
        for c in pair.0.to_string().chars().chain(pair.1.to_string().chars()) {
            if c == '0' {
                return false;
            }
            if seen.contains(&c) {
                return false;
            }
            seen.insert(c);
        }
        true
    });
    println!("Step 2: Got {} possible combinations", combinations.len());

    // Next, take into account multiplication result.
    let mut valid_results = HashSet::new();
    combinations.retain(|&pair| {
        let mut seen = HashSet::new();
        let result = pair.0 * pair.1;
        for c in pair.0.to_string().chars()
            .chain(pair.1.to_string().chars())
            .chain(result.to_string().chars()) {
            if c == '0' {
                return false;
            }
            if seen.contains(&c) {
                return false;
            }
            seen.insert(c);
        }
        if valid_results.insert(result) {
            println!("{} x {} = {}", pair.0, pair.1, result);
        }
        true
    });
    println!("Step 3: Got {} possible combinations", combinations.len());
    println!("Step 4: Got {} valid results", valid_results.len());

    let sum: u32 = valid_results.into_iter().sum();
    println!("Total sum is: {}", sum);
}
