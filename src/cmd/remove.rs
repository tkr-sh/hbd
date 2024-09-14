use crate::{error::HbdResult, files::storage::Storage};

pub fn remove(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    let user_string = user.to_string();
    for (_, names) in storage_birthdays.birthdays.iter_mut() {
        if names.contains(&user_string) {
            names.retain(|n| n != user);
            storage_birthdays.write_to_storage()?;
            return Ok(());
        }
    }

    println!("No users with the name \x1b[1m`{user}`\x1B[0m found.");

    Ok(())
}
