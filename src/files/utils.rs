use crate::error::HbdResult;

pub fn data_home() -> HbdResult<String> {
    Ok(
        std::env::var("XDG_DATA_HOME")
            .unwrap_or(format!("{}/.local/share", std::env::var("HOME")?)),
    )
}

pub fn data_config() -> HbdResult<String> {
    Ok(std::env::var("XDG_DATA_CONFIG").unwrap_or(format!("{}/.config", std::env::var("HOME")?)))
}

pub fn handling_file_creation(
    path: &str,
    dir: &str,
    default_value: fn() -> String,
) -> HbdResult<String> {
    Ok(match std::fs::read_to_string(path) {
        Ok(s) => s,
        // Case the file doesn't exists
        Err(why) => {
            match why.kind() {
                // If the file doesn't exists, create it
                std::io::ErrorKind::NotFound => {
                    match create_file(path, default_value) {
                        Ok(_) => default_value(),
                        // If we can't  create file, we're going to try creating the directory, and
                        // if doesn't work, we forfeit
                        Err(_why) => {
                            match std::fs::create_dir_all(dir) {
                                Ok(_) => {
                                    create_file(path, default_value)?;
                                    default_value()
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
    })
}

fn create_file(using_path: &str, with_default_value: fn() -> String) -> HbdResult<()> {
    Ok(std::fs::write(using_path, with_default_value())?)
}
