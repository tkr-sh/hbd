use crate::{
    error::HbdResult,
    files::storage::Storage,
    utils::{check_exists::check_user_exists, date::DateAndYear},
};


pub fn add(user: &str, birth_date: &str) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    let formatted_date = DateAndYear::from_date_str(birth_date)?;

    // If the user exists, we can't proceed!
    if check_user_exists(&storage_birthdays, user) {
        println!("A person with the name `{user}` already exists.\nUse `rename` to change that person's name, or, add this person with another name.");
        std::process::exit(1)
    }

    if let Some(birthdays) = storage_birthdays
        .birthdays
        .get_mut(formatted_date.date_u16())
    {
        birthdays.push(user.to_owned());
    } else {
        storage_birthdays
            .birthdays
            .insert(*formatted_date.date_u16(), vec![user.to_owned()]);
    }

    if let Some(year) = formatted_date.year() {
        storage_birthdays.ages.insert(user.to_owned(), *year);
    }


    storage_birthdays.write_to_storage()?;

    Ok(())
}
