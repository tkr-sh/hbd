use crate::{
    error::HbdResult,
    files::storage::{self, Storage},
};

pub fn remove(user: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    let user_string = user.to_string();
    for (_, names) in storage_birthdays.birthdays.iter_mut() {
        if names.contains(&user_string) {
            names.retain(|n| n != user);
            storage_birthdays.write_to_storage()?;
            break;
        }
    }

    Ok(())
}
