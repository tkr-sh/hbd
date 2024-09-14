use {
    crate::{
        error::{HbdError, HbdResult},
        files::storage::Storage,
        utils::{check_exists::check_user_exists, date::parse_date},
    },
    chrono::{Datelike, NaiveDate},
    regex::Regex,
    std::str::FromStr,
};


pub fn import(path: &str, exit_on_duplicate: Option<bool>) -> HbdResult<()> {
    let mut storage_birthdays = Storage::read_from_json()?;

    // get the content of the file
    let file_content = std::fs::read_to_string(path)?;


    // Get all the existings names in a vector, to easily check if they exists after that.
    let all_names = storage_birthdays
        .birthdays()
        .clone()
        .into_iter()
        .flat_map(|e| e.1)
        .collect::<Vec<String>>();

    for (i, line) in file_content.lines().enumerate() {
        let mut line_iter = line.split(' ');

        // Name
        let name = if let Some(name) = line_iter.next() {
            if all_names.iter().any(|n| n == name) {
                eprintln!("Person with name `{name}` already exists");

                if exit_on_duplicate.unwrap_or(false) {
                    std::process::exit(1)
                }
            }
            name
        } else {
            eprintln!("Item at line {i} is in a bad format: `{line}`");
            std::process::exit(1);
        };

        // Date
        let formatted_date = if let Some(birth_date) = line_iter.next() {
            match parse_date(birth_date) {
                Ok(d) => d,
                Err(why) => {
                    eprintln!("Item at line {i} is in a bad format: `{line}`");
                    return Err(why);
                },
            }
        } else {
            eprintln!("Item at line {i} is in a bad format: `{line}`");
            std::process::exit(1);
        };


        // Verify that there aren't extra arguments
        if line_iter.next().is_some() {
            eprintln!("Item at line {i} is in a bad format: `{line}`");
            std::process::exit(1);
        }

        if let Some(birthdays) = storage_birthdays
            .birthdays
            .get_mut(formatted_date.date_formatted())
        {
            birthdays.push(name.to_owned());
        } else {
            storage_birthdays.birthdays.insert(
                formatted_date.date_formatted().to_owned(),
                vec![name.to_owned()],
            );
        }

        if let Some(year) = formatted_date.year() {
            storage_birthdays.ages.insert(name.to_owned(), *year);
        }
    }

    storage_birthdays.write_to_storage()?;

    Ok(())
}
