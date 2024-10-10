#[derive(Debug)]
pub enum Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")?;

        Ok(())
    }
}

impl std::error::Error for Error {}
