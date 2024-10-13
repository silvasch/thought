mod cli;
pub(crate) use cli::get_cli;

mod editor_wrapper;
use std::path::Path;

pub(crate) use editor_wrapper::EditorWrapper;

mod error;
pub(crate) use error::Error;

mod thought;
pub(crate) use thought::Thought;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("thought").unwrap();
    let thoughts_dir = base_dirs.get_data_home().join("thoughts");

    match get_cli().get_matches().subcommand() {
        Some(("new", _)) => Thought::new(thoughts_dir)?.edit(EditorWrapper)?,
        Some(("list", _)) => {
            let thoughts = get_thoughts(thoughts_dir)?;
            for thought in thoughts {
                println!("{}", thought);
            }
        }
        Some(("edit", matches)) => {
            let thought_id: ThoughtId = matches.get_one::<String>("id").unwrap().parse().unwrap();

            let thoughts = get_thoughts(thoughts_dir)?;

            let mut found_thought = false;
            for thought in thoughts {
                if thought.id() == &thought_id {
                    found_thought = true;
                    thought.edit(EditorWrapper)?;
                    break;
                }
            }

            if !found_thought {
                todo!()
            }
        }
        Some(("delete", matches)) => {
            let thought_ids: Vec<ThoughtId> = matches
                .get_many::<String>("ids")
                .unwrap()
                .map(|v| v.parse().unwrap())
                .collect();

            let thoughts = get_thoughts(thoughts_dir)?;

            for thought_id in thought_ids {
                let mut found_thought = false;
                for thought in &thoughts {
                    if thought.id() == &thought_id {
                        found_thought = true;
                        thought.delete()?;
                        break;
                    }
                }

                if !found_thought {
                    todo!()
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn get_thoughts<P: AsRef<Path>>(thoughts_dir: P) -> Result<Vec<Thought>, Error> {
    let mut thoughts = vec![];

    for file_path in std::fs::read_dir(thoughts_dir).unwrap() {
        let file_path = file_path.unwrap().path();
        thoughts.push(Thought::from_file_path(file_path).unwrap());
    }

    Ok(thoughts)
}
