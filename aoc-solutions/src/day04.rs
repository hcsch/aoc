use std::lazy::SyncLazy;

use envy;
use itertools::Itertools;
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

static HAIR_COLOR_RE: SyncLazy<Regex> = SyncLazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap());
static EYE_COLOR_RE: SyncLazy<Regex> =
    SyncLazy::new(|| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
static PASSPORT_ID_RE: SyncLazy<Regex> = SyncLazy::new(|| Regex::new(r"^[0-9]{9}$").unwrap());

#[derive(Debug, Deserialize, Validate)]
struct Passport {
    #[serde(rename = "byr")]
    #[validate(range(min = 1920, max = 2002))]
    birth_year: u32,
    #[serde(rename = "iyr")]
    #[validate(range(min = 2010, max = 2020))]
    issue_year: u32,
    #[serde(rename = "eyr")]
    #[validate(range(min = 2020, max = 2030))]
    expiration_year: u32,
    #[serde(rename = "hgt")]
    #[validate(custom = "validate_height")]
    height: String,
    #[serde(rename = "hcl")]
    #[validate(regex = "HAIR_COLOR_RE")]
    hair_color: String,
    #[serde(rename = "ecl")]
    #[validate(regex = "EYE_COLOR_RE")]
    eye_color: String,
    #[serde(rename = "pid")]
    #[validate(regex = "PASSPORT_ID_RE")]
    passport_id: String,
    #[serde(rename = "cid", default)]
    country_id: Option<String>,
}

fn validate_height(height_str: &str) -> Result<(), ValidationError> {
    let height: u32 = height_str[..height_str.len() - 2]
        .parse()
        .map_err(|_| ValidationError::new("invalid_height"))?;

    if height_str.ends_with("cm") && height >= 150 && height <= 193 {
        Ok(())
    } else if height_str.ends_with("in") && height >= 59 && height <= 76 {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_height"))
    }
}

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> impl Iterator<Item = Result<Passport, String>> {
    input_lines
        .coalesce(|current, next| {
            let c_blank = current.trim().is_empty();
            let n_blank = next.trim().is_empty();
            match (c_blank, n_blank) {
                (false, false) => Ok(format!("{} {}", current.trim(), next.trim())),
                (false, true) => Err((current, next)),
                (true, false) => Ok(next.trim().to_owned()),
                (true, true) => Ok("".to_string()),
            }
        })
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            envy::from_iter(s.split_whitespace().map(|field_str| {
                let mut field_parts = field_str.split(':');
                let field_name = field_parts.next().unwrap();
                let field_value = field_parts.next().unwrap();

                (field_name.to_owned(), field_value.to_owned())
            }))
            .map_err(|_| s)
        })
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let passports = parse_input(input_lines);
    passports.filter(Result::is_ok).count().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let passports = parse_input(input_lines);
    passports
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(|pp| pp.validate().is_ok())
        .count()
        .to_string()
}
