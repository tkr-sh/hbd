use {
    chrono::ParseError,
    ron::de::SpannedError,
    std::{env::VarError, num::ParseIntError},
};

pub type HbdResult<OK> = Result<OK, HbdError>;

#[derive(Debug)]
pub enum HbdError {
    CustomError(String),
    VarError(VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
    ParseDate(ParseError),
    ParseInt(ParseIntError),
    Ron(SpannedError),
}

// Macro to implement erros into
macro_rules! impl_from_error {
    ($from:ty, $to:path) => {
        impl From<$from> for HbdError {
            fn from(rejection: $from) -> Self {
                $to(rejection)
            }
        }
    };
}

impl_from_error!(VarError, Self::VarError);
impl_from_error!(std::io::Error, Self::Io);
impl_from_error!(serde_json::Error, Self::Json);
impl_from_error!(ParseError, Self::ParseDate);
impl_from_error!(SpannedError, Self::Ron);
impl_from_error!(ParseIntError, Self::ParseInt);
