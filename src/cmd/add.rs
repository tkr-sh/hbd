use {
    crate::{
        error::{HbdError, HbdResult},
        file::{read_birthdays_from_json, write_birthday_storage},
    },
    chrono::{Datelike, NaiveDate},
    regex::Regex,
    std::str::FromStr,
};

struct DateAndYear {
    date_formatted: String,
    year: Option<u16>,
}

pub fn add(user: &str, birth_date: &str) -> HbdResult<()> {
    let mut storage_birthdays = read_birthdays_from_json()?;

    let formatted_date = parse_date(birth_date)?;


    if let Some(birthdays) = storage_birthdays
        .birthdays
        .get_mut(&formatted_date.date_formatted)
    {
        birthdays.push(user.to_owned());
    } else {
        storage_birthdays
            .birthdays
            .insert(formatted_date.date_formatted, vec![user.to_owned()]);
    }

    if let Some(year) = formatted_date.year {
        storage_birthdays.ages.insert(user.to_owned(), year);
    }


    write_birthday_storage(&storage_birthdays)?;

    Ok(())
}


fn parse_date(birth_date: &str) -> HbdResult<DateAndYear> {
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
