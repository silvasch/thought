#[derive(Debug)]
pub enum Error {
    ThoughtsPathIsFile { invalid_thoughts_dir: String },
    ThoughtPathIsDir { invalid_thought_path: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ThoughtsPathIsFile {
                invalid_thoughts_dir,
            } => write!(f, "'{}' is a file, not a directory.", invalid_thoughts_dir),
            Error::ThoughtPathIsDir {
                invalid_thought_path,
            } => write!(f, "'{}' is a directory, not a file.", invalid_thought_path),
        }
    }
}

impl std::error::Error for Error {}
