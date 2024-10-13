use std::path::Path;

use crate::{Error, Thought, ThoughtId};

pub struct ThoughtCollection {
    thoughts: Vec<Thought>,
}

impl ThoughtCollection {
    pub fn new<P: AsRef<Path>>(thoughts_dir: P) -> Result<Self, Error> {
        Ok(Self {
            thoughts: Self::get_thoughts(thoughts_dir)?,
        })
    }

    pub fn find(&self, thought_id: &ThoughtId) -> Result<&Thought, Error> {
        for thought in &self.thoughts {
            if thought.id() == thought_id {
                return Ok(thought);
            }
        }

        Err(Error::ThoughtNotFound {
            thought_id: thought_id.to_string(),
        })
    }

    pub fn thoughts(&self) -> &Vec<Thought> {
        &self.thoughts
    }

    fn get_thoughts<P: AsRef<Path>>(thoughts_dir: P) -> Result<Vec<Thought>, Error> {
        let mut thoughts = vec![];

        for file_path in std::fs::read_dir(thoughts_dir).map_err(|_| Error::ListThoughts)? {
            let file_path = file_path.map_err(|_| Error::ListThoughts)?.path();
            thoughts.push(Thought::from_file_path(file_path)?);
        }

        Ok(thoughts)
    }
}
