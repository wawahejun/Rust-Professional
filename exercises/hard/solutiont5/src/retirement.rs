use std::ops::{Div, Rem};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
}

impl Date {
    fn new(year: u32, month: u32) -> Self {
        Self { year, month }
    }
}

impl FromStr for Date {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        let year = parts[0].parse::<u32>()?;
        let month = parts[1].parse::<u32>()?;

        Ok(Date { year, month })
    }
}

const FEMALE50: &str = "原法定退休年龄50周岁女职工";
const FEMALE55: &str = "原法定退休年龄55周岁女职工";
const MALE: &str = "男职工";

enum Employee {
    Male,
    Female50,
    Female55,
}

impl Employee {
    fn infos(&self) -> (u32, u32, u32) {
        match self {
            Employee::Male => (60, 4, 36),
            Employee::Female55 => (55, 4, 36),
            Employee::Female50 => (50, 2, 60),
        }
    }
}

impl FromStr for Employee {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            FEMALE50 => Ok(Employee::Female50),
            FEMALE55 => Ok(Employee::Female55),
            MALE => Ok(Employee::Male),
            _ => Err("Error input".to_string()),
        }
    }
}

fn add_month(ym: (u32, u32), month: u32) -> (u32, u32) {
    let (y, m) = ym;
    let new_m = m - 1 + month;
    (y + new_m.div(12), new_m.rem(12) + 1)
}

fn format_num(num: f64) -> String {
    if num.fract() == 0.0 {
        format!("{:.0}", num)
    } else {
        format!("{:.2}", num)
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_date = Date::from_str(time).unwrap();
    let tp = Employee::from_str(tp).unwrap();
    let (original_retire_age, factor, bound) = tp.infos();

    let period_months = if birth_date.year + original_retire_age < 2025 {
        0
    } else {
        (birth_date.year + original_retire_age - 2025) * 12 + birth_date.month
    };

    let extend_month = match period_months {
        0 => 0,
        num if num < bound => (num + factor - 1) / factor,
        _ => bound,
    };

    let retire_age = original_retire_age as f64 + extend_month as f64 / 12.0;

    let (retire_year, retire_month) = add_month(
        (birth_date.year + original_retire_age, birth_date.month),
        extend_month,
    );

    format!(
        "{}-{:02},{},{}",
        retire_year,
        retire_month,
        format_num(retire_age),
        extend_month
    )
}