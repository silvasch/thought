use clap::{Arg, ArgAction, Command};

pub fn get_cli() -> Command {
    Command::new("thought")
        .author("Silvan Schmidt <mail@silvanschmidt.ch>")
        .about("Save your thoughts for later.")
        .subcommand(Command::new("new").about("Create a new thought."))
        .subcommand(Command::new("list").about("List your thoughts."))
        .subcommand(
            Command::new("edit")
                .arg(Arg::new("id").required(true).help("The id of the thought."))
                .about("Edit a thought."),
        )
        .subcommand(
            Command::new("delete")
                .arg(
                    Arg::new("ids")
                        .action(ArgAction::Append)
                        .required(true)
                        .help("The ids of the thoughts"),
                )
                .about("Remove thoughts"),
        )
}
