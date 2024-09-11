use crate::{
        error::HbdResult,
        file::{read_birthdays_from_json, write_birthday_storage},
    };

pub fn remove(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = read_birthdays_from_json()?;

    let user_string = user.to_string();
    for (_, names) in storage_birthdays.birthdays.iter_mut() {
        if names.contains(&user_string) {
            names.retain(|n| n != user);
            write_birthday_storage(&storage_birthdays)?;
            break;
        }
    }

    Ok(())
}
