//! Problem 56
//! 
//! A googol (10^100) is a massive number: one followed by
//! one-hundred zeros; 100^100 is almost unimaginably large:
//! one followed by two-hundred zeros. Despite their size,
//! the sum of the digits in each number is only 1.
//!
//! Considering natural numbers of the form, a^b,
//! where a, b < 100, what is the maximum digital sum?

extern crate num;

use std::num::FromPrimitive;
use num::bigint::BigUint;


fn main() {
    let limits = (99, 99);
    let mut maxlen = 0;

    println!("Hello Euler!");

    for base in (0..limits.0) {
        for exponent in (0..limits.1) {
            let sum = digital_sum(base + 1, exponent + 1);
            if sum > maxlen {
                println!("Digital sum {} is larger than previous {}", sum, maxlen);
                maxlen = sum;
            }
        }
    }

    println!("Highest possible digital sum: {}", maxlen);
}


#[allow(unstable)]
fn do_pow(a : u8, b : u8) -> BigUint {
    let start : BigUint = FromPrimitive::from_u8(a).unwrap();
    let mut result = start.clone();

    for _ in (1..b) {
        result = result * &start;
    }
    result
}

#[allow(unstable)]
fn digital_sum(a : u8, b : u8) -> u16 {
    let result_string = do_pow(a, b).to_string();
    result_string.chars().fold(0, |a, b| a + b.to_digit(10).unwrap() as u16)
}


#[test]
#[allow(unstable)]
fn test_do_pow() {
    assert_eq!(do_pow(1, 1), 1.to_biguint().unwrap());
    assert_eq!(do_pow(5, 3), 125.to_biguint().unwrap());
    assert_eq!(do_pow(2, 65).to_string(), "36893488147419103232");
}

#[test]
fn test_digital_sum() {
    assert_eq!(digital_sum(2, 8), 13);
    assert_eq!(digital_sum(5, 3), 8);
}
