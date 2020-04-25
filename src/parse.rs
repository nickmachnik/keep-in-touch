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

fn parse_date(arg: &str) -> Result<DateTime<Utc>, Box<dyn error::Error>> {
    match arg {
        "now" => Ok(Utc::now()),
        _ => {
            let split = arg.split('-').map(|e| e.parse::<u32>()).collect::<Vec<_>>();
            if arg.len() < 3 {
                Err(ShortVec.into())
            } else {
                Ok(Utc
                    .ymd(split[0]?.try_into().unwrap(), split[1]?, split[2]?)
                    .and_hms(12, 12, 12))
            }
        }
    }
}
