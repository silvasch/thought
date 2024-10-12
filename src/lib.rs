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
        Some(("new", _)) | None => {
            let thought_path = thought_manager.create_thought()?;
            EditorWrapper::new().edit(thought_path)?;
        }
        Some(("list", _)) => {
            for thought_id in thought_manager.get_thought_ids()? {
                println!(
                    "{} ({})",
                    thought_id.get_user_id(),
                    thought_id.date_time.format("%Y-%m-%d"),
                );
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
