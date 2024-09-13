use {
    crate::{
        error::HbdResult,
        files::{config::ToolConfig, storage::Storage},
        utils::fmt::{fmt_string, FormatWith},
    },
    chrono::{Datelike, Utc},
};


pub fn get() -> HbdResult<()> {
    let storage_birthdays = Storage::read_from_json()?;
    let mut config_birthdays = ToolConfig::read_from_config()?;
    println!("{config_birthdays:#?}");

    // Get the current date and time
    let now = Utc::now();

    // Format the date as "MM-DD"
    let formatted_date = now.format("%m-%d").to_string();

    // Get all the user from which it's the birthday today
    if let Some(birthdays) = storage_birthdays.birthdays().get(&formatted_date) {
        for birthday_of in birthdays {
            if !storage_birthdays
                .reads()
                .contains(&format!("{birthday_of}:{}", now.year()))
            {
                if let Some(year) = storage_birthdays.ages().get(birthday_of) {
                    let age = now.year() - *year as i32;
                    println!(
                        "{}",
                        fmt_string(
                            config_birthdays.happy_birthday_age(),
                            FormatWith::new(birthday_of, age.to_string().as_str()),
                        )
                    );
                } else {
                    println!(
                        "{}",
                        fmt_string(
                            config_birthdays.happy_birthday(),
                            FormatWith::s(birthday_of),
                        )
                    );
                }
            }
        }
    }


    Ok(())
}
