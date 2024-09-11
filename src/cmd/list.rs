use {
    crate::{
        error::HbdResult,
        file::read_birthdays_from_json,
        utils::date::convert_date_formatted_to_naive_date,
    },
    chrono::{Datelike, NaiveDate, Utc},
};

pub fn list(limit_day: Option<u16>, limit_names: Option<u16>) -> HbdResult<()> {
    let storage_birthdays = read_birthdays_from_json()?;

    // Get the current date and time
    let now: NaiveDate = Utc::now().date_naive();

    let mut birthdays_sorted = storage_birthdays
        .birthdays
        .into_iter()
        .collect::<Vec<(String, _)>>();

    birthdays_sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut it_names = 0;

    for (birthday, names) in birthdays_sorted.iter_mut() {
        // If there are nobody
        if names.is_empty() {
            continue;
        }

        let date = convert_date_formatted_to_naive_date(birthday, now.year());

        // Calculate the difference in days
        let duration = (date - now).num_days();

        let date = if duration < 0 {
            date.with_year(now.year() + 1).unwrap()
        } else {
            date
        };

        // Print
        let in_num_days = (date - now).num_days();

        // If there is a limit in number of days, and that we're over it.
        if limit_day.is_some() && (limit_day.unwrap() as i64) < in_num_days {
            break;
        }


        println!(
            "\x1B[1mIn {} day{}:\x1B[0m",
            in_num_days,
            if in_num_days != 1 { "s" } else { "" }
        );

        for name in names {
            let stringified_age = if let Some(year) = storage_birthdays.ages.get(name) {
                format!("(Will be {} years old)", now.year() - *year as i32)
            } else {
                String::new()
            };

            println!("Birthday of {name}!! {stringified_age}");

            it_names += 1;

            if limit_names.is_some() && Some(it_names) >= limit_names {
                return Ok(());
            }
        }
        println!();
    }

    Ok(())
}
