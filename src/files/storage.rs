use {
    super::utils::{data_home, handling_file_creation},
    crate::error::HbdResult,
    std::collections::HashMap,
};

/// date_u16 => Vec<name>
pub type Birthdays = HashMap<u16, Vec<String>>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Storage {
    /// name =>  Vec<year>
    pub reads: HashMap<String, Vec<u16>>,
    pub birthdays: Birthdays,
    /// name =>  year
    pub ages: HashMap<String, u16>,
}

impl Storage {
    pub fn write_to_storage(&self) -> HbdResult<()> {
        let directory = format!("{}/hbd", data_home()?);
        let file_to_write_to = format!("{directory}/birthdays.json");

        std::fs::write(file_to_write_to, serde_json::to_string(self)?)?;

        Ok(())
    }

    pub fn read_from_json() -> HbdResult<Self> {
        let directory = format!("{}/hbd", data_home()?);
        let file_to_read_from = format!("{directory}/birthdays.json");

        let json_content =
            handling_file_creation(&file_to_read_from, &directory, default_stringified_struct)?;

        Ok(serde_json::from_str(&json_content)?)
    }

    pub fn birthdays(&self) -> &Birthdays {
        &self.birthdays
    }

    pub fn ages(&self) -> &HashMap<String, u16> {
        &self.ages
    }

    pub fn reads(&self) -> &HashMap<String, Vec<u16>> {
        &self.reads
    }
}



fn default_stringified_struct() -> String {
    String::from("{\"reads\":{},\"birthdays\":{},\"ages\":{}}")
}
