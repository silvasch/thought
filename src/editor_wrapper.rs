use std::path::Path;

use crate::Error;

pub struct EditorWrapper {
    editor_override: Option<String>,
}

impl EditorWrapper {
    pub fn new() -> Self {
        Self {
            editor_override: None,
        }
    }

    pub fn new_with_override(editor_override: impl Into<String>) -> Self {
        Self {
            editor_override: Some(editor_override.into()),
        }
    }

    pub fn edit<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Error> {
        let file_path = file_path.as_ref();

        let editor_command = self
            .editor_override
            .clone()
            .unwrap_or(std::env::var("EDITOR").unwrap_or("xdg-open".to_string()));

        let _ = std::process::Command::new(editor_command)
            .arg(file_path)
            .status()
            .map_err(|_| todo!())?;

        Ok(())
    }
}
