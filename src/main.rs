use clap::{Parser, Subcommand};
use dialoguer::Select;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let items = vec!["Option 1", "Option 2"];
    let chosen = Select::new()
        .with_prompt("Which option do you prefer?")
        .items(&items)
        .interact()?;

    for _ in 0..args.count {
        println!("Hello {}. {}!", args.name, items[chosen]);
    }
    Ok(())
}
