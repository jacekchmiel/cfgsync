use clap::Parser;

#[derive(Parser, Debug)]
#[clap(APP ATTRIBUTE)]
struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    command: Command,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

/// Command to execute
#[derive(Subcommand)]
#[clap(PARENT APP ATTRIBUTE)]
enum Command {
    Add(AddCommand),
}

struct AddCommand {
    filename: String,
}
