use {
    super::{add, remove},
    crate::error::HbdResult,
};

pub fn set(name: &str, date: &str) -> HbdResult<()> {
    remove(name)?;
    add(name, date)?;
    Ok(())
}
