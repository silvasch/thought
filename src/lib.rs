mod cli;
pub(crate) use cli::get_cli;

mod editor_wrapper;
pub(crate) use editor_wrapper::EditorWrapper;

mod error;
pub(crate) use error::Error;

mod thought;
pub(crate) use thought::Thought;

mod thought_collection;
pub(crate) use thought_collection::ThoughtCollection;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("thought").unwrap();
    let thoughts_dir = base_dirs.get_data_home().join("thoughts");
    let thought_collection = ThoughtCollection::new(&thoughts_dir)?;

    match get_cli().get_matches().subcommand() {
        Some(("new", _)) => Thought::new(&thoughts_dir)?.edit(EditorWrapper)?,
        Some(("list", _)) => {
            for thought in thought_collection.thoughts() {
                println!("{}", thought);
            }
        }
        Some(("edit", matches)) => {
            let thought_id: ThoughtId = matches.get_one::<String>("id").unwrap().parse().unwrap();
            let thought = thought_collection.find(&thought_id)?;
            thought.edit(EditorWrapper)?;
        }
        Some(("delete", matches)) => {
            let thought_ids: Vec<ThoughtId> = matches
                .get_many::<String>("ids")
                .unwrap()
                .map(|v| v.parse().unwrap())
                .collect();

            for thought_id in thought_ids {
                let thought = match thought_collection.find(&thought_id) {
                    Ok(thought) => thought,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                thought.delete()?;
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
