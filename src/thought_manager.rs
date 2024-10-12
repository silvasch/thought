use std::path::PathBuf;

use crate::{Error, ThoughtId};

pub struct ThoughtManager {
    base_dirs: xdg::BaseDirectories,
}

impl ThoughtManager {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            base_dirs: xdg::BaseDirectories::with_prefix("thought").map_err(|_| todo!())?,
        })
    }

    pub fn create_thought(&self) -> Result<PathBuf, Error> {
        let thought_id = ThoughtId::new();

        let thought_path = self
            .base_dirs
            .place_data_file(format!("{}.md", thought_id))
            .map_err(|_| todo!())?;

        Ok(thought_path)
    }

    pub fn get_thought_path(&self, thought_id: &ThoughtId) -> PathBuf {
        self.base_dirs.get_data_file(format!("{}.md", thought_id))
    }

    pub fn find_thought(&self, user_thought_id: &str) -> Result<Option<ThoughtId>, Error> {
        for thought_id in self.get_thought_ids()? {
            if thought_id.get_user_thought_id() == user_thought_id {
                return Ok(Some(thought_id));
            }
        }

        Ok(None)
    }

    pub fn get_thought_ids(&self) -> Result<Vec<ThoughtId>, Error> {
        let mut thought_ids = vec![];

        for file_path in std::fs::read_dir(self.base_dirs.get_data_home()).map_err(|_| {
            Error::ListThoughtsDir {
                path: self.base_dirs.get_data_home().to_string_lossy().to_string(),
            }
        })? {
            let file_path: PathBuf = file_path
                .map_err(|_| Error::ListThoughtsDir {
                    path: self.base_dirs.get_data_home().to_string_lossy().to_string(),
                })?
                .path();

            let raw_thought_id = file_path
                .file_stem()
                .unwrap_or(file_path.as_os_str())
                .to_string_lossy()
                .to_string();

            let thought_id = raw_thought_id.parse()?;

            thought_ids.push(thought_id);
        }

        Ok(thought_ids)
    }
}
