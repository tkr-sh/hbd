use {
    super::utils::{data_home, handling_file_creation},
    crate::error::HbdResult,
    std::collections::HashMap,
};

pub type Birthdays = HashMap<String, Vec<String>>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Storage {
    pub reads: Vec<String>,
    pub birthdays: Birthdays,
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

    pub fn reads(&self) -> &Vec<String> {
        &self.reads
    }
}



fn default_stringified_struct() -> String {
    String::from("{\"reads\":[],\"birthdays\":{},\"ages\":{}}")
}
