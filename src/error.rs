#[derive(Debug)]
pub enum Error {
    ThoughtsPathIsFile { invalid_thoughts_dir: String },
    ThoughtPathIsDir { invalid_thought_path: String },
    ThoughtNotFound { thought_id: String },
    ExecuteEditor,
    ListThoughts,
    BaseDirectories(xdg::BaseDirectoriesError),
    ParseId { invalid_thought_id: String },
    ParseTimestamp { invalid_timestamp: String },
    RemoveThought,
    ParseThoughtFromPath { thought_path: String },
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
            Error::ThoughtNotFound { thought_id } => {
                write!(f, "no thought with id '{}' found.", thought_id)
            }
            Error::ExecuteEditor => write!(f, "failed to start the editor."),
            Error::ListThoughts => write!(f, "failed to list your thoughts."),
            Error::BaseDirectories(e) => {
                write!(f, "failed to get the base directories: '{}'.", e)
            }
            Error::ParseId { invalid_thought_id } => {
                write!(f, "failed to parse the id '{}'.", invalid_thought_id)
            }
            Error::ParseTimestamp { invalid_timestamp } => {
                write!(f, "failed to parse the timestamp '{}'.", invalid_timestamp)
            }
            Error::RemoveThought => write!(f, "failed to remove the thought."),
            Error::ParseThoughtFromPath { thought_path } => write!(
                f,
                "failed to parse a thought from the path '{}'.",
                thought_path
            ),
        }
    }
}

impl From<xdg::BaseDirectoriesError> for Error {
    fn from(value: xdg::BaseDirectoriesError) -> Self {
        Self::BaseDirectories(value)
    }
}

impl std::error::Error for Error {}
