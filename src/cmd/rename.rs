use crate::{error::HbdResult, files::storage::Storage};

pub fn rename(from: &str, to: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    let user_string = from.to_string();

    // Search the user
    for (_, names) in storage_birthdays.birthdays.iter_mut() {
        // When found:
        if names.contains(&user_string) {
            // Remove it from the vec
            names.retain(|n| n != from);
            // And add him with his new name
            names.push(to.to_owned());
            // update the age too
            if let Some(year) = storage_birthdays.ages.remove(from) {
                storage_birthdays.ages.insert(to.to_owned(), year);
            }

            // TODO: Add reads when the update is done

            storage_birthdays.write_to_storage()?;
            return Ok(());
        }
    }

    println!("No users with the name \x1b[1m`{from}`\x1B[0m found.");

    Ok(())
}
