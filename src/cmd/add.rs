use {
    crate::{
        error::{HbdError, HbdResult},
        files::storage::Storage,
        utils::check_exists::check_user_exists,
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
    let mut storage_birthdays = Storage::read_from_json()?;

    let formatted_date = parse_date(birth_date)?;

    // If the user exists, we can't proceed!
    if check_user_exists(&storage_birthdays, user) {
        println!("A person with the name `{user}` already exists.\nUse `rename` to change that person's name, or, add this person with another name.");
        std::process::exit(1)
    }

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


    storage_birthdays.write_to_storage()?;

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
