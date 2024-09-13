use {
    crate::{
        error::HbdResult,
        files::{config::ToolConfig, storage::Storage},
        utils::{
            date::convert_date_formatted_to_naive_date,
            fmt::{fmt_string, FormatWith},
        },
    },
    chrono::{Datelike, NaiveDate, Utc},
};

pub fn list(limit_day: Option<u16>, limit_names: Option<u16>) -> HbdResult<()> {
    let storage_birthdays = Storage::read_from_json()?;
    let mut config = ToolConfig::read_from_config()?;

    // Get the current date and time
    let now: NaiveDate = Utc::now().date_naive();

    let mut birthdays_sorted = storage_birthdays
        .birthdays
        .into_iter()
        .collect::<Vec<(String, _)>>();

    birthdays_sorted.sort_by(|a, b| {
        let a_date = convert_date_formatted_to_naive_date(&a.0, now.year());
        let b_date = convert_date_formatted_to_naive_date(&b.0, now.year());
        // convert in number of days
        let a_duration = (a_date - now).num_days();
        let a_duration = if a_duration > 0 {
            a_duration
        } else {
            a_duration + 1024
        };
        // convert in number of days
        let b_duration = (b_date - now).num_days();
        let b_duration = if b_duration > 0 {
            b_duration
        } else {
            b_duration + 1024
        };

        a_duration.cmp(&b_duration)
    });

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
            "{}",
            fmt_string(
                config.in_x_days(),
                FormatWith::new(
                    if in_num_days != 1 { "s" } else { "" },
                    in_num_days.to_string().as_str(),
                )
            ),
        );

        for name in names {
            let stringified_age = if let Some(year) = storage_birthdays.ages.get(name) {
                format!(
                    "{}",
                    fmt_string(
                        config.will_be(),
                        FormatWith::d(
                            (now.year() - *year as i32 +
                                if date.year() != now.year() { 1 } else { 0 })
                            .to_string()
                            .as_str()
                        )
                    )
                )
            } else {
                String::new()
            };

            println!(
                "{}",
                fmt_string(
                    config.birthday_of(),
                    FormatWith::new(name, &stringified_age)
                )
            );

            it_names += 1;

            if limit_names.is_some() && Some(it_names) >= limit_names {
                return Ok(());
            }
        }
        println!();
    }

    Ok(())
}
