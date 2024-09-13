use {
    super::utils::{data_config, handling_file_creation},
    crate::error::HbdResult,
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct ToolConfig {
    pub format: ConfigFormat,
    default_config: Option<Box<ToolConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFormat {
    separator_happy_birthday: Option<String>,
    separator_happy_birthday_age: Option<String>,
    happy_birthday: Option<String>,
    happy_birthday_age: Option<String>,
    birthday_of: Option<String>,
    in_x_days: Option<String>,
    will_be: Option<String>,
}

/// This macro implements getter for fieds
macro_rules! fn_format {
    ($name:ident) => {
        pub fn $name(&mut self) -> String {
            self.format.$name.clone().unwrap_or_else(|| {
                if self.default_config.is_none() {
                    self.default_config = Some(Box::new(
                        ron::from_str(include_str!(r"../../config.ron"))
                            .expect("The default config file for hbd should always be present."),
                    ));
                }

                if let Some(t) = &self.default_config {
                    t.format
                        .$name
                        .clone()
                        .expect("The default config should have all the fields defined.")
                } else {
                    String::new()
                }
            })
        }
    };
}


impl ToolConfig {
    fn_format!(separator_happy_birthday);

    fn_format!(separator_happy_birthday_age);

    fn_format!(happy_birthday);

    fn_format!(happy_birthday_age);

    fn_format!(birthday_of);

    fn_format!(in_x_days);

    fn_format!(will_be);

    pub fn read_from_config() -> HbdResult<Self> {
        let directory = format!("{}/hbd", data_config()?);
        let file_to_read_from = format!("{directory}/config.ron");

        let ron_content =
            handling_file_creation(&file_to_read_from, &directory, default_stringified_struct)?;


        Ok(ron::from_str(&ron_content)?)
    }
}



fn default_stringified_struct() -> String {
    String::from(include_str!(r"../../config.ron"))
}
