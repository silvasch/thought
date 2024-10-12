mod cli;
pub(crate) use cli::get_cli;

mod editor_wrapper;
pub(crate) use editor_wrapper::EditorWrapper;

mod error;
pub(crate) use error::Error;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod thought_manager;
pub(crate) use thought_manager::ThoughtManager;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    let thought_manager = ThoughtManager::new()?;

    let matches = get_cli().get_matches();

    match matches.subcommand() {
        Some(("new", _)) | None => new_thought(thought_manager),
        Some(("list", _)) => list_thoughts(thought_manager),
        Some(("edit", matches)) => {
            let user_thought_id = matches
                .get_one::<String>("id")
                .expect("argument is required");

            edit_thought(thought_manager, user_thought_id)
        }
        Some(("remove", matches)) => {
            let user_thought_id = matches
                .get_one::<String>("id")
                .expect("argument is required");

            remove_thought(thought_manager, user_thought_id)
        }
        _ => unreachable!(),
    }
}

fn new_thought(thought_manager: ThoughtManager) -> Result<(), Error> {
    let thought_path = thought_manager.create_thought()?;
    EditorWrapper::new().edit(thought_path)?;
    Ok(())
}

fn list_thoughts(thought_manager: ThoughtManager) -> Result<(), Error> {
    for thought_id in thought_manager.get_thought_ids()? {
        println!(
            "{} ({})",
            thought_id.get_user_thought_id(),
            thought_id.date_time.format("%Y-%m-%d"),
        );
    }

    Ok(())
}

fn edit_thought(thought_manager: ThoughtManager, user_thought_id: &str) -> Result<(), Error> {
    match thought_manager.find_thought(user_thought_id)? {
        Some(thought_id) => {
            let thought_path = thought_manager.get_thought_path(&thought_id);
            EditorWrapper::new().edit(thought_path)?
        }
        None => eprintln!("That thought does not exist."),
    }

    Ok(())
}

fn remove_thought(thought_manager: ThoughtManager, user_thought_id: &str) -> Result<(), Error> {
    match thought_manager.find_thought(user_thought_id)? {
        Some(thought_id) => {
            let thought_path = thought_manager.get_thought_path(&thought_id);
            std::fs::remove_file(thought_path).map_err(|_| todo!())?;
        }
        None => eprintln!("That thought does not exist."),
    }

    Ok(())
}
