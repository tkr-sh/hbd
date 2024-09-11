use chrono::NaiveDate;

/// This function uses `.unwrap()` because the arguments passed to the function are considered to
/// be always in the valid format MM-DD.
/// FIXME: Can error with 02-29.
pub fn convert_date_formatted_to_naive_date(s: &str, with_year: i32) -> NaiveDate {
    let month = &s[0..=1].parse().unwrap();
    let day = &s[3..=4].parse().unwrap();
    NaiveDate::from_ymd_opt(with_year, *month, *day).unwrap()
}
