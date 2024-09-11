use {
    crate::{
        error::{HbdError, HbdResult},
        file::{read_birthdays_from_json, write_birthday_storage},
        shared::Storage,
    },
    chrono::{Datelike, NaiveDate, Utc},
    regex::Regex,
    std::{fmt::write, str::FromStr},
};

pub fn read(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = read_birthdays_from_json()?;

    // Get the current date and time
    let now = Utc::now();

    // Format the date as "MM-DD"
    let formatted_date = now.format("%m-%d").to_string();

    // Get all the user from which it's the birthday today
    if let Some(birthdays) = storage_birthdays.birthdays().get(&formatted_date) {
        if birthdays.contains(&user.to_string()) {
            storage_birthdays
                .reads
                .push(format!("{user}:{}", now.year()));
        } else {
            println!("Error: Today is not the birthday of {user}.");
        }
    }


    write_birthday_storage(&storage_birthdays)?;

    Ok(())
}
