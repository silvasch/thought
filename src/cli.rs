use clap::Command;

pub fn get_cli() -> Command {
    Command::new("thought")
        .author("Silvan Schmidt <mail@silvanschmidt.ch>")
        .about("Save your thoughts for later.")
        .subcommand(Command::new("new"))
        .subcommand(Command::new("list"))
}
