use {
    crate::{error::HbdResult, files::storage::Storage},
    chrono::{Datelike, Utc},
};

pub fn read(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

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


    storage_birthdays.write_to_storage()?;

    Ok(())
}
