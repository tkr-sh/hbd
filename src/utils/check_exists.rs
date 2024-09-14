use crate::files::storage::Storage;

pub fn check_user_exists(storage: &Storage, user: &str) -> bool {
    storage
        .birthdays()
        .iter()
        .any(|b| b.1.iter().any(|e| e == user))
}
