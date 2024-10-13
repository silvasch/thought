use clap::{Arg, ArgAction, Command};

pub fn get_cli() -> Command {
    Command::new("thought")
        .author("Silvan Schmidt <mail@silvanschmidt.ch>")
        .about("Save your thoughts for later.")
        .subcommand(Command::new("new").about("Create a new thought."))
        .subcommand(Command::new("list").about("List your thoughts."))
        .subcommand(
            Command::new("edit")
                .arg(
                    Arg::new("id")
                        .required_unless_present("last")
                        .help("The id of the thought."),
                )
                .arg(
                    Arg::new("last")
                        .long("last")
                        .action(ArgAction::SetTrue)
                        .help("Edit the last accessed note."),
                )
                .about("Edit a thought."),
        )
        .subcommand(
            Command::new("delete")
                .arg(
                    Arg::new("ids")
                        .action(ArgAction::Append)
                        .required(true)
                        .help("The ids of the thoughts."),
                )
                .about("Remove thoughts"),
        )
        .subcommand(
            Command::new("search")
                .arg(
                    Arg::new("pattern")
                        .required(true)
                        .help("The pattern to search for."),
                )
                .about("Search your thoughts."),
        )
}
