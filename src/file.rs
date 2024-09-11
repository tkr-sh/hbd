use crate::{error::HbdResult, shared::Storage};


pub fn data_home() -> HbdResult<String> {
    Ok(
        std::env::var("XDG_DATA_HOME")
            .unwrap_or(format!("{}/.local/share", std::env::var("HOME")?)),
    )
}

pub fn read_birthdays_from_json() -> HbdResult<Storage> {
    let directory = format!("{}/hbd", data_home()?);
    let file_to_read_from = format!("{directory}/birthdays.json");

    let json_content = match std::fs::read_to_string(&file_to_read_from) {
        Ok(s) => s,
        // Case the file doesn't exists
        Err(why) => {
            match why.kind() {
                // If the file doesn't exists, create it
                std::io::ErrorKind::NotFound => {
                    match create_file(&file_to_read_from) {
                        Ok(_) => default_stringified_struct(),
                        // If we can't  create file, we're going to try creating the directory, and
                        // if doesn't work, we forfeit
                        Err(_why) => {
                            match std::fs::create_dir_all(&directory) {
                                Ok(_) => {
                                    create_file(&file_to_read_from)?;
                                    default_stringified_struct()
                                },
                                Err(why) => {
                                    eprintln!("An unexpected error occured: {why:#?}");
                                    std::process::exit(1);
                                },
                            }
                        },
                    }
                },
                err @ std::io::ErrorKind::OutOfMemory => {
                    eprintln!("Couldn't complete the operation due to missing memory.\n{err:#?}");
                    std::process::exit(1);
                },
                unhandled_error => {
                    eprintln!("An unexpected error occured: {unhandled_error:#?}");
                    std::process::exit(1);
                },
            }
        },
    };


    Ok(serde_json::from_str(&json_content)?)
}


pub fn write_birthday_storage(birthdays_storage: &Storage) -> HbdResult<()> {
    let directory = format!("{}/hbd", data_home()?);
    let file_to_write_to = format!("{directory}/birthdays.json");

    std::fs::write(file_to_write_to, serde_json::to_string(birthdays_storage)?)?;

    Ok(())
}



fn default_stringified_struct() -> String {
    String::from("{\"reads\":[],\"birthdays\":{},\"ages\":{}}")
}

fn create_file(using_path: &str) -> HbdResult<()> {
    Ok(std::fs::write(using_path, default_stringified_struct())?)
}
