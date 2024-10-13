use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::{EditorWrapper, Error, ThoughtId, Truncate};

#[derive(Clone, Debug)]
pub struct Thought {
    thought_id: ThoughtId,
    date_time: DateTime<Utc>,
    file_path: PathBuf,
}

impl Thought {
    pub fn new<P: AsRef<Path>>(thoughts_dir: P) -> Result<Self, Error> {
        let thoughts_dir = thoughts_dir.as_ref().to_path_buf();
        if thoughts_dir.is_file() {
            return Err(Error::ThoughtsPathIsFile {
                invalid_thoughts_dir: thoughts_dir.to_string_lossy().to_string(),
            });
        }

        let thought_id = ThoughtId::new();
        let date_time = Utc::now();
        let file_path = thoughts_dir.join(format!("{}-{}.md", date_time.timestamp(), thought_id));

        Ok(Self {
            thought_id,
            date_time,
            file_path,
        })
    }

    pub fn from_file_path<P: AsRef<Path>>(file_path: P) -> Result<Self, Error> {
        let file_path = file_path.as_ref().to_path_buf();
        if file_path.is_dir() {
            return Err(Error::ThoughtPathIsDir {
                invalid_thought_path: file_path.to_string_lossy().to_string(),
            });
        }

        let file_stem = file_path
            .file_stem()
            .expect("file should always have a name")
            .to_string_lossy()
            .to_string();

        let mut split = file_stem.splitn(2, '-');
        let raw_timestamp = split.next().ok_or(Error::ParseThoughtFromPath {
            thought_path: file_path.to_string_lossy().to_string(),
        })?;
        let date_time = DateTime::from_timestamp(
            raw_timestamp.parse().map_err(|_| Error::ParseTimestamp {
                invalid_timestamp: raw_timestamp.to_string(),
            })?,
            0,
        )
        .ok_or(Error::ParseTimestamp {
            invalid_timestamp: raw_timestamp.to_string(),
        })?;
        let thought_id = split
            .next()
            .ok_or(Error::ParseThoughtFromPath {
                thought_path: file_path.to_string_lossy().to_string(),
            })?
            .parse()?;

        Ok(Self {
            thought_id,
            date_time,
            file_path,
        })
    }

    pub fn get_content(&self) -> Result<String, Error> {
        Ok(std::fs::read_to_string(&self.file_path).unwrap())
    }

    pub fn edit(&self, editor_wrapper: EditorWrapper) -> Result<(), Error> {
        editor_wrapper.edit(&self.file_path)
    }

    pub fn delete(&self) -> Result<(), Error> {
        std::fs::remove_file(&self.file_path).map_err(|_| Error::RemoveThought)?;
        Ok(())
    }

    pub fn id(&self) -> &ThoughtId {
        &self.thought_id
    }
}

impl std::fmt::Display for Thought {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let thought_content = self
            .get_content()
            .unwrap_or("".to_string())
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string()
            .truncate_with_ellipsis(40);

        write!(
            f,
            "{} ({}) {}",
            self.thought_id,
            self.date_time.naive_local().format("%Y-%m-%d"),
            thought_content,
        )
    }
}
