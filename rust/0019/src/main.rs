//! Problem 19
//! INCOMPLETE!
//! 
//! You are given the following information, but you may prefer to do some research for yourself.
//!
//! * 1 Jan 1900 was a Monday.
//! * Thirty days has September,
//!   April, June and November.
//!   All the rest have thirty-one,
//!   Saving February alone,
//!   Which has twenty-eight, rain or shine.
//!   And on leap years, twenty-nine.
//! * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?


/// Return whether the specified year is a leap year.
fn is_leap_year(year: u16) -> bool {
    let divisible = (year % 4 == 0, year % 100 == 0, year % 400 == 0);
    match divisible {
        (true, false, _) => true,
        (true, true, true) => true,
        _ => false
    }
}


/// Return the number of days in February for the specified year
fn days_in_feb(year: u16) -> u8 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}


/// Return the number of days per month (0 index based) in the specified year
fn days_per_month(id: u8, year: u16) -> u8 {
    match id {
        1 => days_in_feb(year),
        3 | 5 | 8 | 10 => 30,
        _ => 31
    }
}


fn main() {
    println!("Hello, world!\n");
    let year = 2014;
    for month in 0..12 {
        println!("Month {} in {} has {} days.", month, year, days_per_month(month, year));
    }
    println!("");
    for year in 1899..2001 {
        if is_leap_year(year) {
            println!("Year {} is leap.", year);
        }
    }
}
