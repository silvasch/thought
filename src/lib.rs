mod error;
pub(crate) use error::Error;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    Ok(())
}
