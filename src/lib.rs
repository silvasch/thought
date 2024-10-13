mod cli;
pub(crate) use cli::get_cli;

mod editor_wrapper;
pub(crate) use editor_wrapper::EditorWrapper;

mod error;
pub(crate) use error::Error;

mod state;
pub(crate) use state::State;

mod thought;
pub(crate) use thought::Thought;

mod thought_collection;
pub(crate) use thought_collection::ThoughtCollection;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod truncate;
pub(crate) use truncate::Truncate;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("thought")?;

    let thoughts_dir_path = base_dirs.get_data_home().join("thoughts");
    let thought_collection = ThoughtCollection::new(&thoughts_dir_path)?;

    let state_file_path = base_dirs.place_data_file("state.toml").unwrap();
    let raw_state = std::fs::read_to_string(&state_file_path).unwrap_or("".to_string());
    let mut state: State = toml::from_str(&raw_state).unwrap();

    match get_cli().get_matches().subcommand() {
        Some(("new", _)) => {
            let thought = Thought::new(&thoughts_dir_path)?;
            thought.edit(EditorWrapper)?;
            state.last_accessed_thought_id = Some(thought.id().to_string());
        }
        Some(("list", _)) => {
            for thought in thought_collection.thoughts() {
                println!("{}", thought);
            }
        }
        Some(("edit", matches)) => {
            let raw_thought_id = if matches.get_flag("last") {
                state.last_accessed_thought_id.unwrap()
            } else {
                matches
                    .get_one::<String>("id")
                    .expect("id is required")
                    .to_string()
            };
            let thought_id = raw_thought_id.parse()?;

            let thought = thought_collection.find(&thought_id)?;
            thought.edit(EditorWrapper)?;

            state.last_accessed_thought_id = Some(thought.id().to_string());
        }
        Some(("delete", matches)) => {
            let raw_thought_ids = if matches.get_flag("last") {
                vec![state.last_accessed_thought_id.clone().unwrap()]
            } else {
                matches
                    .get_many::<String>("ids")
                    .expect("id is required")
                    .map(ToString::to_string)
                    .collect()
            };

            for raw_thought_id in raw_thought_ids {
                let thought_id: ThoughtId = match raw_thought_id.parse() {
                    Ok(thought_id) => thought_id,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                let thought = thought_collection.find(&thought_id)?;
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

    std::fs::write(&state_file_path, toml::to_string(&state).unwrap()).unwrap();

    Ok(())
}
