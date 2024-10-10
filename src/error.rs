#[derive(Debug)]
pub enum Error {
    ParseThoughtId(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseThoughtId(invalid_thought_id) => write!(
                f,
                "failed to parse '{}' as a thought id.",
                invalid_thought_id
            )?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}
