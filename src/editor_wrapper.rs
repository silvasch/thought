use std::path::Path;

use crate::Error;

pub struct EditorWrapper;

impl EditorWrapper {
    pub fn edit<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Error> {
        let file_path = file_path.as_ref();

        let editor_command = std::env::var("EDITOR").unwrap_or("xdg-open".to_string());

        let _ = std::process::Command::new(editor_command)
            .arg(file_path)
            .status()
            .map_err(|_| Error::ExecuteEditor)?;

        Ok(())
    }
}
