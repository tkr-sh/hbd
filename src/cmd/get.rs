use {
    crate::{error::HbdResult, file::read_birthdays_from_json, shared::Storage},
    chrono::{Datelike, Utc},
    std::fmt::write,
};


pub fn get() -> HbdResult<()> {
    let storage_birthdays = read_birthdays_from_json()?;

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
                        "\x1B[1;33mToday is the birthday of {birthday_of} ({age} years old) !!!\x1B[0m"
                    );
                } else {
                    println!("\x1B[1;33mToday is the birthday of {birthday_of}!!!\x1B[0m");
                }
            }
        }
    }


    Ok(())
}
