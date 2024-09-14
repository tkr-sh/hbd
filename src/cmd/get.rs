use {
    crate::{
        error::HbdResult,
        files::{config::ToolConfig, storage::Storage},
        utils::{
            date::DateAndYear,
            fmt::{fmt_string, FormatWith},
        },
    },
    chrono::{Datelike, Utc},
};


pub fn get(separator: Option<String>) -> HbdResult<()> {
    // If there is a separator, we will just print it.
    let storage_birthdays = Storage::read_from_json()?;
    let mut config_birthdays = ToolConfig::read_from_config()?;

    // Get the current date and time
    let now_utc = Utc::now();
    let now = DateAndYear::from_naivedate(&now_utc.date_naive());

    let ref_separator: &Option<String> = &separator;

    // Get all the user from which it's the birthday today
    if let Some(birthdays) = storage_birthdays.birthdays().get(&now.date_u16()) {
        let iter = birthdays
            .iter()
            .filter(|b| {
                !storage_birthdays
                    .reads()
                    .get(*b)
                    .map(|v| v.contains(&(now_utc.year() as u16)))
                    .unwrap_or(false)
            })
            .collect::<Vec<&String>>();

        let num_of_birthdays = iter.len();

        for (idx, birthday_of) in iter.into_iter().enumerate() {
            // in case we have the year, we'll try getting the age too
            if let Some(year) = storage_birthdays.ages().get(birthday_of) {
                let age = now_utc.year() - *year as i32;

                print!(
                    "{}",
                    fmt_string(
                        if ref_separator.is_some() {
                            config_birthdays.separator_happy_birthday_age()
                        } else {
                            config_birthdays.happy_birthday_age()
                        },
                        FormatWith::new(birthday_of, age.to_string().as_str()),
                    )
                );
            }
            // Else we won't print the age
            else {
                print!(
                    "{}",
                    fmt_string(
                        if ref_separator.is_some() {
                            config_birthdays.separator_happy_birthday()
                        } else {
                            config_birthdays.happy_birthday()
                        },
                        FormatWith::s(birthday_of),
                    )
                );
            }

            if ref_separator.is_some() {
                if num_of_birthdays - 1 != idx {
                    print!("{}", ref_separator.as_ref().unwrap());
                }
            } else {
                println!();
            }
        }
    }


    Ok(())
}
