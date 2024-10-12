use clap::{Arg, Command};

pub fn get_cli() -> Command {
    Command::new("thought")
        .author("Silvan Schmidt <mail@silvanschmidt.ch>")
        .about("Save your thoughts for later.")
        .subcommand(Command::new("new").about("Create a new thought."))
        .subcommand(Command::new("list").about("List your thoughts."))
        .subcommand(
            Command::new("edit")
                .about("Edit a thought.")
                .arg(Arg::new("id").required(true).help("The id of the thought.")),
        )
}
