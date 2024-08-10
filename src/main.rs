use std::{fmt, ops::Add};

use time::{OffsetDateTime, PrimitiveDateTime};
fn main() {
    println!("{} {}", Clock::new(0, 3).add_minutes(-4), "23:59")
}

pub fn time_after_gigaseconds(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let required_time =
        OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + (10_i64.pow(9)))
            .unwrap();
    PrimitiveDateTime::new(required_time.date(), required_time.time())
}

pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut calculated_hours: i32 = hours;
        let mut calculated_minutes: i32 = minutes;
        if minutes < 0 {
            if minutes % 60 == 0 {
                calculated_hours += minutes / 60;
                calculated_minutes = 0;
            } else {
                calculated_hours += (minutes / 60) - 1;
                calculated_minutes = 60 + (minutes % 60);
            };
            println!("{} {}", calculated_hours, calculated_minutes)
        }
        if calculated_hours < 0 {
            calculated_hours = 24 + (calculated_hours % 24);
        }

        Self {
            hours: ((calculated_hours % 24) + calculated_minutes / 60) % 24,
            minutes: calculated_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut calculated_hours = 0;
        let mut calculated_minutes = 0;
        if minutes < 0 {
            let temp = if minutes % 60 == 0 || self.minutes >= -minutes {
                0
            } else {
                1
            };
            calculated_hours = self.hours + (minutes / 60) - temp;
            println!("{} {}", calculated_hours, calculated_minutes);
            while calculated_hours < 0 {
                calculated_hours += 24;
            }
            if self.minutes >= (-minutes % 60) {
                calculated_minutes = self.minutes + (minutes % 60);
            } else {
                calculated_minutes = self.minutes + (minutes % 60) + 60
            }

            return Self {
                hours: calculated_hours,
                minutes: calculated_minutes,
            };
        }

        Self {
            hours: ((self.hours.add(minutes / 60) + (self.minutes.add(minutes % 60)) / 60) % 24),
            minutes: (self.minutes.add(minutes % 60)) % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours <= 9 && self.minutes > 9 {
            write!(f, "0{}:{}", self.hours, self.minutes)
        } else if self.hours > 9 && self.minutes <= 9 {
            write!(f, "{}:0{}", self.hours, self.minutes)
        } else if self.hours <= 9 && self.minutes <= 9 {
            write!(f, "0{}:0{}", self.hours, self.minutes)
        } else {
            write!(f, "{}:{}", self.hours, self.minutes)
        }
    }
}
