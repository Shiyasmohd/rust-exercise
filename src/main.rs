use std::{
    collections::HashSet,
    fmt,
    ops::{Add, Div, Mul},
};

use time::{OffsetDateTime, PrimitiveDateTime};
fn main() {
    println!("{}", square_of_sum(10));
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

// Exercise 5
pub fn is_armstrong_number(num: i64) -> bool {
    let total_digits = number_of_digits(num);
    if total_digits == 1 {
        return true;
    }
    println!("{}", total_digits);
    let mut n = num.clone();
    let mut sum = 0;
    for i in 1..=total_digits {
        sum += (n % 10).pow(total_digits);
        n = n / 10;
        println!("{}", sum);
    }
    if sum == num {
        return true;
    }
    false
}

fn number_of_digits(num: i64) -> u32 {
    num.to_string().len().try_into().unwrap()
}

// Excercise 6
pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1)).pow(2).div_euclid(4)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n).abs_diff(sum_of_squares(n))
}

// Exercise 7
#[derive(Debug)]
pub struct Duration {
    age_in_earth: f64,
}

impl From<i64> for Duration {
    fn from(s: i64) -> Self {
        Self {
            age_in_earth: (s as f64 / 31_557_600.0).mul(100.0).round().div(100.0),
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.age_in_earth / Self::orbital_period()
    }
    fn orbital_period() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn orbital_period() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn orbital_period() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn orbital_period() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn orbital_period() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn orbital_period() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn orbital_period() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn orbital_period() -> f64 {
        164.79132
    }
}

// Exercise 8
#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }
        let total_scores = self.scores.len();
        Some(self.scores[total_scores - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 0 {
            return None;
        }
        let mut max = self.scores[0];
        for score in &self.scores {
            if score > &max {
                max = score.clone()
            }
        }
        Some(max)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.len() == 0 {
            return [].to_vec();
        }
        let mut sorted = self.scores.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.iter().take(3).cloned().collect()
    }
}
