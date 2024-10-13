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

        todo!()
    }

    pub fn thoughts(&self) -> &Vec<Thought> {
        &self.thoughts
    }

    fn get_thoughts<P: AsRef<Path>>(thoughts_dir: P) -> Result<Vec<Thought>, Error> {
        let mut thoughts = vec![];

        for file_path in std::fs::read_dir(thoughts_dir).unwrap() {
            let file_path = file_path.unwrap().path();
            thoughts.push(Thought::from_file_path(file_path).unwrap());
        }

        Ok(thoughts)
    }
}
