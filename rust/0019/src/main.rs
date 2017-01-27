//! Problem 19
//! INCOMPLETE!
//! 
//! You are given the following information, but you may prefer to do some
//! research for yourself.
//!
//! * 1 Jan 1900 was a Monday.
//! * Thirty days has September,
//!   April, June and November.
//!   All the rest have thirty-one,
//!   Saving February alone,
//!   Which has twenty-eight, rain or shine.
//!   And on leap years, twenty-nine.
//! * A leap year occurs on any year evenly divisible by 4, but not on a
//! century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month during the twentieth
//! century (1 Jan 1901 to 31 Dec 2000)?


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
fn days_per_month(month: u8, year: u16) -> u8 {
    match month {
        2 => days_in_feb(year),
        4 | 6 | 9 | 11 => 30,
        _ => 31
    }
}


fn main() {
    // January 1, 1900 was a Monday.
    // That means that the first Sunday ways on day 6.
    let offset = 6;

    // Start counting the days, starting at January 1, 1900.
    let mut total_day: u32 = 0;
    let mut month: u8 = 1;
    let mut year: u16 = 1900;

    // Main loop
    let mut count = 0;
    loop {
        // Is the current day a Sunday?
        if total_day > offset
            && year >= 1901
            && (total_day - offset) % 7 == 0 {
            println!("Day {} ({}/{}) is a Sunday", total_day, month, year);
            count += 1;
        }

        // Get days in current month
        total_day += days_per_month(month, year) as u32;

        // Increment month
        if month == 12 {
            month = 1;
            year += 1;
        } else {
            month += 1
        }

        // Stop condition
        if year > 2000 {
            break;
        }
    }

    println!("Done! Result is {}", count);
}
