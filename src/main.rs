use std::{collections::HashSet, fmt, ops::Add};

use time::{OffsetDateTime, PrimitiveDateTime};
fn main() {
    let word = "BANANA";
    let inputs = &["banana"];
    println!("{:?}", find_anagrams(word, inputs));
}

// Exercise 1
pub fn time_after_gigaseconds(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let required_time =
        OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + (10_i64.pow(9)))
            .unwrap();
    PrimitiveDateTime::new(required_time.date(), required_time.time())
}

// Exercise 2
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

// Exercise 3
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
        let mut calculated_hours;
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

// Exercise 4

fn remove_char(arr: &mut Vec<char>, target: char) {
    if let Some(pos) = arr.iter().position(|&c| c == target) {
        arr.remove(pos);
    }
}

fn find_anagrams<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let result_arr: HashSet<&str> = possible_anagrams
        .iter()
        .filter(|&&anagram| is_anagram(word, anagram))
        .copied()
        .collect();
    println!("{:?}", result_arr);
    result_arr
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    if word.len() != possible_anagram.len() || word == possible_anagram {
        return false;
    }
    let mut chars: Vec<_> = possible_anagram.to_lowercase().chars().collect();
    let mut temp_word: Vec<_> = word.to_lowercase().chars().collect();
    for char in &chars.clone() {
        if temp_word.contains(&char) {
            remove_char(&mut temp_word, *char);
            remove_char(&mut chars, *char);
            println!("After removing {} : {:?} {:?}", char, temp_word, chars);
        }
    }
    if chars.len() == 0 && temp_word.len() == 0 {
        true
    } else {
        false
    }
}
