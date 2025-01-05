use chrono::{Datelike, Local, NaiveDate};
use rand::prelude;
use rusqlite::{params, Connection};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{:04}", self.day, self.month, self.year)
    }
}

impl Date {
    pub fn from_string(date: String) -> Result<Option<Self>, Box<dyn Error>> {
        if date.as_str() == "None" {
            return Ok(None);
        }
        let result = NaiveDate::parse_from_str(date.as_str(), "%d-%m-%Y");
        return match result {
            Ok(i) => Ok(Some(Date {
                day: i.day() as u8,
                month: i.month() as u8,
                year: i.year(),
            })),
            Err(_) => Err("invalid date".into()),
        };
    }
    pub fn get_local_date() -> Self {
        let actual_date = Local::now();
        return Date {
            day: actual_date.day() as u8,
            month: actual_date.month() as u8,
            year: actual_date.year() as i32,
        };
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        return self.day == other.day && self.month == other.month && self.year == other.year;
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            return Some(std::cmp::Ordering::Less);
        }
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        return Some(std::cmp::Ordering::Greater);
    }

    fn lt(&self, other: &Self) -> bool {
        if self.year < other.year {
            return true;
        }
        if self.year == other.year && self.month < other.month {
            return true;
        }
        if self.year == other.year && self.month == other.month && self.day < other.day {
            return true;
        }
        return false;
    }

    fn gt(&self, other: &Self) -> bool {
        if self.year > other.year {
            return true;
        }
        if self.year == other.year && self.month > other.month {
            return true;
        }
        if self.year == other.year && self.month == other.month && self.day > other.day {
            return true;
        }
        return false;
    }

    fn le(&self, other: &Self) -> bool {
        return self == other || self < other;
    }

    fn ge(&self, other: &Self) -> bool {
        return self == other || self > other;
    }
}

impl Clone for Date {
    fn clone(&self) -> Self {
        let value = Self {
            day: self.day,
            month: self.month,
            year: self.year,
        };
        return value;
    }
}

#[cfg(test)]
mod tests_date {
    use super::*;

    #[test]
    fn test_equality() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let c = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_neq_day() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 30,
            month: 12,
            year: 1999,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_neq_month() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 11,
            year: 1999,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_neq_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_neq_day_and_month() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 29,
            month: 1,
            year: 1999,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn assert_neq_month_and_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 11,
            year: 2013,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn assert_neq_day_and_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 18,
            month: 11,
            year: 2014,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn assert_neq_day_month_and_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 18,
            month: 11,
            year: 2014,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        assert!(!(a > b));
        assert!(!(b < a));
    }

    #[test]
    fn assert_partial_cmp_greater_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b > a);
        assert!(a < b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_greater_month() {
        let a = Date {
            day: 31,
            month: 11,
            year: 2000,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b > a);
        assert!(a < b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_greater_day() {
        let a = Date {
            day: 30,
            month: 12,
            year: 2000,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b > a);
        assert!(a < b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_greater_eq_year() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b >= a);
        assert!(a <= b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_greater_eq_month() {
        let a = Date {
            day: 31,
            month: 11,
            year: 2000,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b >= a);
        assert!(a <= b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_greater_eq_day() {
        let a = Date {
            day: 30,
            month: 12,
            year: 2000,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 2000,
        };
        assert!(b > a);
        assert!(a < b);
        assert_ne!(a, b);
    }

    #[test]
    fn assert_partial_cmp_eq() {
        let a = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        let b = Date {
            day: 31,
            month: 12,
            year: 1999,
        };
        assert!(a >= b);
        assert!(b <= a);
        assert!(b >= a);
        assert!(a <= b);

        assert!(!(a > b));
        assert!(!(b > a));
        assert!(!(a < b));
        assert!(!(b < a));

        assert_eq!(a, b);
        assert!(!(a != b));
    }

    #[test]
    fn create_valid_date() {
        let a = Date::from_string(String::from("12-1-2002"));
        let b = Date {
            day: 12,
            month: 1,
            year: 2002,
        };

        assert!(a.is_ok(), "Result should be a valid date");
        assert_eq!(a.unwrap().unwrap(), b);
    }

    #[test]
    fn create_not_valid_date() {
        let a = Date::from_string(String::from("12-13-2002"));

        assert!(a.is_err(), "Result should be a error");
        if let Err(i) = a {
            assert_eq!(i.to_string(), String::from("invalid date"));
        }
    }

    #[test]
    fn create_none_date() {
        let a = Date::from_string(String::from("None"));

        assert!(a.is_ok(), "Result should be not a error");
        assert_eq!(a.unwrap(), None);
    }
}
