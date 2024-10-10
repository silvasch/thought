mod cli;
pub(crate) use cli::get_cli;

mod editor_wrapper;
pub(crate) use editor_wrapper::EditorWrapper;

mod error;
pub(crate) use error::Error;

mod thought_id;
pub(crate) use thought_id::ThoughtId;

mod words;
pub(crate) use words::WORDS;

pub fn run() -> Result<(), Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("thought").map_err(|_| todo!())?;

    let matches = get_cli().get_matches();

    match matches.subcommand() {
        Some(("new", _)) | None => {
            let thought_id = ThoughtId::new();
            let thought_path = base_dirs
                .place_data_file(format!("{}.md", thought_id))
                .map_err(|_| todo!())?;
            EditorWrapper::new().edit(thought_path)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
