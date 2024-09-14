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
    date_u16: u16,
    year: Option<u16>,
}

impl From<DateAndYear> for NaiveDate {
    fn from(val: DateAndYear) -> Self {
        NaiveDate::from_ymd_opt(
            val.year.unwrap() as i32,
            val.date_u16 as u32 / 31 + 1,
            val.date_u16 as u32 % 31 + 1,
        )
        .unwrap()
    }
}

impl DateAndYear {
    pub fn new(date_u16: u16, year: Option<u16>) -> DateAndYear {
        DateAndYear { date_u16, year }
    }

    pub fn from_naivedate(naive_date: &NaiveDate) -> DateAndYear {
        DateAndYear {
            date_u16: (naive_date.month() as u16 - 1) * 31 + (naive_date.day() as u16 - 1),
            year: Some(naive_date.year() as u16),
        }
    }

    pub fn from_date_str(birth_date: &str) -> HbdResult<DateAndYear> {
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
                        date_u16: (capture[1].parse::<u16>()? - 1) * 31 +
                            (capture[2].parse::<u16>()? - 1),
                        year: None,
                    });
                } else {
                    return Err(w.into());
                }
            },
        };

        // Format the date as "MM-DD"
        Ok(DateAndYear {
            date_u16: (naive_date.month() as u16 - 1) * 31 + (naive_date.day() as u16 - 1),
            year: Some(naive_date.year() as u16),
        })
    }

    pub fn date_u16(&self) -> &u16 {
        &self.date_u16
    }

    pub fn date_formatted(&self) -> String {
        format!(
            "{:0>2}-{:0>2}",
            self.date_u16 / 31 + 1,
            self.date_u16 % 31 + 1
        )
    }

    pub fn year(&self) -> &Option<u16> {
        &self.year
    }

    pub fn set_year(&mut self, y: u16) {
        self.year = Some(y)
    }
}
