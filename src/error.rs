use {chrono::ParseError, std::env::VarError};

pub type HbdResult<OK> = Result<OK, HbdError>;

#[derive(Debug)]
pub enum HbdError {
    CustomError(String),
    VarError(VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
    ParseDate(ParseError),
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
