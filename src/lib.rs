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
    let base_dirs = xdg::BaseDirectories::with_prefix("thought")?;
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
            let thought_id: ThoughtId = matches
                .get_one::<String>("id")
                .expect("id is required")
                .parse()?;
            let thought = thought_collection.find(&thought_id)?;
            thought.edit(EditorWrapper)?;
        }
        Some(("delete", matches)) => {
            let thought_ids: Vec<&String> = matches
                .get_many::<String>("ids")
                .expect("ids is required")
                .collect();

            for thought_id in thought_ids {
                let thought = match thought_collection.find(&thought_id.parse()?) {
                    Ok(thought) => thought,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                thought.delete()?;
            }
        }
        Some(("search", matches)) => {
            let pattern = matches
                .get_one::<String>("pattern")
                .expect("pattern is required");

            let mut matching_thoughts = vec![];

            for thought in thought_collection.thoughts() {
                let thought_content = thought.get_content()?;
                if thought_content.to_lowercase().contains(pattern) {
                    matching_thoughts.push(thought.clone());
                }
            }

            for thought in matching_thoughts {
                println!("{}", thought);
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
