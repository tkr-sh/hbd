use {
    crate::error::{HbdError, HbdResult},
    chrono::{Datelike, NaiveDate},
    regex::Regex,
    std::str::FromStr,
};

/// This function uses `.unwrap()` because the arguments passed to the function are considered to
/// be always in the valid format MM-DD.
/// FIXME: Can error with 02-29.
pub fn convert_date_formatted_to_naive_date(s: &str, with_year: i32) -> NaiveDate {
    let month = &s[0..=1].parse().unwrap();
    let day = &s[3..=4].parse().unwrap();
    NaiveDate::from_ymd_opt(with_year, *month, *day).unwrap()
}



pub struct DateAndYear {
    date_formatted: String,
    year: Option<u16>,
}
impl DateAndYear {
    pub fn date_formatted(&self) -> &str {
        &self.date_formatted
    }

    pub fn year(&self) -> &Option<u16> {
        &self.year
    }
}


pub fn parse_date(birth_date: &str) -> HbdResult<DateAndYear> {
    let naive_date = match NaiveDate::from_str(birth_date) {
        Ok(date) => date,
        Err(w) => {
            let pattern = r"^(\d{2})-(\d{2})$"; // Matches "MM-DD" format
            let re = Regex::new(pattern).unwrap();

            // Check if the entire string matches the pattern
            if let Some(capture) = re.captures(birth_date) {
                if capture.len() < 3 {
                    return Err(HbdError::CustomError("Invalid date format.".to_owned()));
                }


                return Ok(DateAndYear {
                    date_formatted: format!("{}-{}", &capture[1], &capture[2]),
                    year: None,
                });
            } else {
                return Err(w.into());
            }
        },
    };

    // Format the date as "MM-DD"
    Ok(DateAndYear {
        date_formatted: naive_date.format("%m-%d").to_string(),
        year: Some(naive_date.year() as u16),
    })
}
