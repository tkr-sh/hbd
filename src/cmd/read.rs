use {
    crate::{error::HbdResult, files::storage::Storage, utils::date::DateAndYear},
    chrono::{Datelike, Utc},
};

pub fn read(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    // Get the current date and time
    let now = Utc::now().naive_utc().date();

    // Format the date as "MM-DD"
    let date = DateAndYear::from_naivedate(&now);

    // Get all the user from which it's the birthday today
    if let Some(birthdays) = storage_birthdays.birthdays().get(date.date_u16()) {
        if birthdays.contains(&user.to_string()) {
            storage_birthdays
                .reads
                .entry(user.to_owned())
                .and_modify(|v| v.push(now.year() as u16))
                .or_insert(vec![now.year() as u16]);
        } else {
            println!("Error: Today is not the birthday of {user}.");
        }
    }


    storage_birthdays.write_to_storage()?;

    Ok(())
}
