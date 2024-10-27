//! You are given the following information, but you may prefer
//! to do some research for yourself.
//! * 1 Jan 1900 was a Monday.
//! * Thirty days has September,
//!
//!   April, June and November.
//!
//!   All the rest have thirty-one,
//!
//!   Saving February alone,
//!
//!   Which has twenty-eight, rain or shine.
//!
//!   And on leap years, twenty-nine.
//! * A leap year occurs on any year evenly divisible by 4, but not
//!   on a century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month during
//! the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn is_leap_year(year: u32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn count_days_of_month(year: u32, month: u32) -> u32 {
    match month {
        3 | 5 | 8 | 10 => 30,
        1 => match is_leap_year(year) {
            true => 29,
            false => 28,
        },
        _ => 31,
    }
}

pub fn main() {
    let mut count = 0;

    // 1 Jan 1900 was a Monday
    let mut first_of_month_weekday = 0;

    for year in 0..=100 {
        for month in 0..12 {
            if first_of_month_weekday == 6
                && year >= 1
                && (year < 100 || month < 11)
            {
                count += 1;
            }

            first_of_month_weekday += count_days_of_month(1900 + year, month);
            first_of_month_weekday %= 7;
        }
    }

    println!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(2200));
        assert!(!is_leap_year(2300));
    }
}
