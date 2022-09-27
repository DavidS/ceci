use askama::Template;
use clap::{Parser, Subcommand};
use dialoguer::Select;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[clap(short, long, value_parser)]
    // name: String,

    // /// Number of times to greet
    // #[clap(short, long, value_parser, default_value_t = 1)]
    // count: u8,
    // #[clap(subcommand)]
    // command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Pipeline {
        // /// lists test values
        // #[clap(short, long, action)]
        // list: bool,
    },
}

#[derive(Debug)]
enum Components {
    Cargo,
}

#[derive(Debug)]
enum Target {
    CircleCI,
}

#[derive(Template)]
#[template(path = "circleci/default.yml")]
struct CircleCI {}
fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // let items = vec!["Option 1", "Option 2"];
    // let chosen = Select::new()
    //     .with_prompt("Which option do you prefer?")
    //     .items(&items)
    //     .interact()?;

    // for _ in 0..args.count {
    //     println!("Hello {}. {}!", args.name, items[chosen]);
    // }

    // match &args.command {
    //     Some(Commands::Pipeline {}) => {
    //         println!("'myapp test' was used, list is: {:?}", list)
    //     }
    //     None => todo!(),
    // }
    println!("Initialising pipeline");

    // let mut handlebars = Handlebars::new();
    // handlebars
    //     .register_template_file("template", "./templates/circleci/default.hbs")
    //     .unwrap();

    // discover components

    let mut components: Vec<Components> = vec![];

    let path = Path::new("Cargo.toml");
    if path.exists() {
        components.push(Components::Cargo);
    }

    // for each component:
    //   add config overrides

    // render pipeline config
    let target = Target::CircleCI;

    match target {
        Target::CircleCI => {
            let config_dir = Path::new(".circleci");
            create_dir_all(config_dir).expect("create config_dir");
            let config_file = Path::new(".circleci/config.yml");
            let mut fd = File::create(config_file)?;
            // let mut fmt = adapter::FmtIoWriter::new(fd);
            for component in components {
                // fd.write_fmt(format_args!("config for {:#?}", component))?;
                // handlebars.render_to_write("template", &(), &mut fd).unwrap();
                let cc = CircleCI {};
                // cc.render_into(&mut fmt).expect("could render");
                write!(fd, "{cc}")?;
            }
        }
    }

    Ok(())
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
