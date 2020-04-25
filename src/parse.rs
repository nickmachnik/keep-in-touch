//! Functions for parsing inputs.

use chrono::{DateTime, TimeZone, Utc};
use std::convert::TryInto;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct ShortVec;

impl fmt::Display for ShortVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected three elements")
    }
}

impl error::Error for ShortVec {
    fn description(&self) -> &str {
        "Expected three elements"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub fn parse_date(arg: &str) -> Result<DateTime<Utc>, Box<dyn error::Error>> {
    match arg {
        "now" => Ok(Utc::now()),
        _ => {
            let mut split = Vec::new();
            for e in arg.split('-') {
                split.push(e.parse::<u32>()?);
            }
            if split.len() < 3 {
                Err(ShortVec.into())
            } else {
                Ok(Utc
                    .ymd(split[0].try_into().unwrap(), split[1], split[2])
                    .and_hms(12, 12, 12))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_now() {
        assert_eq!(Utc::now().date(), parse_date("now").unwrap().date());
    }

    #[test]
    fn test_parse_date_fail_short_date() {
        assert!(parse_date("2002-05").is_err());
    }

    #[test]
    fn test_parse_date_fail_wrong_format() {
        assert!(parse_date("baba-05-02").is_err());
    }

    #[test]
    fn test_parse_date_custom_date() {
        assert_eq!(
            Utc.ymd(2020, 5, 2).and_hms(12, 12, 12),
            parse_date("2020-05-02").unwrap()
        );
    }
}
