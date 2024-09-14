use {
    crate::{
        error::HbdResult,
        files::{config::ToolConfig, storage::Storage},
        utils::{
            date::DateAndYear,
            fmt::{fmt_string, FormatWith},
        },
    },
    chrono::{Datelike, NaiveDate, Utc},
};

pub fn list(limit_day: Option<usize>, limit_names: Option<usize>) -> HbdResult<()> {
    let storage_birthdays = Storage::read_from_json()?;
    let mut config = ToolConfig::read_from_config()?;

    // Get the current date and time
    let now_naive = Utc::now().date_naive();
    let now = DateAndYear::from_naivedate(&now_naive);

    let mut birthdays_sorted = storage_birthdays
        .birthdays
        .into_iter()
        .collect::<Vec<(u16, _)>>();

    birthdays_sorted.sort_by(|a, b| {
        // convert in number of days
        let a_duration = if a.0 > *now.date_u16() {
            a.0
        } else {
            a.0 + 1024
        };

        // convert in number of days
        let b_duration = if b.0 > *now.date_u16() {
            b.0
        } else {
            b.0 + 1024
        };

        a_duration.cmp(&b_duration)
    });

    let mut it_names = 0usize;

    for (birthday, names) in birthdays_sorted.iter_mut() {
        // If there are nobody
        if names.is_empty() {
            continue;
        }

        let birthday_date = DateAndYear::new(*birthday, Some(now_naive.year() as u16));
        let birthday_naive = NaiveDate::from(birthday_date);

        let date = if *birthday < *now.date_u16() {
            birthday_naive.with_year(now_naive.year() + 1).unwrap()
        } else {
            birthday_naive
        };

        // Print
        let in_num_days = (date - now_naive).num_days();

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
                fmt_string(
                    config.will_be(),
                    FormatWith::d(
                        (now_naive.year() - *year as i32 +
                            if date.year() != now_naive.year() {
                                1
                            } else {
                                0
                            })
                        .to_string()
                        .as_str(),
                    ),
                )
                .to_string()
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
