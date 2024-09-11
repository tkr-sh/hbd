use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Storage {
    pub reads: Vec<String>,
    pub birthdays: Birthdays,
    pub ages: HashMap<String, u16>,
}

impl Storage {
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

pub type Birthdays = HashMap<String, Vec<String>>;
